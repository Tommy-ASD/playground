//! Utilities for tracking time.
//!
//! This module provides a number of types for executing code after a set period
//! of time.
//!
//! * [`Sleep`] is a future that does no work and completes at a specific [`Instant`]
//!   in time.
//!
//! * [`Interval`] is a stream yielding a value at a fixed period. It is
//!   initialized with a [`Duration`] and repeatedly yields each time the duration
//!   elapses.
//!
//! * [`Timeout`]: Wraps a future or stream, setting an upper bound to the amount
//!   of time it is allowed to execute. If the future or stream does not
//!   complete in time, then it is canceled and an error is returned.
//!
//! These types are sufficient for handling a large number of scenarios
//! involving time.
//!
//! These types must be used from within the context of the [`Runtime`](crate::custom_tokio::runtime::Runtime).
//!
//! # Examples
//!
//! Wait 100ms and print "100 ms have elapsed"
//!
//! ```
//! use std::time::Duration;
//! use crate::custom_tokio::time::sleep;
//!
//! #[crate::custom_tokio::main]
//! async fn main() {
//!     sleep(Duration::from_millis(100)).await;
//!     println!("100 ms have elapsed");
//! }
//! ```
//!
//! Require that an operation takes no more than 1s.
//!
//! ```
//! use crate::custom_tokio::time::{timeout, Duration};
//!
//! async fn long_future() {
//!     // do work here
//! }
//!
//! # async fn dox() {
//! let res = timeout(Duration::from_secs(1), long_future()).await;
//!
//! if res.is_err() {
//!     println!("operation timed out");
//! }
//! # }
//! ```
//!
//! A simple example using [`interval`] to execute a task every two seconds.
//!
//! The difference between [`interval`] and [`sleep`] is that an [`interval`]
//! measures the time since the last tick, which means that `.tick().await` may
//! wait for a shorter time than the duration specified for the interval
//! if some time has passed between calls to `.tick().await`.
//!
//! If the tick in the example below was replaced with [`sleep`], the task
//! would only be executed once every three seconds, and not every two
//! seconds.
//!
//! ```
//! use crate::custom_tokio::time;
//!
//! async fn task_that_takes_a_second() {
//!     println!("hello");
//!     time::sleep(time::Duration::from_secs(1)).await
//! }
//!
//! #[crate::custom_tokio::main]
//! async fn main() {
//!     let mut interval = time::interval(time::Duration::from_secs(2));
//!     for _i in 0..5 {
//!         interval.tick().await;
//!         task_that_takes_a_second().await;
//!     }
//! }
//! ```
//!
//! [`interval`]: crate::custom_tokio::time::interval()
//! [`sleep`]: sleep()

mod clock;
pub(crate) use self::clock::Clock;
pub(crate) use clock::{advance, pause, resume};

pub(crate) mod error;

mod instant;
pub(crate) use self::instant::Instant;

mod interval;
pub(crate) use interval::{interval, interval_at, Interval, MissedTickBehavior};

mod sleep;
pub(crate) use sleep::{sleep, sleep_until, Sleep};

mod timeout;
#[doc(inline)]
pub(crate) use timeout::{timeout, timeout_at, Timeout};

// Re-export for convenience
#[doc(no_inline)]
pub(crate) use std::time::Duration;
