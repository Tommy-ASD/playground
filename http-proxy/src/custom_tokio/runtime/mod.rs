//! The Tokio runtime.
//!
//! Unlike other Rust programs, asynchronous applications require runtime
//! support. In particular, the following runtime services are necessary:
//!
//! * An **I/O event loop**, called the driver, which drives I/O resources and
//!   dispatches I/O events to tasks that depend on them.
//! * A **scheduler** to execute [tasks] that use these I/O resources.
//! * A **timer** for scheduling work to run after a set period of time.
//!
//! Tokio's [`Runtime`] bundles all of these services as a single type, allowing
//! them to be started, shut down, and configured together. However, often it is
//! not required to configure a [`Runtime`] manually, and a user may just use the
//! [`crate::custom_tokio::main`] attribute macro, which creates a [`Runtime`] under the hood.
//!
//! # Usage
//!
//! When no fine tuning is required, the [`crate::custom_tokio::main`] attribute macro can be
//! used.
//!
//! ```no_run
//! use crate::custom_tokio::net::TcpListener;
//! use crate::custom_tokio::io::{AsyncReadExt, AsyncWriteExt};
//!
//! #[crate::custom_tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let listener = TcpListener::bind("127.0.0.1:8080").await?;
//!
//!     loop {
//!         let (mut socket, _) = listener.accept().await?;
//!
//!         crate::custom_tokio::spawn(async move {
//!             let mut buf = [0; 1024];
//!
//!             // In a loop, read data from the socket and write the data back.
//!             loop {
//!                 let n = match socket.read(&mut buf).await {
//!                     // socket closed
//!                     Ok(n) if n == 0 => return,
//!                     Ok(n) => n,
//!                     Err(e) => {
//!                         println!("failed to read from socket; err = {:?}", e);
//!                         return;
//!                     }
//!                 };
//!
//!                 // Write the data back
//!                 if let Err(e) = socket.write_all(&buf[0..n]).await {
//!                     println!("failed to write to socket; err = {:?}", e);
//!                     return;
//!                 }
//!             }
//!         });
//!     }
//! }
//! ```
//!
//! From within the context of the runtime, additional tasks are spawned using
//! the [`crate::custom_tokio::spawn`] function. Futures spawned using this function will be
//! executed on the same thread pool used by the [`Runtime`].
//!
//! A [`Runtime`] instance can also be used directly.
//!
//! ```no_run
//! use crate::custom_tokio::net::TcpListener;
//! use crate::custom_tokio::io::{AsyncReadExt, AsyncWriteExt};
//! use crate::custom_tokio::runtime::Runtime;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create the runtime
//!     let rt  = Runtime::new()?;
//!
//!     // Spawn the root task
//!     rt.block_on(async {
//!         let listener = TcpListener::bind("127.0.0.1:8080").await?;
//!
//!         loop {
//!             let (mut socket, _) = listener.accept().await?;
//!
//!             crate::custom_tokio::spawn(async move {
//!                 let mut buf = [0; 1024];
//!
//!                 // In a loop, read data from the socket and write the data back.
//!                 loop {
//!                     let n = match socket.read(&mut buf).await {
//!                         // socket closed
//!                         Ok(n) if n == 0 => return,
//!                         Ok(n) => n,
//!                         Err(e) => {
//!                             println!("failed to read from socket; err = {:?}", e);
//!                             return;
//!                         }
//!                     };
//!
//!                     // Write the data back
//!                     if let Err(e) = socket.write_all(&buf[0..n]).await {
//!                         println!("failed to write to socket; err = {:?}", e);
//!                         return;
//!                     }
//!                 }
//!             });
//!         }
//!     })
//! }
//! ```
//!
//! ## Runtime Configurations
//!
//! Tokio provides multiple task scheduling strategies, suitable for different
//! applications. The [runtime builder] or `#[crate::custom_tokio::main]` attribute may be
//! used to select which scheduler to use.
//!
//! #### Multi-Thread Scheduler
//!
//! The multi-thread scheduler executes futures on a _thread pool_, using a
//! work-stealing strategy. By default, it will start a worker thread for each
//! CPU core available on the system. This tends to be the ideal configuration
//! for most applications. The multi-thread scheduler requires the `rt-multi-thread`
//! feature flag, and is selected by default:
//! ```
//! use crate::custom_tokio::runtime;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let threaded_rt = runtime::Runtime::new()?;
//! # Ok(()) }
//! ```
//!
//! Most applications should use the multi-thread scheduler, except in some
//! niche use-cases, such as when running only a single thread is required.
//!
//! #### Current-Thread Scheduler
//!
//! The current-thread scheduler provides a _single-threaded_ future executor.
//! All tasks will be created and executed on the current thread. This requires
//! the `rt` feature flag.
//! ```
//! use crate::custom_tokio::runtime;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let rt = runtime::Builder::new_current_thread()
//!     .build()?;
//! # Ok(()) }
//! ```
//!
//! #### Resource drivers
//!
//! When configuring a runtime by hand, no resource drivers are enabled by
//! default. In this case, attempting to use networking types or time types will
//! fail. In order to enable these types, the resource drivers must be enabled.
//! This is done with [`Builder::enable_io`] and [`Builder::enable_time`]. As a
//! shorthand, [`Builder::enable_all`] enables both resource drivers.
//!
//! ## Lifetime of spawned threads
//!
//! The runtime may spawn threads depending on its configuration and usage. The
//! multi-thread scheduler spawns threads to schedule tasks and for `spawn_blocking`
//! calls.
//!
//! While the `Runtime` is active, threads may shut down after periods of being
//! idle. Once `Runtime` is dropped, all runtime threads have usually been
//! terminated, but in the presence of unstoppable spawned work are not
//! guaranteed to have been terminated. See the
//! [struct level documentation](Runtime#shutdown) for more details.
//!
//! [tasks]: crate::custom_tokio::task
//! [`Runtime`]: Runtime
//! [`crate::custom_tokio::spawn`]: crate::custom_tokio::spawn
//! [`crate::custom_tokio::main`]: ../attr.main.html
//! [runtime builder]: crate::custom_tokio::runtime::Builder
//! [`Runtime::new`]: crate::custom_tokio::runtime::Runtime::new
//! [`Builder::threaded_scheduler`]: crate::custom_tokio::runtime::Builder::threaded_scheduler
//! [`Builder::enable_io`]: crate::custom_tokio::runtime::Builder::enable_io
//! [`Builder::enable_time`]: crate::custom_tokio::runtime::Builder::enable_time
//! [`Builder::enable_all`]: crate::custom_tokio::runtime::Builder::enable_all

// At the top due to macros
#[cfg(test)]
#[cfg(not(target_family = "wasm"))]
#[macro_use]
mod tests;

pub(crate) mod context;

pub(crate) mod coop;

pub(crate) mod park;

mod driver;

pub(crate) mod scheduler;

cfg_io_driver_impl! {
    pub(crate) mod io;
}

cfg_process_driver! {
    mod process;
}

cfg_time! {
    pub(crate) mod time;
}

cfg_signal_internal_and_unix! {
    pub(crate) mod signal;
}

cfg_rt! {
    pub(crate) mod task;

    mod config;
    use config::Config;

    mod blocking;
    #[cfg_attr(target_os = "wasi", allow(unused_imports))]
    pub(crate) use blocking::spawn_blocking;

    cfg_trace! {
        pub(crate) use blocking::Mandatory;
    }

    cfg_fs! {
        pub(crate) use blocking::spawn_mandatory_blocking;
    }

    mod builder;
    pub(crate)use self::builder::Builder;
    cfg_unstable! {
        mod id;
        #[cfg_attr(not(tokio_unstable), allow(unreachable_pub))]
        pub(crate)use id::Id;

        pub(crate)use self::builder::UnhandledPanic;
        pub(crate)use crate::custom_tokio::util::rand::RngSeed;
    }

    cfg_taskdump! {
        pub(crate)mod dump;
        pub(crate)use dump::Dump;
    }

    mod handle;
    pub(crate)use handle::{EnterGuard, Handle, TryCurrentError};

    mod runtime;
    pub(crate)use runtime::{Runtime, RuntimeFlavor};

    mod thread_id;
    pub(crate) use thread_id::ThreadId;

    cfg_metrics! {
        mod metrics;
        pub(crate)use metrics::{RuntimeMetrics, HistogramScale};

        pub(crate) use metrics::{MetricsBatch, SchedulerMetrics, WorkerMetrics, HistogramBuilder};

        cfg_net! {
        pub(crate) use metrics::IoDriverMetrics;
        }
    }

    cfg_not_metrics! {
        pub(crate) mod metrics;
        pub(crate) use metrics::{SchedulerMetrics, WorkerMetrics, MetricsBatch, HistogramBuilder};
    }

    /// After thread starts / before thread stops
    type Callback = std::sync::Arc<dyn Fn() + Send + Sync>;
}
