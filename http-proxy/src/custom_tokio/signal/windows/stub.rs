//! Stub implementations for the platform API so that rustdoc can build linkable
//! documentation on non-windows platforms.

use crate::custom_tokio::signal::RxFuture;
use std::io;

pub(crate) fn ctrl_break() -> io::Result<RxFuture> {
    panic!()
}

pub(crate) fn ctrl_close() -> io::Result<RxFuture> {
    panic!()
}

pub(crate) fn ctrl_c() -> io::Result<RxFuture> {
    panic!()
}

pub(crate) fn ctrl_logoff() -> io::Result<RxFuture> {
    panic!()
}

pub(crate) fn ctrl_shutdown() -> io::Result<RxFuture> {
    panic!()
}
