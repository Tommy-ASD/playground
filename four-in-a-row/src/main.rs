#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
enum Color {
    Red,
    Blue,
}

impl std::ops::Not for Color {
    type Output = Color;
    
    fn not(self) -> Self::Output {
        match self {
            Color::Red => Color::Blue,
            Color::Blue => Color::Red,
        }
    }
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
        let (x, first) = match col.iter_mut().enumerate().find(|(index, cell)| cell.piece.is_none()) {
            Some(cell) => cell,
            None => return PlayResult::ColumnFull,
        };
        first.piece = Some(color);
        if let Some(winner) = self.check_four_in_a_row((y, x)) {
            println!("Winner: {}", winner);
        } else {
            println!("No winner yet after {y} {x} turned to {color}");
        }
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

    fn check_four_in_a_row(&self, index: (usize, usize)) -> Option<Color> {
        let directions = [
            (1, 0),   // horizontal
            (0, 1),   // vertical
            (-1, 0),
            (0, -1),
            (1, 1),   // diagonal (bottom-left to top-right)
            (1, -1),  // diagonal (top-left to bottom-right)
            (-1, 1),
            (-1, -1), 
        ];

        if let Some(color) = &self.grid[index.0][index.1].piece {
            for &direction in &directions {
                if self.check_direction(index, direction, color) {
                    return Some(color.clone());
                }
            }
        }
        None
    }

    fn check_direction(&self, index: (usize, usize), direction: (isize, isize), color: &Color) -> bool {
        for i in 0..4 {
            let ny = index.0 as isize + i * direction.0;
            let nx = index.1 as isize + i * direction.1;
            if ny < 0 || nx < 0 || ny >= self.grid.len() as isize || nx >= self.grid[0].len() as isize {
                return false;
            }
            if self.grid[ny as usize][nx as usize].piece.as_ref() != Some(color) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut board = Board::new(7, 6);


    board.display();
}
