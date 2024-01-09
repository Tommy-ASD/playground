use std::io::{BufReader, Write};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct LangFileEntry {
    #[serde(rename = "English")]
    english: String,
    #[serde(rename = "French")]
    french: String,
    alpha2: String,
    #[serde(rename = "alpha3-b")]
    alpha3_b: String,
    #[serde(rename = "alpha3-t")]
    alpha3_t: String,
}

fn main() {
    let file = std::fs::read_to_string("./whisp.txt").unwrap();
    let map = file
        .lines()
        .into_iter()
        .map(|line| {
            let mut split = line.split(" ");
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect::<Vec<(&str, &str)>>();
    let lang_file = std::fs::File::open("./lang.json").unwrap();
    let reader = BufReader::new(lang_file);
    let mut u: Vec<LangFileEntry> = serde_json::from_reader(reader).unwrap();

    u = u
        .into_iter()
        .filter(|entry| {
            let langs: (Vec<&str>, Vec<&str>) = map.clone().into_iter().unzip();
            let lang_names = langs
                .0
                .iter()
                .map(|names| names.split("; ").collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>();
            !entry.alpha2.is_empty()
                && langs.1.contains(&entry.alpha2.as_str())
                && lang_names.concat().contains(&entry.english.as_str())
        })
        .collect();

    let res = serde_json::to_string_pretty(&u).unwrap();
    let mut file = std::fs::File::create("res.json").unwrap();
    file.write_all(res.as_bytes()).unwrap();

    println!("{u:#?}");
}
