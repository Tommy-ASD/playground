/*
This function is simply the input() function from Python, as getting input is a bit annoying in Rust.
*/
pub fn input_inner() -> String {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => {}
        Err(e) => {
            println!("Error reading line: {}", e);
            println!("Please try again");
            return input_inner();
        }
    }
    line.trim().to_string()
}

/// Simulates the behavior of Python's `input()` function to capture user input.
///
/// The `input!` macro simplifies the process of capturing user input from the standard input (stdin).
/// It takes one or more prompts (strings) as arguments, displays them to the user, and then waits for user input.
///
/// # Usage
/// ```rust
/// fn main() {
///     let name: String = utils::input!("Enter your name: ");
///     println!("Hello, {}!", name);
/// }
/// ```
///
/// In this example, the `input!` macro displays the "Enter your name: " prompt and waits for the user to enter their name.
/// The entered value is stored in the `name` variable.
///
/// # Returns
///
/// - `String` - The user's input as a string, with leading and trailing whitespace removed.
#[macro_export]
macro_rules! input {
    ($($arg:expr),*) => {{
        $(print!("{} ", $arg);)* // Print each argument followed by a space
        println!(); // Print a newline at the end

        $crate::input_inner()
    }};
}
