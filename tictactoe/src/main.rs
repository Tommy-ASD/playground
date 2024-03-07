use core::fmt;
use std::io::Write;
use termcolor::WriteColor;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SquareState {
    X,
    O,
    Unplayed,
}

impl Default for SquareState {
    fn default() -> Self {
        SquareState::Unplayed
    }
}

impl SquareState {
    fn is_default(&self) -> bool {
        self == &Self::default()
    }
}

impl fmt::Display for SquareState {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);

        // Set color based on SquareState
        match self {
            SquareState::X => {
                stdout
                    .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Blue)))
                    .unwrap();
            }
            SquareState::O => {
                stdout
                    .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Green)))
                    .unwrap();
            }
            SquareState::Unplayed => {
                // Default color for unplayed state
                stdout
                    .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::White)))
                    .unwrap();
            }
        }

        // Print the SquareState symbol
        write!(
            &mut stdout,
            "{}",
            match self {
                SquareState::X => 'X',
                SquareState::O => 'O',
                SquareState::Unplayed => ' ',
            }
        )
        .unwrap();

        // Reset color
        stdout.reset().unwrap();

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubGameState {
    X,
    O,
    Unfinished,
    Draw,
}

impl Default for SubGameState {
    fn default() -> Self {
        SubGameState::Unfinished
    }
}

impl SubGameState {
    fn is_default(&self) -> bool {
        self == &Self::default()
    }
}

impl Into<SubGameState> for SquareState {
    fn into(self) -> SubGameState {
        match self {
            SquareState::O => SubGameState::O,
            SquareState::X => SubGameState::X,
            SquareState::Unplayed => SubGameState::Unfinished,
        }
    }
}

// Function to display a welcome message
fn greeting() {
    println!(
        "\nRust TicTacToe\n\
         --------------\n\
         A simple game written in the Rust programming language.\n\
         Code is available at: https://github.com/flofriday/tictactoe"
    );
}

fn as_string(square: &SquareState, index: usize) -> String {
    if square.is_default() {
        index.to_string()
    } else {
        square.to_string()
    }
}

fn draw_nested(state: &[[SquareState; 9]]) {
    for i in (0..3).rev() {
        let offset = i * 3;
        draw(&state[offset], offset);
        draw(&state[offset + 1], offset + 1);
        draw(&state[offset + 2], offset + 2);
    }
}

// Function to draw the TicTacToe board
fn draw(state: &[SquareState], index: usize) {
    let mut board = format!(
        "| ------{corrected_index}------ |\n",
        corrected_index = index + 1
    );

    // Iterate over rows and columns to print the board
    for i in (0..3).rev() {
        let offset = i * 3;

        let line = format!(
            "| | {} | {} | {} | |\n",
            as_string(&state[offset], offset + 1),
            as_string(&state[offset + 1], offset + 1 + 1),
            as_string(&state[offset + 2], offset + 2 + 1)
        );

        let separator = "| ------------- |\n";
        board.push_str(&line);
        board.push_str(separator);
    }
    println!("{board}");
}

// Function to prompt the user for input
fn ask_user(state: &mut [SquareState], player: SquareState) -> usize {
    loop {
        print!("Player '{player}', enter a number: ");

        // Read user input
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Couldn't read line! Try again.");
            continue;
        }

        // Parse input as a number
        if let Ok(number) = input.trim().parse::<usize>() {
            if number < 1 || number > 9 {
                println!("The field number must be between 1 and 9.");
                continue;
            }

            let number = number - 1;

            // Check if the chosen field is already taken
            if state[number] == SquareState::X || state[number] == SquareState::O {
                println!(
                    "This field is already taken by '{player}'.",
                    player = &state[number]
                );
                continue;
            }

            // Update the game state with the player's move
            state[number] = player;

            break number;
        } else {
            println!("Only numbers are allowed.");
            continue;
        }
    }
}

// Function to check if a player has won
fn has_won(state: &[SquareState]) -> bool {
    for tmp in 0..3 {
        if state[tmp] == SquareState::Unplayed {
            continue;
        }
        if state[tmp] == state[tmp + 3] && state[tmp] == state[tmp + 6] {
            return true;
        }

        let tmp = tmp * 3;

        if state[tmp] == SquareState::Unplayed {
            continue;
        }

        if state[tmp] == state[tmp + 1] && state[tmp] == state[tmp + 2] {
            return true;
        }
    }

    if (state[0] != SquareState::Unplayed && state[0] == state[4] && state[0] == state[8])
        || (state[2] != SquareState::Unplayed && state[2] == state[4] && state[2] == state[6])
    {
        return true;
    }

    false
}

// Function to check if a player has won
fn has_won_outer(state: &[SubGameState]) -> bool {
    for tmp in 0..3 {
        if (state[tmp] == SubGameState::Unfinished || state[tmp] == SubGameState::Draw) {
            continue;
        }
        if state[tmp] == state[tmp + 3] && state[tmp] == state[tmp + 6] {
            return true;
        }

        let tmp = tmp * 3;

        if (state[tmp] == SubGameState::Unfinished || state[tmp] == SubGameState::Draw) {
            continue;
        }

        if state[tmp] == state[tmp + 1] && state[tmp] == state[tmp + 2] {
            return true;
        }
    }

    if ((state[0] != SubGameState::Unfinished || state[0] != SubGameState::Draw)
        && state[0] == state[4]
        && state[0] == state[8])
        || ((state[2] != SubGameState::Unfinished || state[2] != SubGameState::Draw)
            && state[2] == state[4]
            && state[2] == state[6])
    {
        return true;
    }

    false
}

// Function to check if the game is over (all fields used)
#[inline(always)]
fn is_over(state: &[SquareState]) -> bool {
    state.iter().all(|&v| !v.is_default())
}

// Main function to run the TicTacToe game
fn main() {
    let mut state = [[SquareState::Unplayed; 9]; 9]; // second element in the tuple represents who won
    let mut outer_state = [SubGameState::Unfinished; 9];
    let mut player = SquareState::X;

    draw_nested(&state);

    // Welcome the player
    greeting();

    let mut index = 4;

    // Main game loop
    loop {
        let mv = in_square(&mut state[index], &mut player, index);
        outer_state[index] = mv.current_state;
        index = mv.index;
        println!("State; {substate:?}", substate = mv.current_state);

        if has_won_outer(&outer_state) {
            println!("Player '{player}' won the entire thing! \\(^.^)/");
            return;
        };

        // Switch to the other player for the next turn
        player = if player == SquareState::X {
            SquareState::O
        } else {
            SquareState::X
        };
        println!("Full state; {outer_state:?}");
    }
}

pub struct Move {
    pub outer_index: usize,
    pub inner_index: usize,
    pub player: SquareState,
}

pub struct InnerMove {
    pub index: usize,
    pub player: SquareState,

    pub current_state: SubGameState,
}

fn in_square(state: &mut [SquareState], player: &mut SquareState, index: usize) -> InnerMove {
    let mut end_state = SubGameState::Unfinished;
    // Draw the current state of the board
    draw(state, index);

    // Prompt the current player for a move
    let played_index = ask_user(state, *player);

    // Check if the current player has won
    if has_won(state) {
        draw(state, index);
        println!("Player '{player}' won! \\(^.^)/");
        end_state = (*player).into();
        return InnerMove {
            index: played_index,
            player: *player,
            current_state: end_state,
        };
    }

    // Check if all fields are used (game is a draw)
    if is_over(state) {
        draw(state, index);
        println!("All fields are used. No one won. (._.)");
        end_state = SubGameState::Draw;
        return InnerMove {
            index: played_index,
            player: *player,
            current_state: end_state,
        };
    }
    InnerMove {
        index: played_index,
        player: *player,
        current_state: end_state,
    }
}
