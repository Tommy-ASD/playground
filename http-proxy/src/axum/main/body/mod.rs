//! HTTP body utilities.

#[doc(no_inline)]
pub use http_body::Body as HttpBody;

#[doc(no_inline)]
pub use bytes::Bytes;

#[doc(inline)]
pub use crate::axum::core::body::Body;
