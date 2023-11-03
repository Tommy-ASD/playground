use std::collections::HashMap;

mod dom;
mod parser;

const HTML: &str = include_str!("../html.html");

fn main() {
    println!("Hello, world!");
    let node = parser::parse(HTML.to_string());
    println!("Node: {node:?}");
}
