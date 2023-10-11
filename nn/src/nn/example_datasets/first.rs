use super::Fruit;

pub fn first_dataset() -> Vec<Fruit> {
    let mut ds = vec![];
    ds.push(Fruit {
        spike_length: 1.0,
        spot_size: 1.0,
        poison: true,
    });
    ds.push(Fruit {
        spike_length: 0.0,
        spot_size: 0.0,
        poison: true,
    });
    ds.push(Fruit {
        spike_length: -1.0,
        spot_size: -1.0,
        poison: false,
    });
    ds.push(Fruit {
        spike_length: -0.5,
        spot_size: -1.0,
        poison: false,
    });
    ds
}
