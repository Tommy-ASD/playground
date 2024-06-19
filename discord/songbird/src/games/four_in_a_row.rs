use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{Context, Error, FourInARowManager, UserData};

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Color {
    #[default]
    Blue,
    Red,
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
pub struct Cell {
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

#[derive(Default, Clone)]
pub struct Board {
    pub grid: Vec<Vec<Cell>>,
    pub player: Color
}

pub enum PlayResult {
    ColumnFull,
    OutOfBounds,
    Ok,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Cell::default(); height]; width];
        Self { grid, player: Color::Blue }
    }

    pub fn play(&mut self, y: usize) -> PlayResult {
        let col = match self.grid.get_mut(y) {
            Some(col) => col,
            None => return PlayResult::OutOfBounds,
        };
        let (x, first) = match col.iter_mut().enumerate().find(|(index, cell)| cell.piece.is_none()) {
            Some(cell) => cell,
            None => return PlayResult::ColumnFull,
        };
        first.piece = Some(self.player);
        if let Some(winner) = self.check_four_in_a_row((y, x)) {
            println!("Winner: {}", winner);
        } else {
            println!("No winner yet after {y} {x} turned to {color}", color = self.player);
        }
        self.player = !self.player;
        return PlayResult::Ok;
    }

    pub fn display(&self) {
        for row in (0..self.grid[0].len()).rev() {
            for col in 0..self.grid.len() {
                print!("{} ", self.grid[col][row]);
            }
            println!();
        }
    }

    pub fn to_emojis(&self) -> String {
        let mut display = String::new();
        for row in (0..self.grid[0].len()).rev() {
            for col in 0..self.grid.len() {
                match self.grid[col][row].piece {
                    Some(Color::Blue) => display.push_str("🔵"),
                    Some(Color::Red) => display.push_str("🔴"),
                    None => display.push_str("◼️"),
                }
            }
            display.push('\n')
        }
        display
    }

    pub fn check_four_in_a_row(&self, index: (usize, usize)) -> Option<Color> {
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

    pub fn check_direction(&self, index: (usize, usize), direction: (isize, isize), color: &Color) -> bool {
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

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn four_in_a_row(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let mut users_lock = ctx.data().users.lock().await;
    let udata = users_lock.entry(ctx.author().id).or_insert_with(|| Arc::new(Mutex::new(UserData::default())));
    let user_data_arc = Arc::clone(udata);
    let board = Board::new(10, 10);
    let display = board.to_emojis();
    user_data_arc.lock().await.four_in_a_row = Some(FourInARowManager {
        board,
        origin_channel_id: ctx.channel_id()
    });
    ctx.reply(display).await.unwrap();
    Ok(())
}