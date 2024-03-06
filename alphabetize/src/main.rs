use std::io::BufRead;

fn main() {
    let binding = std::fs::read_to_string("./lines").unwrap();
    let mut lines = binding
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            let mut line = line
                .strip_prefix("sudo apt install ")
                .unwrap()
                .split("#")
                .collect::<Vec<&str>>()[0]
                .replace(" ", "\n");
            line.push('\n');
            line
        })
        .collect::<String>();
    let mut lines = lines
        .lines()
        .filter(|line| !line.is_empty() && line.chars().next().unwrap().is_alphabetic())
        .collect::<Vec<&str>>();

    lines.sort();
    lines.dedup();
    lines.iter().for_each(|line| {
        println!("{line}");
    });
}
