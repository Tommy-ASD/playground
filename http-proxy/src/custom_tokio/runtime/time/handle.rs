use crate::custom_tokio::runtime::time::TimeSource;
use std::fmt;

/// Handle to time driver instance.
pub(crate) struct Handle {
    pub(crate) time_source: TimeSource,
    pub(crate) inner: super::Inner,
}

impl Handle {
    /// Returns the time source associated with this handle.
    pub(crate) fn time_source(&self) -> &TimeSource {
        &self.time_source
    }

    /// Checks whether the driver has been shutdown.
    pub(crate) fn is_shutdown(&self) -> bool {
        self.inner.is_shutdown()
    }

    /// Track that the driver is being unparked
    pub(crate) fn unpark(&self) {
        #[cfg(feature = "test-util")]
        self.inner
            .did_wake
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

cfg_not_rt! {
    impl Handle {
        /// Tries to get a handle to the current timer.
        ///
        /// # Panics
        ///
        /// This function panics if there is no current timer set.
        ///
        /// It can be triggered when [`Builder::enable_time`] or
        /// [`Builder::enable_all`] are not included in the builder.
        ///
        /// It can also panic whenever a timer is created outside of a
        /// Tokio runtime. That is why `rt.block_on(sleep(...))` will panic,
        /// since the function is executed outside of the runtime.
        /// Whereas `rt.block_on(async {sleep(...).await})` doesn't panic.
        /// And this is because wrapping the function on an async makes it lazy,
        /// and so gets executed inside the runtime successfully without
        /// panicking.
        ///
        /// [`Builder::enable_time`]: crate::custom_tokio::runtime::Builder::enable_time
        /// [`Builder::enable_all`]: crate::custom_tokio::runtime::Builder::enable_all
        #[track_caller]
        pub(crate) fn current() -> Self {
            panic!("{}", crate::custom_tokio::util::error::CONTEXT_MISSING_ERROR)
        }
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle")
    }
}
