use std::{fs::File, path::Path};

use notify::{Error, Event, RecommendedWatcher, RecursiveMode, Watcher};

fn main() -> Result<(), Error> {
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(|res: Result<Event, Error>| match res {
        Ok(event) => {
            let path = &event.paths[0];
            println!("Got event: {path:?}");
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(
        Path::new("C:\\Users\\tommy\\Documents\\prog\\playground"),
        RecursiveMode::NonRecursive,
    )?;

    loop {}

    Ok(())
}
