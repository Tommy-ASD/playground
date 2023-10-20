use std::fs::{self, DirEntry};
use std::path::Path;

fn map_file_system<F>(path: &Path, handler: F)
where
    F: FnOnce(&DirEntry) -> () + Clone,
{
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    handler.clone()(&entry);
                    let entry_path = entry.path();
                    println!("{}", entry_path.display());

                    // Recursively call the function for directories
                    if entry_path.is_dir() {
                        map_file_system(&entry_path, handler.clone());
                    }
                }
            }
        }
    }
}

fn main() {
    let start_dir = std::env::current_dir().unwrap(); // Start from the current directory
    let handler = |entry: &DirEntry| {
        let path = entry.path();
    };
    map_file_system(&start_dir, handler);
}
