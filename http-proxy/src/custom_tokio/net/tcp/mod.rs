//! TCP utility types.

pub(crate) mod listener;

cfg_not_wasi! {
    pub(crate) mod socket;
}

mod split;
pub(crate)use split::{ReadHalf, WriteHalf};

mod split_owned;
pub(crate)use split_owned::{OwnedReadHalf, OwnedWriteHalf, ReuniteError};

pub(crate) mod stream;
pub(crate) use stream::TcpStream;
