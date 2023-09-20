use crate::custom_tokio::loom::sync::atomic::{AtomicBool, Ordering};
use crate::custom_tokio::loom::sync::{Barrier, Mutex};
use crate::custom_tokio::runtime::dump::Dump;
use crate::custom_tokio::runtime::scheduler::multi_thread::Handle;
use crate::custom_tokio::sync::notify::Notify;

/// Tracing status of the worker.
pub(crate) struct TraceStatus {
    pub(crate) trace_requested: AtomicBool,
    pub(crate) trace_start: Barrier,
    pub(crate) trace_end: Barrier,
    pub(crate) result_ready: Notify,
    pub(crate) trace_result: Mutex<Option<Dump>>,
}

impl TraceStatus {
    pub(crate) fn new(remotes_len: usize) -> Self {
        Self {
            trace_requested: AtomicBool::new(false),
            trace_start: Barrier::new(remotes_len),
            trace_end: Barrier::new(remotes_len),
            result_ready: Notify::new(),
            trace_result: Mutex::new(None),
        }
    }

    pub(crate) fn trace_requested(&self) -> bool {
        self.trace_requested.load(Ordering::Relaxed)
    }

    pub(crate) async fn start_trace_request(&self, handle: &Handle) {
        while self
            .trace_requested
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            handle.notify_all();
            crate::custom_tokio::task::yield_now().await;
        }
    }

    pub(crate) fn stash_result(&self, dump: Dump) {
        let _ = self.trace_result.lock().insert(dump);
        self.result_ready.notify_one();
    }

    pub(crate) fn take_result(&self) -> Option<Dump> {
        self.trace_result.lock().take()
    }

    pub(crate) async fn end_trace_request(&self, handle: &Handle) {
        while self
            .trace_requested
            .compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            handle.notify_all();
            crate::custom_tokio::task::yield_now().await;
        }
    }
}
