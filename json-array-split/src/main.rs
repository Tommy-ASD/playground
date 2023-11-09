use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use serde_json::Value;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    in_file: PathBuf,

    #[arg(short, long)]
    out_folder: Option<PathBuf>,

    #[arg(long, conflicts_with = "split_size")]
    split_amount: Option<usize>,

    #[arg(long)]
    split_size: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let input: PathBuf = args.in_file;
    let output: PathBuf = match args.out_folder {
        Some(out) => out,
        None => derive_dest_from_src(&input).unwrap(),
    };

    println!(
        "In: {in_file:?}, out: {out_file:?}",
        in_file = input,
        out_file = output
    );

    println!("Creating folder {output:?}");
    create_dir_all(&output).unwrap();

    println!("Reading contents of {input:?}");
    let contents = std::fs::read_to_string(&input).unwrap();

    println!("Parsing to JSON...");
    let val: Value = serde_json::from_str(&contents).unwrap();

    println!("Parsing to array...");
    let parsed = val.as_array().unwrap();

    println!("Finished JSON parsing.");

    let chunk_size = match (args.split_amount, args.split_size) {
        (Some(_), Some(_)) => {
            panic!("It should be impossible to have both split-amount and split-size set")
        }
        (None, Some(num)) => num,
        (Some(num), None) => parsed.len() / num,
        (None, None) => 1,
    };

    println!("Splittng into chunks of size {chunk_size}");

    let parsed_split = parsed.chunks(chunk_size);
    let len = parsed_split.len() - 1;

    println!("Starting iteration through {len} chunks",);

    for (index, chunk) in parsed_split.enumerate() {
        println!("At index {index} of {len}");
        let mut output_clone = output.clone();
        output_clone.extend(vec![format!("{index}.json")]);
        println!("Making {output_clone:?}");
        let mut file = match File::create(&output_clone) {
            Ok(f) => f,
            Err(e) => {
                dbg!(e);
                continue;
            }
        };
        println!("Made {output_clone:?}, stringifying JSON");
        let chunk = match serde_json::to_string(chunk) {
            Ok(c) => c,
            Err(e) => {
                dbg!(e);
                continue;
            }
        };
        println!("Stringifyed, writing to {output_clone:?}");
        match file.write_all(chunk.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                dbg!(e);
                continue;
            }
        };
        println!("Done with {index}");
    }
}

fn derive_dest_from_src(src: &PathBuf) -> Option<PathBuf> {
    let mut output = None;
    if let (Some(parent), Some(Some(mut filename))) = (
        src.parent(),
        src.file_name()
            .map(|ok| ok.to_str().map(|ok| ok.to_string())),
    ) {
        // remove extension if it exists
        if let Some(Some(ext)) = src.extension().map(|ok| ok.to_str()) {
            filename = filename
                .strip_suffix(&format!(".{ext}"))
                .unwrap()
                .to_string();
        }
        let mut parent = parent.to_path_buf();
        parent.extend(vec![filename]);
        println!("Path: {parent:?}");
        output = Some(parent);
    }

    output
}
