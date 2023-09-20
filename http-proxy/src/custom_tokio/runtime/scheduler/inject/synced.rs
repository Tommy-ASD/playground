#![cfg_attr(
    any(not(all(tokio_unstable, feature = "full")), target_family = "wasm"),
    allow(dead_code)
)]

use crate::custom_tokio::runtime::task;

pub(crate) struct Synced {
    /// True if the queue is closed.
    pub(crate) is_closed: bool,

    /// Linked-list head.
    pub(crate) head: Option<task::RawTask>,

    /// Linked-list tail.
    pub(crate) tail: Option<task::RawTask>,
}

unsafe impl Send for Synced {}
unsafe impl Sync for Synced {}

impl Synced {
    pub(crate) fn pop<T: 'static>(&mut self) -> Option<task::Notified<T>> {
        let task = self.head?;

        self.head = unsafe { task.get_queue_next() };

        if self.head.is_none() {
            self.tail = None;
        }

        unsafe { task.set_queue_next(None) };

        // safety: a `Notified` is pushed into the queue and now it is popped!
        Some(unsafe { task::Notified::from_raw(task) })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
