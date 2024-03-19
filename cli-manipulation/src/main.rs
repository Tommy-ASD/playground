use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() {
    print!("This is some text without a newline");
    io::stdout().flush().unwrap();

    // Wait for a moment (just for demonstration purposes)
    thread::sleep(Duration::from_secs(2));

    print!("\x1B[5C"); // Carriage return to the beginning of the line
    io::stdout().flush().unwrap();

    println!("Updated text on the same line");
}
