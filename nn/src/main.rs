use ndarray::{Array2, Axis};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
use std::error::Error;

struct NeuralNetwork {
    input_size: usize,
    output_size: usize,
    weights: Array2<f64>,
    bias: Array2<f64>,
}

impl NeuralNetwork {
    fn new(input_size: usize, output_size: usize) -> NeuralNetwork {
        let weights = Array2::random((input_size, output_size), Uniform::new(0.0, 1.0));
        let bias = Array2::random((1, output_size), Uniform::new(0.0, 1.0));

        NeuralNetwork {
            input_size,
            output_size,
            weights,
            bias,
        }
    }

    fn forward(&self, input: &Array2<f64>) -> Array2<f64> {
        let weighted_sum = input.dot(&self.weights) + &self.bias;
        weighted_sum
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_size = 2;
    let output_size = 1;

    let nn = NeuralNetwork::new(input_size, output_size);

    // Example input
    let input = Array2::from_shape_vec((1, 2), vec![0.1, 0.2]).unwrap();

    // Forward pass
    let output = nn.forward(&input);

    println!("Input: {:?}", input);
    println!("Output: {:?}", output);

    Ok(())
}
