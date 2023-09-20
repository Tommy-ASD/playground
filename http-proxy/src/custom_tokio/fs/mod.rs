#![cfg(not(loom))]

//! Asynchronous file and standard stream adaptation.
//!
//! This module contains utility methods and adapter types for input/output to
//! files or standard streams (`Stdin`, `Stdout`, `Stderr`), and
//! filesystem manipulation, for use within (and only within) a Tokio runtime.
//!
//! Tasks run by *worker* threads should not block, as this could delay
//! servicing reactor events. Portable filesystem operations are blocking,
//! however. This module offers adapters which use a `blocking` annotation
//! to inform the runtime that a blocking operation is required. When
//! necessary, this allows the runtime to convert the current thread from a
//! *worker* to a *backup* thread, where blocking is acceptable.
//!
//! ## Usage
//!
//! Where possible, users should prefer the provided asynchronous-specific
//! traits such as [`AsyncRead`], or methods returning a `Future` or `Poll`
//! type. Adaptions also extend to traits like `std::io::Read` where methods
//! return `std::io::Result`. Be warned that these adapted methods may return
//! `std::io::ErrorKind::WouldBlock` if a *worker* thread can not be converted
//! to a *backup* thread immediately.
//!
//! **Warning**: These adapters may create a large number of temporary tasks,
//! especially when reading large files. When performing a lot of operations
//! in one batch, it may be significantly faster to use [`spawn_blocking`]
//! directly:
//!
//! ```
//! use crate::custom_tokio::fs::File;
//! use std::io::{BufReader, BufRead};
//! async fn count_lines(file: File) -> Result<usize, std::io::Error> {
//!     let file = file.into_std().await;
//!     crate::custom_tokio::task::spawn_blocking(move || {
//!         let line_count = BufReader::new(file).lines().count();
//!         Ok(line_count)
//!     }).await?
//! }
//! ```
//!
//! [`spawn_blocking`]: fn@crate::custom_tokio::task::spawn_blocking
//! [`AsyncRead`]: trait@crate::custom_tokio::io::AsyncRead

mod canonicalize;
pub(crate) use self::canonicalize::canonicalize;

mod create_dir;
pub(crate) use self::create_dir::create_dir;

mod create_dir_all;
pub(crate) use self::create_dir_all::create_dir_all;

mod dir_builder;
pub(crate) use self::dir_builder::DirBuilder;

mod file;
pub(crate) use self::file::File;

mod hard_link;
pub(crate) use self::hard_link::hard_link;

mod metadata;
pub(crate) use self::metadata::metadata;

mod open_options;
pub(crate) use self::open_options::OpenOptions;

mod read;
pub(crate) use self::read::read;

mod read_dir;
pub(crate) use self::read_dir::{read_dir, DirEntry, ReadDir};

mod read_link;
pub(crate) use self::read_link::read_link;

mod read_to_string;
pub(crate) use self::read_to_string::read_to_string;

mod remove_dir;
pub(crate) use self::remove_dir::remove_dir;

mod remove_dir_all;
pub(crate) use self::remove_dir_all::remove_dir_all;

mod remove_file;
pub(crate) use self::remove_file::remove_file;

mod rename;
pub(crate) use self::rename::rename;

mod set_permissions;
pub(crate) use self::set_permissions::set_permissions;

mod symlink_metadata;
pub(crate) use self::symlink_metadata::symlink_metadata;

mod write;
pub(crate) use self::write::write;

mod copy;
pub(crate) use self::copy::copy;

mod try_exists;
pub(crate) use self::try_exists::try_exists;

#[cfg(test)]
mod mocks;

feature! {
    #![unix]

    mod symlink;
    pub(crate)use self::symlink::symlink;
}

cfg_windows! {
    mod symlink_dir;
    pub(crate)use self::symlink_dir::symlink_dir;

    mod symlink_file;
    pub(crate)use self::symlink_file::symlink_file;
}

use std::io;

#[cfg(not(test))]
use crate::custom_tokio::blocking::spawn_blocking;
#[cfg(test)]
use mocks::spawn_blocking;

pub(crate) async fn asyncify<F, T>(f: F) -> io::Result<T>
where
    F: FnOnce() -> io::Result<T> + Send + 'static,
    T: Send + 'static,
{
    match spawn_blocking(f).await {
        Ok(res) => res,
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "background task failed",
        )),
    }
}
