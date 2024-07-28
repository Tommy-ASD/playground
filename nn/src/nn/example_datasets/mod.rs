use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod first;

#[derive(Debug, Serialize, Deserialize)]
pub struct Fruit {
    pub spike_length: f64,
    pub spot_size: f64,
    pub poison: bool,
}

impl Fruit {
    pub fn generate_random() -> Fruit {
        let mut rng = rand::thread_rng();
        let n1 = rng.gen_range(-1f64..1f64);
        let n2 = rng.gen_range(-1f64..1f64);
        let poison = rng.gen_bool(0.5);
        Fruit {
            spike_length: n1,
            spot_size: n2,
            poison,
        }
    }
}
