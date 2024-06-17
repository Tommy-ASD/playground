use log::info;
use std::{
    convert::Infallible,
    path::{Path, PathBuf},
};

use notify::{Error, Event, RecursiveMode, Watcher};
use traceback_error::{traceback, TracebackError};

use crate::{block_on::block_on, create_path};

#[traceback_derive::traceback]
pub async fn listen(
    sender: tokio::sync::mpsc::Sender<PathBuf>,
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
            Err(e) => {
                return Err(traceback!(err e));
            }
        },
    };

    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, Error>| match res {
        Ok(event) => {
            let path = &event.paths[0];

            if path.extension().map(|string| string.to_str()) != Some(Some("mp4")) {
                return;
            }

            match block_on(sender.send(path.clone())) {
                Ok(_) => println!("Successfully sent event to receiver"),
                Err(e) => {
                    info!("Failed to send event to receiver");
                    info!("{e}");
                    info!("{e:?}");
                }
            };
        }
        Err(e) => {
            info!("watch error: {:?}", e);
        }
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(&path, RecursiveMode::Recursive)?;

    loop {}
}
