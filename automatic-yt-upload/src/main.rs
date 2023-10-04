#![windows_subsystem = "windows"]

mod block_on;
mod listener;
mod yt;

use std::{path::PathBuf, str::FromStr, sync::Arc};

use log::{error, info};
use tokio::sync::Mutex;

use traceback_error::{traceback, TracebackError};

use native_dialog::FileDialog;

pub(crate) static GOOGLE_CLIENT_ID: &str = include_str!("../id.secret");
pub(crate) static GOOGLE_CLIENT_SECRET: &str = include_str!("../secret.secret");

#[traceback_derive::traceback]
#[tokio::main]
async fn main() -> Result<(), TracebackError> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let _ = reset_entries();

    // Create a channel for sending PathBuf
    let (tx, rx) = tokio::sync::mpsc::channel::<PathBuf>(32); // You can adjust the buffer size as needed

    let rx = Arc::new(Mutex::new(rx)); // Wrap the Receiver in an Arc and Mutex

    let listener_handle = tokio::task::spawn(listener::listen(tx));

    let rx_clone = Arc::clone(&rx); // Create a clone of the Arc for use in the yt::handler task

    let yt_handle = tokio::task::spawn(yt::handler(rx_clone));

    tokio::select! {
        listen_end = listener_handle => {
            error!("Listener ended: {listen_end:?}");
        },
        yt_end = yt_handle => {
            error!("YT ended: {yt_end:?}")
        },
    }

    Ok(())
}

#[traceback_derive::traceback]
fn create_path() -> Result<PathBuf, TracebackError> {
    let path = match FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("Folder", &["png"])
        .show_open_single_dir()
    {
        Ok(val) => val,
        Err(_e) => PathBuf::from_str("E:\\Outplayed\\Valorant").ok(),
    };
    info!("Created path: {path:?}");

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
