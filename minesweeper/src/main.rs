use rand::Rng;
use utils::input;

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PlayType {
    #[default]
    Press,
    Flag
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PlayResult {
    OutOfBounds,
    CellAlreadyPlayed,
    CellFlagged,
    Unflagged,
    Flagged,
    PlayedClear(u8),
    PlayedMine
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CellState {
    #[default]
    Unplayed,
    Pressed,
    Flagged, 
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CellType {
    #[default]
    Clear,
    Mine
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
            for index in self.get_neighbors((y, x)) {
                println!("{y}-{x} neighbor: {index:?}");
                self.grid[index.0][index.1].mine_neighbors += 1;
            }
        }
    }
    fn get_cell_at(&self, index: (usize, usize)) -> Option<&Cell> {
        self.grid.get(index.0).and_then(|row| row.get(index.1))
    }
    fn get_cell_at_mut(&mut self, index: (usize, usize)) -> Option<&mut Cell> {
        self.grid.get_mut(index.0).and_then(|row| row.get_mut(index.1))
    }
    fn get_neighbors(&self, index: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        // for 1, 0, -1
        for i in -1..=1 {
            for j in -1..=1 {
                // skip i == 0 && j == 0 because that's the cell itself
                if i == 0 && j == 0 {
                    continue;
                }
                let y = index.0 as i8 + j;
                let x = index.1 as i8 + i;
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
                match cell.state {
                    CellState::Unplayed => print!("◼ "),
                    CellState::Pressed => print!("{} ", number_to_emoji(cell.mine_neighbors)),
                    CellState::Flagged => print!("🚩 "),
                }
            }
            println!("");
        }
    }
    fn clear_zeroes(&mut self, index: (usize, usize)) {
        let cell = self.get_cell_at_mut(index).unwrap();
        let cell_state_clone = cell.state.clone();
        cell.state = CellState::Pressed;
        
        if cell.mine_neighbors == 0 && cell_state_clone == CellState::Unplayed {
            self.get_neighbors(index).iter().for_each(|neighbor| {
                self.clear_zeroes(*neighbor);
            })
        }
    }
    fn play(&mut self, index: (usize, usize), playtype: PlayType) -> PlayResult {
        let cell = match self.grid.get_mut(index.0).and_then(|row| row.get_mut(index.1)) {
            Some(cell) => cell,
            None => return PlayResult::OutOfBounds,
        };
    
        match cell.state {
            CellState::Pressed => PlayResult::CellAlreadyPlayed,
            CellState::Flagged => match playtype {
                PlayType::Press => PlayResult::CellFlagged,
                PlayType::Flag => {
                    cell.state = CellState::Unplayed;
                    PlayResult::Unflagged
                }
            },
            CellState::Unplayed => match playtype {
                PlayType::Press => {
                    match cell.ctype {
                        CellType::Clear => {
                            let mine_neighbors = cell.mine_neighbors;
                            self.clear_zeroes(index);
                            PlayResult::PlayedClear(mine_neighbors)
                        },
                        CellType::Mine => PlayResult::PlayedMine
                    }
                }
                PlayType::Flag => {
                    cell.state = CellState::Flagged;
                    PlayResult::Flagged
                }
            }
        }
    }
}

fn main() {
    let mut board = Board::new(10, 10);
    board.generate_mines(10);
    loop {
        board.display();
        let input = input!();
        let mut split = input.split(" ");
        let y = split.next().unwrap().parse::<usize>().unwrap();
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let flag = split.next().and_then(|f| Some(if f.starts_with("f") {
            PlayType::Flag
        } else {
            PlayType::Press
        })).unwrap_or(PlayType::Press);
        match board.play((y, x), flag) {
            PlayResult::CellAlreadyPlayed => {},
            PlayResult::CellFlagged => {},
            PlayResult::OutOfBounds => {},
            PlayResult::Unflagged => {},
            PlayResult::Flagged => {},
            PlayResult::PlayedMine => {},
            PlayResult::PlayedClear(num) => {}
        }
    }
}

fn number_to_emoji<'a>(num: u8) -> &'a str {
    match num {
        0 => "0️⃣",
        1 => "1️⃣",
        2 => "2️⃣",
        3 => "3️⃣",
        4 => "4️⃣",
        5 => "5️⃣",
        6 => "6️⃣",
        7 => "7️⃣",
        8 => "8️⃣",
        9 => "9️⃣",
        _ => panic!("{}", num)
    }
}
