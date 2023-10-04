use std::{
    convert::Infallible,
    path::{Path, PathBuf},
};

use notify::{Error, Event, RecursiveMode, Watcher};
use traceback_error::{traceback, TracebackError};

use crate::create_path;

#[traceback_derive::traceback]
pub async fn listen(
    sender: tokio::sync::mpsc::Sender<PathBuf>,
) -> Result<Infallible, TracebackError> {
    traceback!(format!("Entered listen"));
    let username = whoami::username();
    traceback!(format!("{username}"));
    let entry = keyring::Entry::new("AUTOMATIC_YOUTUBE_UPLOAD_TARGET_FOLDER", &username)?;
    traceback!(format!("Entry fetched"));
    let path: PathBuf = match entry.get_password() {
        Ok(path) => {
            traceback!(format!("Password found; not creating path"));
            Path::new(&path).to_path_buf()
        }
        // path has not yet been set
        Err(_) => {
            traceback!(format!("Creating entry"));
            match create_path() {
                Ok(path) => {
                    traceback!(format!("Created entry"));
                    let _ = entry.set_password(&path.as_os_str().to_str().unwrap());
                    path
                }
                Err(e) => {
                    traceback!(format!("Failed to create entry"));
                    return Err(traceback!(err e));
                }
            }
        }
    };

    traceback!(format!("Listening on {:?}", &path));
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, Error>| match res {
        Ok(event) => {
            let path = &event.paths[0];
            traceback!(format!("Got event {:?}", &path));

            let _ = sender.send(path.clone());
        }
        Err(e) => {
            traceback!(format!("watch error: {:?}", e));
        }
    })?;
    traceback!(format!("Created watcher"));

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(&path, RecursiveMode::NonRecursive)?;
    traceback!(format!("Watcher is watching"));

    loop {}
}
