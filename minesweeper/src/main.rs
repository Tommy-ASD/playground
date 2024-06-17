use rand::Rng;


#[derive(Default, Clone)]
enum CellState {
    #[default]
    Unplayed,
    Pressed,
    Flagged, 
}

#[derive(Default, Clone)]
enum CellType {
    #[default]
    Clear,
    Mine
}

#[derive(Default, Clone)]
struct Cell {
    state: CellState,
    ctype: CellType,
    mine_neighbors: u8
}

impl Cell {
    fn set_mine(&mut self) {
        self.ctype = CellType::Mine;
    }
}

#[derive(Default)]
struct Board {
    grid: Vec<Vec<Cell>>,
    mines: Vec<(usize, usize)>
}

impl Board {
    fn new(height: usize, width: usize) -> Self {
        let grid = vec![vec![Cell::default(); width as usize]; height as usize];

        Self {
            grid,
            mines: Vec::new()
        }
    }
    fn get_width(&self) -> usize {
        self.grid.len()
    }
    fn get_height(&self) -> usize {
        self.grid[0].len()
    }
    fn generate_mines(&mut self, mine_amount: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..mine_amount {
            let y = rng.gen_range(0..self.get_height());
            let x = rng.gen_range(0..self.get_width());
            self.mines.push((x, y));
            self.grid[y as usize][x as usize].set_mine();
            for index in self.get_neighbors(y, x) {
                println!("{y}-{x} neighbor: {index:?}");
                self.grid[index.0][index.1].mine_neighbors += 1;
            }
        }
    }
    fn get_neighbors(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        // for 1, 0, -1
        for i in -1..=1 {
            for j in -1..=1 {
                // skip i == 0 && j == 0 because that's the cell itself
                if i == 0 && j == 0 {
                    continue;
                }
                let x = x as i8 + i;
                let y = y as i8 + j;
                // if cell is out of bounds, skip it
                if x < 0 || y < 0 || x >= self.get_width() as i8 || y >= self.get_height() as i8 {
                    continue;
                }
                neighbors.push((y as usize, x as usize));
            }
        }
        neighbors
    }
    fn display(&self) {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                let cell = &self.grid[y as usize][x as usize];
                match cell.ctype {
                    CellType::Mine => print!("* "),
                    CellType::Clear => print!("{} ", cell.mine_neighbors),
                }
            }
            println!("");
        }
    }
}

fn main() {
    let mut board = Board::new(10, 10);
    board.generate_mines(10);
    board.display();
}
