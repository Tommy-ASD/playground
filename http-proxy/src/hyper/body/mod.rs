//! Streaming bodies for Requests and Responses
//!
//! For both [Clients](crate::hyper::client) and [Servers](crate::hyper::server), requests and
//! responses use streaming bodies, instead of complete buffering. This
//! allows applications to not use memory they don't need, and allows exerting
//! back-pressure on connections by only reading when asked.
//!
//! There are two pieces to this in hyper:
//!
//! - **The [`Body`](Body) trait** describes all possible bodies.
//!   hyper allows any body type that implements `Body`, allowing
//!   applications to have fine-grained control over their streaming.
//! - **The [`Incoming`](Incoming) concrete type**, which is an implementation
//!   of `Body`, and returned by hyper as a "receive stream" (so, for server
//!   requests and client responses).
//!
//! There are additional implementations available in [`http-body-util`][],
//! such as a `Full` or `Empty` body.
//!
//! [`http-body-util`]: https://docs.rs/http-body-util

pub use bytes::{Buf, Bytes};
pub use http_body::Body;
pub use http_body::Frame;
pub use http_body::SizeHint;

pub use self::incoming::Incoming;

#[cfg(feature = "http1")]
pub(crate) use self::incoming::Sender;
pub(crate) use self::length::DecodedLength;

mod incoming;
mod length;

fn _assert_send_sync() {
    fn _assert_send<T: Send>() {}
    fn _assert_sync<T: Sync>() {}

    _assert_send::<Incoming>();
    _assert_sync::<Incoming>();
}
