use std::sync::Arc;

use rand::Rng;
use tokio::sync::Mutex;
use crate::{Context, Error, MinesweeperManager, UserData};

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayType {
    #[default]
    Press,
    Flag
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayResult {
    OutOfBounds,
    CellAlreadyPlayed,
    CellFlagged,
    Unflagged,
    Flagged,
    PlayedClear(u8),
    PlayedMine
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellState {
    #[default]
    Unplayed,
    Pressed,
    Flagged, 
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellType {
    #[default]
    Clear,
    Mine
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cell {
    state: CellState,
    ctype: CellType,
    mine_neighbors: u8
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoardDisplayType {
    #[default]
    InPlay,
    Finished
}

impl Cell {
    pub fn set_mine(&mut self) {
        self.ctype = CellType::Mine;
    }
}

#[derive(Default)]
pub struct Board {
    grid: Vec<Vec<Cell>>,
    mines: Vec<(usize, usize)>
}

impl Board {
    pub fn new(height: usize, width: usize) -> Self {
        let grid = vec![vec![Cell::default(); width as usize]; height as usize];

        Self {
            grid,
            mines: Vec::new()
        }
    }
    pub fn get_width(&self) -> usize {
        self.grid.len()
    }
    pub fn get_height(&self) -> usize {
        self.grid[0].len()
    }
    pub fn generate_mines(&mut self, mine_amount: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..mine_amount {
            let y = rng.gen_range(0..self.get_height());
            let x = rng.gen_range(0..self.get_width());
            self.mines.push((x, y));
            self.grid[y as usize][x as usize].set_mine();
            for index in self.get_neighbors((y, x)) {
                self.grid[index.0][index.1].mine_neighbors += 1;
            }
        }
    }
    pub fn get_cell_at(&self, index: (usize, usize)) -> Option<&Cell> {
        self.grid.get(index.0).and_then(|row| row.get(index.1))
    }
    pub fn get_cell_at_mut(&mut self, index: (usize, usize)) -> Option<&mut Cell> {
        self.grid.get_mut(index.0).and_then(|row| row.get_mut(index.1))
    }
    pub fn get_neighbors(&self, index: (usize, usize)) -> Vec<(usize, usize)> {
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
    pub fn clear_zeroes(&mut self, index: (usize, usize)) {
        let cell = self.get_cell_at_mut(index).unwrap();
        let cell_state_clone = cell.state.clone();
        cell.state = CellState::Pressed;
        
        if cell.mine_neighbors == 0 && cell_state_clone == CellState::Unplayed {
            self.get_neighbors(index).iter().for_each(|neighbor| {
                self.clear_zeroes(*neighbor);
            })
        }
    }
    pub fn play(&mut self, index: (usize, usize), playtype: PlayType) -> PlayResult {
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
                        CellType::Mine => {
                            cell.state = CellState::Pressed;
                            PlayResult::PlayedMine
                        }
                    }
                }
                PlayType::Flag => {
                    cell.state = CellState::Flagged;
                    PlayResult::Flagged
                }
            }
        }
    }
    
    pub fn display(&self) {
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
    pub fn display_lost(&self) {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                let cell = &self.grid[y as usize][x as usize];
                match (&cell.ctype, &cell.state) {
                    (&CellType::Clear, _) => print!("{} ", number_to_emoji(cell.mine_neighbors)),
                    (&CellType::Mine, &CellState::Pressed) => print!("💥 "),
                    (&CellType::Mine, &CellState::Unplayed) => print!("💣 "),
                    (&CellType::Mine, &CellState::Flagged) => print!("🚩 "),
                }
            }
            println!("");
        }
    }
    pub fn to_emojis(&self) -> String {
        let mut result = "".to_string();
        result.push_str("⬜");
        result.push_str("⬜");
        for x in 0..self.get_width() {
            result.push_str(&number_to_emoji(x as u8));
        }
        result.push('\n');
        result.push_str("⬜");
        result.push_str("⬜");
        for _ in 0..self.get_height() {
            result.push_str("⬜");
        }
        result.push('\n');
        for y in 0..self.get_height() {
            result.push_str(&number_to_emoji(y as u8));
            result.push_str("⬜");
            for x in 0..self.get_width() {
                let cell = &self.grid[y as usize][x as usize];
                match cell.state {
                    CellState::Unplayed => result.push_str("◼️"),
                    CellState::Pressed => result.push_str(number_to_emoji(cell.mine_neighbors)),
                    CellState::Flagged => result.push_str("🚩"),
                }
            }
            result.push('\n');
        }
        result
    }
    pub fn to_emojis_lost(&self) -> String {
        let mut result = "".to_string();
        result.push_str("⬜");
        result.push_str("⬜");
        for x in 0..self.get_width() {
            result.push_str(&number_to_emoji(x as u8));
        }
        result.push('\n');
        result.push_str("⬜");
        result.push_str("⬜");
        for _ in 0..self.get_height() {
            result.push_str("⬜");
        }
        result.push('\n');
        for y in 0..self.get_height() {
            result.push_str(&number_to_emoji(y as u8));
            result.push_str("⬜");
            for x in 0..self.get_width() {
                let cell = &self.grid[y as usize][x as usize];
                match (&cell.ctype, &cell.state) {
                    (&CellType::Clear, _) => result.push_str(number_to_emoji(cell.mine_neighbors)),
                    (&CellType::Mine, &CellState::Pressed) => result.push_str("💥"),
                    (&CellType::Mine, &CellState::Unplayed) => result.push_str("💣"),
                    (&CellType::Mine, &CellState::Flagged) => result.push_str("🚩"),
                }
            }
            result.push('\n');
        }
        result
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
        _ => "❓",
    }
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn minesweeper(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let mut users_lock = ctx.data().users.lock().await;
    let udata = users_lock.entry(ctx.author().id).or_insert_with(|| Arc::new(Mutex::new(UserData::default())));
    let user_data_arc = Arc::clone(udata);
    let mut board = Board::new(10, 10);
    board.generate_mines(10);
    let display = board.to_emojis();
    user_data_arc.lock().await.minesweeper = Some(MinesweeperManager {
        board,
        origin_channel_id: ctx.channel_id()
    });
    ctx.reply(display).await.unwrap();
    Ok(())
}