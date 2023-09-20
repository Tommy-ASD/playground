//! Unix specific network types.
// This module does not currently provide any public API, but it was
// unintentionally defined as a public module. Hide it from the documentation
// instead of changing it to a private module to avoid breakage.
#[doc(hidden)]
pub(crate)mod datagram;

pub(crate) mod listener;

mod split;
pub(crate)use split::{ReadHalf, WriteHalf};

mod split_owned;
pub(crate)use split_owned::{OwnedReadHalf, OwnedWriteHalf, ReuniteError};

mod socketaddr;
pub(crate)use socketaddr::SocketAddr;

pub(crate) mod stream;
pub(crate) use stream::UnixStream;

mod ucred;
pub(crate)use ucred::UCred;

pub(crate)mod pipe;

/// A type representing process and process group IDs.
#[allow(non_camel_case_types)]
pub(crate)type uid_t = u32;

/// A type representing user ID.
#[allow(non_camel_case_types)]
pub(crate)type gid_t = u32;

/// A type representing group ID.
#[allow(non_camel_case_types)]
pub(crate)type pid_t = i32;
