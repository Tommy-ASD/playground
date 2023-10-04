use std::{
    convert::Infallible,
    path::{Path, PathBuf},
};

use notify::{Error, Event, RecursiveMode, Watcher};
use traceback_error::{traceback, TracebackError};

use crate::create_path;

#[traceback_derive::traceback]
pub async fn listen(
    sender: std::sync::mpsc::Sender<PathBuf>,
) -> Result<Infallible, TracebackError> {
    let username = whoami::username();
    let entry = keyring::Entry::new("AUTOMATIC_YOUTUBE_UPLOAD_TARGET_FOLDER", &username)?;
    let path: PathBuf = match entry.get_password() {
        Ok(path) => Path::new(&path).to_path_buf(),
        // path has not yet been set
        Err(_) => match create_path() {
            Ok(path) => {
                let _ = entry.set_password(&path.as_os_str().to_str().unwrap());
                path
            }
            Err(e) => return Err(traceback!(err e)),
        },
    };

    println!("Listening on {path:?}");
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, Error>| match res {
        Ok(event) => {
            let path = &event.paths[0];
            println!("Got event: {path:?}");

            let _ = sender.send(path.clone());
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(&path, RecursiveMode::NonRecursive)?;

    loop {}
}
