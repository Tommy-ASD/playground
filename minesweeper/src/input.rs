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

        $crate::input::input_inner()
    }};
}

/// Gets the type of a value as a string,
/// and returns it.
/// Example:
/// ```
/// let x = 5;
/// type_of(&x);
/// // Returns "i32"
/// ```
pub fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

/// Gets the type of a value as a string,
/// and prints it.
/// Example:
/// ```
/// let x = 5;
/// print_type_of(&x);
/// // Prints "i32"
/// ```
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

/// Sets a custom panic handler for handling panics in Rust programs.
///
/// This macro allows you to easily set a custom panic handler function using the
/// `std::panic::set_hook` function. A panic handler is a function that is called
/// when a panic occurs in your program, allowing you to customize how panics are
/// handled.
///
/// # Usage
///
/// To use this macro, provide the name of the function you want to use as the
/// custom panic handler. This function should have the following signature:
///
/// ```rust
/// fn my_panic_handler(info: &std::panic::PanicInfo);
/// ```
///
/// # Example
///
/// ```rust
/// // Define a custom panic handler function
/// fn my_panic_handler(info: &std::panic::PanicInfo) {
///     println!("Custom panic handler called: {:?}", info);
///     // You can add your custom panic handling logic here
/// }
///
/// // Use the set_panic_handler macro to set the custom panic handler
/// set_panic_handler!(my_panic_handler);
///
/// // Any panics that occur in the program will now be handled by my_panic_handler.
/// ```
///
/// For more information on panic handling in Rust, see the Rust documentation on
/// panic handling: https://doc.rust-lang.org/std/panic/index.html
#[macro_export]
macro_rules! set_panic_handler {
    ($handler:ident) => {
        std::panic::set_hook(Box::new($handler))
    };
}


pub fn get_multi_mut<T>(v: &mut [T], i: usize, j: usize) -> Option<(&mut T, &mut T)> {
    if i == j {
        return None;
    }
    let (start, end) = if i < j { (i, j) } else { (j, i) };

    let (first, second) = v.split_at_mut(start + 1);
    Some((&mut first[start], &mut second[end - start - 1]))
}