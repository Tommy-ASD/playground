struct Coordinate {
    x: i32,
    y: i32,
}

struct Life {
    alive_cells: Vec<Coordinate>,
}

impl Default for Life {
    fn default() -> Self {
        Life {
            alive_cells: vec![],
        }
    }
}

impl Life {
    // if an index is here, it should be toggled
    // in other words, it should be removed from alive_cells if it was there
    // and added if it wasn't
    fn get_changed_state(&self) -> Vec<Coordinate> {
        vec![]
    }

    fn next(&mut self) {}
}

fn main() {
    println!("Hello, world!");
}
