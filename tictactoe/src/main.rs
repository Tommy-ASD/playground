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

// Function to display a welcome message
fn greeting() {
    println!(
        "\nRust TicTacToe\n\
         --------------\n\
         A simple game written in the Rust programming language.\n\
         Code is available at: https://github.com/flofriday/tictactoe"
    );
}

fn write_state(square: &SquareState, index: usize) {
    if square.is_default() {
        print!("{index}");
    } else {
        print!("{square}");
    }
}

// Function to draw the TicTacToe board
fn draw(state: &[SquareState]) {
    println!("\n");

    // Iterate over rows and columns to print the board
    for i in (0..3).rev() {
        let offset = i * 3;

        print!("-------------\n| ");
        write_state(&state[offset], offset + 1);
        print!(" | ");
        write_state(&state[offset + 1], offset + 1 + 1);
        print!(" | ");
        write_state(&state[offset + 2], offset + 2 + 1);
        println!(" |");
    }

    println!("-------------");
}

// Function to prompt the user for input
fn ask_user(state: &mut [SquareState], player: SquareState) {
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

            break;
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

// Function to check if the game is over (all fields used)
#[inline(always)]
fn is_over(state: &[SquareState]) -> bool {
    state.iter().all(|&v| !v.is_default())
}

// Main function to run the TicTacToe game
fn main() {
    let mut state = [SquareState::Unplayed; 9];
    let mut player = SquareState::X;

    // Welcome the player
    greeting();

    // Main game loop
    loop {
        // Draw the current state of the board
        draw(&state);

        // Prompt the current player for a move
        ask_user(&mut state, player);

        // Check if the current player has won
        if has_won(&state) {
            draw(&state);
            println!("Player '{player}' won! \\(^.^)/");
            break;
        }

        // Check if all fields are used (game is a draw)
        if is_over(&state) {
            draw(&state);
            println!("All fields are used. No one won. (._.)");
            break;
        }

        // Switch to the other player for the next turn
        player = if player == SquareState::X {
            SquareState::O
        } else {
            SquareState::X
        }
    }
}
