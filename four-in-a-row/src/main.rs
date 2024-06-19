#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    Red,
    Blue,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::Red => write!(f, "R"),
            Color::Blue => write!(f, "B"),
        }
    }
}

#[derive(Default, Clone)]
struct Cell {
    pub piece: Option<Color>,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.piece {
            Some(color) => write!(f, "{}", color),
            None => write!(f, "."),
        }
    }
}

struct Board {
    pub grid: Vec<Vec<Cell>>,
}

enum PlayResult {
    ColumnFull,
    OutOfBounds,
    Ok,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Cell::default(); height]; width];
        Self { grid }
    }

    fn play(&mut self, y: usize, color: Color) -> PlayResult {
        let col = match self.grid.get_mut(y) {
            Some(col) => col,
            None => return PlayResult::OutOfBounds,
        };
        let first = match col.iter_mut().find(|cell| cell.piece.is_none()) {
            Some(cell) => cell,
            None => return PlayResult::ColumnFull,
        };
        first.piece = Some(color);
        return PlayResult::Ok;
    }

    fn display(&self) {
        for row in (0..self.grid[0].len()).rev() {
            for col in 0..self.grid.len() {
                print!("{} ", self.grid[col][row]);
            }
            println!();
        }
    }

    fn check_four_in_a_row(&self) -> Option<Color> {
        let directions = [
            (1, 0),   // horizontal
            (0, 1),   // vertical
            (1, 1),   // diagonal (bottom-left to top-right)
            (1, -1),  // diagonal (top-left to bottom-right)
        ];

        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                if let Some(color) = &self.grid[x][y].piece {
                    for &(dx, dy) in &directions {
                        if self.check_direction(x, y, dx, dy, color) {
                            return Some(color.clone());
                        }
                    }
                }
            }
        }
        None
    }

    fn check_direction(&self, x: usize, y: usize, dx: isize, dy: isize, color: &Color) -> bool {
        for i in 0..4 {
            let nx = x as isize + i * dx;
            let ny = y as isize + i * dy;
            if nx < 0 || ny < 0 || nx >= self.grid.len() as isize || ny >= self.grid[0].len() as isize {
                return false;
            }
            if self.grid[nx as usize][ny as usize].piece.as_ref() != Some(color) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut board = Board::new(7, 6);

    board.play(0, Color::Red);
    board.play(1, Color::Blue);
    board.play(0, Color::Blue);
    board.play(2, Color::Red);
    board.play(1, Color::Red);
    board.play(2, Color::Blue);
    board.play(3, Color::Red);
    board.play(4, Color::Blue);
    board.play(4, Color::Red);
    board.play(5, Color::Blue);
    board.play(5, Color::Blue);
    board.play(5, Color::Red);
    board.play(6, Color::Blue);
    board.play(6, Color::Blue);
    board.play(6, Color::Blue);
    board.play(6, Color::Red);

    board.display();

    if let Some(winner) = board.check_four_in_a_row() {
        println!("Winner: {}", winner);
    } else {
        println!("No winner yet.");
    }
}
