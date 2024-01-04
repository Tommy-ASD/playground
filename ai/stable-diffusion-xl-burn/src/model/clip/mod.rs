pub mod load;

use crate::model::layernorm::{LayerNorm, LayerNormConfig};

use burn::{
    config::Config,
    module::{Module, Param},
    nn,
    tensor::{
        activation::{sigmoid, softmax},
        backend::Backend,
        module::embedding,
        Distribution, Int, Tensor,
    },
};

use crate::backend::Backend as MyBackend;

#[derive(Config, Debug)]
pub struct CLIPConfig {
    n_vocab: usize,
    n_state: usize,
    embed_dim: usize,
    n_head: usize,
    n_ctx: usize,
    n_layer: usize,
    quick_gelu: bool,
}

impl CLIPConfig {
    pub fn init<B: Backend>(&self) -> CLIP<B> {
        let token_embedding = nn::EmbeddingConfig::new(self.n_vocab, self.n_state).init();
        let position_embedding =
            Tensor::random([self.n_ctx, self.n_state], Distribution::Normal(0.0, 1.0)).into();
        let blocks = (0..self.n_layer)
            .into_iter()
            .map(|_| {
                ResidualDecoderAttentionBlockConfig::new(self.n_state, self.n_head, self.quick_gelu)
                    .init()
            })
            .collect();
        let layer_norm = LayerNormConfig::new(self.n_state).init();
        let text_projection = Some(
            Tensor::random(
                [self.n_state, self.embed_dim],
                Distribution::Normal(0.0, 1.0),
            )
            .into(),
        );

        CLIP {
            token_embedding,
            position_embedding,
            blocks,
            layer_norm,
            text_projection,
        }
    }
}

#[derive(Module, Debug)]
pub struct CLIP<B: Backend> {
    token_embedding: nn::Embedding<B>,
    position_embedding: Param<Tensor<B, 2>>,
    blocks: Vec<ResidualDecoderAttentionBlock<B>>,
    layer_norm: LayerNorm<B>,
    text_projection: Option<Param<Tensor<B, 2>>>,
}

impl<B: MyBackend> CLIP<B> {
    /*pub fn forward(&self, x: Tensor<B, 2, Int>) -> Tensor<B, 3> {
        let [n_batch, seq_len] = x.dims();

        let mask = attn_decoder_mask(seq_len, &x.device());

        let embedded = self.token_embedding.forward(x)
            + self.position_embedding.val().slice([0..seq_len]).unsqueeze();

        let mut x = embedded;
        for block in &self.blocks {
            x = block.forward(x, mask.clone());
        }

        let o = self.layer_norm.forward(x);

        if let Some(t_proj) = self.t_proj.as_ref() {
            o.mat_mul(t_proj.val())
        } else {
            o
        }
    }*/

    pub fn forward_hidden(&self, x: Tensor<B, 2, Int>, hidden_idx: usize) -> Tensor<B, 3> {
        let [n_batch, seq_len] = x.dims();

        let mask = Tensor::from_primitive(B::attn_decoder_mask(seq_len, &x.device()));

        let embedded = self.token_embedding.forward(x)
            + self
                .position_embedding
                .val()
                .slice([0..seq_len])
                .unsqueeze();

        let mut x = embedded;
        for block in &self.blocks[0..hidden_idx] {
            x = block.forward(x, mask.clone());
        }

        x
    }

    pub fn forward_hidden_pooled(
        &self,
        text: Tensor<B, 2, Int>,
        hidden_idx: usize,
    ) -> (Tensor<B, 3>, Tensor<B, 2>) {
        let [n_batch, seq_len] = text.dims();

        let mask = Tensor::from_primitive(B::attn_decoder_mask(seq_len, &text.device()));

        let embedded = self.token_embedding.forward(text.clone())
            + self
                .position_embedding
                .val()
                .slice([0..seq_len])
                .unsqueeze();

        let mut x = embedded;
        let mut h_out = Tensor::empty(x.shape());
        for (i, block) in self.blocks.iter().enumerate() {
            if i == hidden_idx {
                h_out = x.clone();
            }

            x = block.forward(x, mask.clone());
        }

        // get features from the eot embedding (eot_token is the highest number in each sequence)
        let eot_indices = text.argmax(1).squeeze(1);
        let normed = self.layer_norm.forward(x);
        let o = normed.select(1, eot_indices).squeeze(1);
        let pooled = if let Some(t_proj) = self.text_projection.as_ref() {
            o.matmul(t_proj.val())
        } else {
            o
        };

        (h_out, pooled)
    }

    pub fn max_sequence_length(&self) -> usize {
        self.position_embedding.dims()[0]
    }

    pub fn num_layers(&self) -> usize {
        self.blocks.len()
    }
}

#[derive(Config)]
pub struct ResidualDecoderAttentionBlockConfig {
    n_state: usize,
    n_head: usize,
    quick_gelu: bool,
}

impl ResidualDecoderAttentionBlockConfig {
    pub fn init<B: Backend>(&self) -> ResidualDecoderAttentionBlock<B> {
        let attn = MultiHeadSelfAttentionConfig::new(self.n_state, self.n_head).init();
        let attn_ln = LayerNormConfig::new(self.n_state).init();

        let mlp = MLPConfig::new(self.n_state, 4 * self.n_state, self.quick_gelu).init();
        let mlp_ln = LayerNormConfig::new(self.n_state).init();

        ResidualDecoderAttentionBlock {
            attn,
            attn_ln,
            mlp,
            mlp_ln,
        }
    }
}

#[derive(Module, Debug)]
pub struct ResidualDecoderAttentionBlock<B: Backend> {
    attn: MultiHeadSelfAttention<B>,
    attn_ln: LayerNorm<B>,
    mlp: MLP<B>,
    mlp_ln: LayerNorm<B>,
}

impl<B: MyBackend> ResidualDecoderAttentionBlock<B> {
    fn forward(&self, x: Tensor<B, 3>, mask: Tensor<B, 2>) -> Tensor<B, 3> {
        let x = x.clone() + self.attn.forward(self.attn_ln.forward(x), Some(mask));
        let x = x.clone() + self.mlp.forward(self.mlp_ln.forward(x));
        return x;
    }
}

#[derive(Config)]
pub struct MultiHeadSelfAttentionConfig {
    n_state: usize,
    n_head: usize,
}

impl MultiHeadSelfAttentionConfig {
    fn init<B: Backend>(&self) -> MultiHeadSelfAttention<B> {
        assert!(
            self.n_state % self.n_head == 0,
            "State size {} must be a multiple of head size {}",
            self.n_state,
            self.n_head
        );

        let n_head = self.n_head;
        let query = nn::LinearConfig::new(self.n_state, self.n_state).init();
        let key = nn::LinearConfig::new(self.n_state, self.n_state).init();
        let value = nn::LinearConfig::new(self.n_state, self.n_state).init();
        let out = nn::LinearConfig::new(self.n_state, self.n_state).init();

        MultiHeadSelfAttention {
            n_head,
            query,
            key,
            value,
            out,
        }
    }
}

#[derive(Module, Debug)]
pub struct MultiHeadSelfAttention<B: Backend> {
    n_head: usize,
    query: nn::Linear<B>,
    key: nn::Linear<B>,
    value: nn::Linear<B>,
    out: nn::Linear<B>,
}

impl<B: MyBackend> MultiHeadSelfAttention<B> {
    pub fn forward(&self, x: Tensor<B, 3>, mask: Option<Tensor<B, 2>>) -> Tensor<B, 3> {
        let q = self.query.forward(x.clone());
        let k = self.key.forward(x.clone());
        let v = self.value.forward(x);

        let wv = Tensor::from_primitive(B::qkv_attention(
            q.into_primitive(),
            k.into_primitive(),
            v.into_primitive(),
            mask.map(|m| m.into_primitive()),
            self.n_head,
        ));

        return self.out.forward(wv);
    }
}

#[derive(Config, Debug)]
pub struct MLPConfig {
    input_size: usize,
    hidden_size: usize,
    quick_gelu: bool,
}

impl MLPConfig {
    fn init<B: Backend>(&self) -> MLP<B> {
        let fc1 = nn::LinearConfig::new(self.input_size, self.hidden_size).init();
        let qgelu = QuickGELU::new();
        let gelu = nn::GELU::new();
        let fc2 = nn::LinearConfig::new(self.hidden_size, self.input_size).init();

        let quick_gelu = self.quick_gelu;

        MLP {
            quick_gelu,
            fc1,
            qgelu,
            gelu,
            fc2,
        }
    }
}

#[derive(Module, Debug)]
pub struct MLP<B: Backend> {
    quick_gelu: bool,
    fc1: nn::Linear<B>,
    qgelu: QuickGELU,
    gelu: nn::GELU,
    fc2: nn::Linear<B>,
}

impl<B: Backend> MLP<B> {
    fn forward<const D: usize>(&self, x: Tensor<B, D>) -> Tensor<B, D> {
        let x = self.fc1.forward(x);
        let x = if self.quick_gelu {
            self.qgelu.forward(x)
        } else {
            self.gelu.forward(x)
        };
        let x = self.fc2.forward(x);

        x
    }
}

#[derive(Module, Clone, Debug)]
pub struct QuickGELU {}

impl QuickGELU {
    fn new() -> Self {
        Self {}
    }

    fn forward<B: Backend, const D: usize>(&self, x: Tensor<B, D>) -> Tensor<B, D> {
        x.clone() * sigmoid(x * 1.702)
    }
}
