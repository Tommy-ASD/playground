cfg_macros! {
    pub(crate)use crate::custom_tokio::future::poll_fn;
    pub(crate)use crate::custom_tokio::future::maybe_done::maybe_done;

    #[doc(hidden)]
    pub(crate)fn thread_rng_n(n: u32) -> u32 {
        crate::custom_tokio::runtime::context::thread_rng_n(n)
    }
}

pub(crate)use std::future::Future;
pub(crate)use std::pin::Pin;
pub(crate)use std::task::Poll;
