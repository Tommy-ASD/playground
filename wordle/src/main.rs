use crate::words::{get_word, valid_word};

mod words;

use colored::*;

fn main() {
    wordle_loop();
}

fn wordle_loop() {
    loop {
        println!("Guess a word");
        let correct = get_word();
        let mut counter = 1;
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            println!("You entered '{input}' as guess {counter}");
            if valid_word(&input) {
                check(&input, correct);
                counter += 1;
                if correct == input {
                    println!("You win! Guessed in {counter} tries");
                    break;
                } else if counter > 6 {
                    println!("You used your 6 guesses! The word was {correct}",);
                    break;
                }
            } else {
                println!("Invalid word");
            }
        }
        println!("Play again? (y)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input != "y" {
            break;
        };
    }
}

fn check(current: &str, correct: &str) {
    let incorrect = "⬜".white();
    let mut result: [ColoredString; 5] = [
        incorrect.clone(),
        incorrect.clone(),
        incorrect.clone(),
        incorrect.clone(),
        incorrect.clone(),
    ];
    for i in 0..current.len() {
        if current.chars().nth(i).unwrap() == correct.chars().nth(i).unwrap() {
            result[i] = current.chars().nth(i).unwrap().to_string().green();
        } else if correct.contains(current.chars().nth(i).unwrap()) {
            result[i] = current.chars().nth(i).unwrap().to_string().yellow();
        } else {
            result[i] = current.chars().nth(i).unwrap().to_string().white();
        }
    }
    println!(
        "{}{}{}{}{}",
        result[0], result[1], result[2], result[3], result[4]
    );
}
