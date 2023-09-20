use std::future::Future;

cfg_rt! {
    #[track_caller]
    pub(crate) fn block_on<F: Future>(f: F) -> F::Output {
        let mut e = crate::custom_tokio::runtime::context::try_enter_blocking_region().expect(
            "Cannot block the current thread from within a runtime. This \
            happens because a function attempted to block the current \
            thread while the thread is being used to drive asynchronous \
            tasks."
        );
        e.block_on(f).unwrap()
    }
}

cfg_not_rt! {
    #[track_caller]
    pub(crate) fn block_on<F: Future>(f: F) -> F::Output {
        let mut park = crate::custom_tokio::runtime::park::CachedParkThread::new();
        park.block_on(f).unwrap()
    }
}
