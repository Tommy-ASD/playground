use burn::tensor::ElementConversion;
use std::error::Error;

use burn::{
    config::Config,
    module::{Module, Param},
    nn,
    tensor::{backend::Backend, Tensor},
};

use super::*;
use crate::model::{
    autoencoder::load::load_autoencoder, clip::load::load_clip_text_transformer, load::*,
    unet::load::load_unet,
};

/*pub fn load_stable_diffusion<B: Backend>(path: &str, device: &B::Device) -> Result<StableDiffusion<B>, Box<dyn Error>> {
    let n_steps = load_usize::<B>("n_steps", path, device)?;
    let alpha_cumulative_products = load_tensor::<B, 1>("alphas_cumprod", path, device)?.into();
    let autoencoder = load_autoencoder(&format!("{}/{}", path, "autoencoder"), device)?;
    let diffusion = load_unet(&format!("{}/{}", path, "unet"), device)?;
    let clip = load_clip_text_transformer(&format!("{}/{}", path, "clip"), device, false)?;

    Ok(StableDiffusion {
        n_steps,
        alpha_cumulative_products,
        autoencoder,
        diffusion,
        clip,
    })
}*/

pub fn load_embedder<B: Backend>(
    path: &str,
    device: &B::Device,
) -> Result<Embedder<B>, Box<dyn Error>> {
    let clip = load_clip_text_transformer(&format!("{}/{}", path, "clip"), device, false)?;
    let open_clip = load_clip_text_transformer(&format!("{}/{}", path, "open_clip"), device, true)?;

    let clip_tokenizer = ClipTokenizer::new()?;
    let open_clip_tokenizer = OpenClipTokenizer::new()?;

    Ok(Embedder {
        clip,
        open_clip,
        clip_tokenizer,
        open_clip_tokenizer,
    })
}

pub fn load_diffuser<B: Backend>(
    path: &str,
    device: &B::Device,
    is_refiner: bool,
) -> Result<Diffuser<B>, Box<dyn Error>> {
    let n_steps = load_usize::<B>("n_steps", path, device)?;
    let alpha_cumulative_products = load_tensor::<B, 1>("alphas_cumprod", path, device)?.into();
    let name = if is_refiner {
        "diffuser_refiner"
    } else {
        "diffuser_base"
    };
    let diffusion = load_unet(&format!("{}/{}", path, name), device)?;

    Ok(Diffuser {
        n_steps,
        alpha_cumulative_products,
        diffusion,
        is_refiner,
    })
}

pub fn load_latent_decoder<B: Backend>(
    path: &str,
    device: &B::Device,
) -> Result<LatentDecoder<B>, Box<dyn Error>> {
    let autoencoder = load_autoencoder(&format!("{}/{}", path, "autoencoder"), device)?;
    let scale_factor = load_f32::<B>("scale_factor", path, device)?.into();

    Ok(LatentDecoder {
        autoencoder,
        scale_factor,
    })
}
