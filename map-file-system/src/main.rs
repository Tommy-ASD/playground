use std::fs::{self};
use std::path::Path;

fn map_file_system(path: &Path) {
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    println!("{}", entry_path.display());

                    let skips = vec![
                        ".git",
                        "src",
                        ".vscode",
                        ".vs",
                        "node_modules",
                        "errors",
                        "queries_ran",
                        "logs",
                    ];

                    // Recursively call the function for directories
                    if entry_path.is_dir() {
                        if entry_path.ends_with("target") {
                            println!("Deleting");
                            std::fs::remove_dir_all(entry_path);
                            return;
                        }
                        for skip in skips {
                            if entry_path.ends_with(skip) {
                                println!("Skipping");
                                return;
                            }
                        }
                        map_file_system(&entry_path);
                    }
                }
            }
        }
    }
}

fn main() {
    let start_dir = std::env::current_dir().unwrap(); // Start from the current directory
    map_file_system(&start_dir);
}
