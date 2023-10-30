use std::collections::HashMap;

mod dom;
mod parser;

fn main() {
    println!("Hello, world!");
    let node = parser::parse("<html><body id=\"test\">Hello, world!</body></html>".to_string());
    println!("Node: {node:?}");
}
