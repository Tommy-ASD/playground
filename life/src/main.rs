use std::collections::HashSet;

use rand::Rng;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

struct Life {
    alive_cells: HashSet<Coordinate>,
}

impl Default for Life {
    fn default() -> Self {
        Life {
            alive_cells: HashSet::new(),
        }
    }
}

impl Life {
    // if an index is here, it should be toggled
    // in other words, it should be removed from alive_cells if it was there
t    fn get_changed_state(&self) -> HashSet<t> {
        let (mut xs, mut ys): (Vec<i32>, Vec<i32>) = self
            .alive_cells
            .iter()
            .map(|coord| (coord.x, coord.y))
            .unzip();
        xs.sort();
        ys.sort();
        let min_x = xs.first();
        let min_y = ys.first();
        let max_x = xs.last();
        let max_y = ys.last();
        println!("Min x: {min_x:?}\nMin y: {min_y:?}");
        println!("Max x: {max_x:?}\nMax y: {max_y:?}");
        println!("x: {xs:?}");
        println!("y: {ys:?}");
        // iterate through self.alive_cells
        // create a temporary, virtual vector
        // starting at the very edge of alive_cells
        HashSet::new()
    }

    fn next(&mut self) {}

    fn generate_random_life(width: i32, height: i32, num_cells: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut alive_cells = HashSet::new();

        for _ in 0..num_cells {
            let random_x = rng.gen_range(-width / 2..width / 2);
            let random_y = rng.gen_range(-height / 2..height / 2);
            let mut coordinate = Coordinate {
                x: random_x,
                y: random_y,
            };
            while alive_cells.contains(&coordinate) {
                let random_x = rng.gen_range(-width / 2..width / 2);
                let random_y = rng.gen_range(-height / 2..height / 2);
                coordinate = Coordinate {
                    x: random_x,
                    y: random_y,
                };
            }
            alive_cells.insert(coordinate);
        }

        Self { alive_cells }
    }
}

fn main() {
    let life = Life::generate_random_life(100, 100, 100);
    println!("Life: {:?}", life.alive_cells);
    let mut sorted = life.alive_cells.iter().collect::<Vec<&Coordinate>>();
    sorted.sort();
    println!("Sorted: {:?}", sorted);
    println!("{:?}", life.get_changed_state());
}
