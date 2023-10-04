// #![windows_subsystem = "windows"]

mod listener;
mod yt;

use std::{path::PathBuf, sync::Arc};

use tokio::sync::Mutex;

use traceback_error::{traceback, TracebackError};

use native_dialog::FileDialog;

pub(crate) static GOOGLE_CLIENT_ID: &str = include_str!("../id.secret");
pub(crate) static GOOGLE_CLIENT_SECRET: &str = include_str!("../secret.secret");

#[traceback_derive::traceback]
#[tokio::main]
async fn main() -> Result<(), TracebackError> {
    traceback!();
    let _ = reset_entries();

    traceback!();
    // Create a channel for sending PathBuf
    let (tx, rx) = tokio::sync::mpsc::channel::<PathBuf>(32); // You can adjust the buffer size as needed

    traceback!();
    let rx = Arc::new(Mutex::new(rx)); // Wrap the Receiver in an Arc and Mutex

    traceback!();
    let listener_handle = tokio::task::spawn(listener::listen(tx));

    traceback!();
    let rx_clone = Arc::clone(&rx); // Create a clone of the Arc for use in the yt::handler task

    traceback!();
    let yt_handle = tokio::task::spawn(yt::handler(rx_clone));

    traceback!();
    tokio::select! {
        _ = listener_handle => (),
        _ = yt_handle => (),
    }

    traceback!();
    Ok(())
}

fn create_path() -> Result<PathBuf, TracebackError> {
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("Folder", &["png"])
        .show_open_single_dir()
        .unwrap();

    let path = match path {
        Some(path) => path,
        None => return Err(traceback!()),
    };
    Ok(path)
}

#[traceback_derive::traceback]
fn reset_entries() -> Result<(), TracebackError> {
    let username = whoami::username();
    match keyring::Entry::new("AUTOMATIC_YOUTUBE_UPLOAD", &username) {
        Ok(entry) => {
            let _ = entry.delete_password();
        }
        Err(_) => {}
    }

    match keyring::Entry::new("AUTOMATIC_YOUTUBE_UPLOAD_TARGET_FOLDER", &username) {
        Ok(entry) => {
            let _ = entry.delete_password();
        }
        Err(_) => {}
    }
    Ok(())
}
