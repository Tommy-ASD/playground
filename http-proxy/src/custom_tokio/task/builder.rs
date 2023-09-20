#![allow(unreachable_pub)]
use crate::custom_tokio::{
    runtime::Handle,
    task::{JoinHandle, LocalSet},
};
use std::{future::Future, io};

/// Factory which is used to configure the properties of a new task.
///
/// **Note**: This is an [unstable API][unstable]. The public API of this type
/// may break in 1.x releases. See [the documentation on unstable
/// features][unstable] for details.
///
/// Methods can be chained in order to configure it.
///
/// Currently, there is only one configuration option:
///
/// - [`name`], which specifies an associated name for
///   the task
///
/// There are three types of task that can be spawned from a Builder:
/// - [`spawn_local`] for executing futures on the current thread
/// - [`spawn`] for executing [`Send`] futures on the runtime
/// - [`spawn_blocking`] for executing blocking code in the
///   blocking thread pool.
///
/// ## Example
///
/// ```no_run
/// use crate::custom_tokio::net::{TcpListener, TcpStream};
///
/// use std::io;
///
/// async fn process(socket: TcpStream) {
///     // ...
/// # drop(socket);
/// }
///
/// #[crate::custom_tokio::main]
/// async fn main() -> io::Result<()> {
///     let listener = TcpListener::bind("127.0.0.1:8080").await?;
///
///     loop {
///         let (socket, _) = listener.accept().await?;
///
///         crate::custom_tokio::task::Builder::new()
///             .name("tcp connection handler")
///             .spawn(async move {
///                 // Process each socket concurrently.
///                 process(socket).await
///             })?;
///     }
/// }
/// ```
/// [unstable]: crate#unstable-features
/// [`name`]: Builder::name
/// [`spawn_local`]: Builder::spawn_local
/// [`spawn`]: Builder::spawn
/// [`spawn_blocking`]: Builder::spawn_blocking
#[derive(Default, Debug)]
#[cfg_attr(docsrs, doc(cfg(all(tokio_unstable, feature = "tracing"))))]
pub(crate) struct Builder<'a> {
    name: Option<&'a str>,
}

impl<'a> Builder<'a> {
    /// Creates a new task builder.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Assigns a name to the task which will be spawned.
    pub(crate) fn name(&self, name: &'a str) -> Self {
        Self { name: Some(name) }
    }

    /// Spawns a task with this builder's settings on the current runtime.
    ///
    /// # Panics
    ///
    /// This method panics if called outside of a Tokio runtime.
    ///
    /// See [`task::spawn`](crate::custom_tokio::task::spawn()) for
    /// more details.
    #[track_caller]
    pub(crate) fn spawn<Fut>(self, future: Fut) -> io::Result<JoinHandle<Fut::Output>>
    where
        Fut: Future + Send + 'static,
        Fut::Output: Send + 'static,
    {
        Ok(super::spawn::spawn_inner(future, self.name))
    }

    /// Spawn a task with this builder's settings on the provided [runtime
    /// handle].
    ///
    /// See [`Handle::spawn`] for more details.
    ///
    /// [runtime handle]: crate::custom_tokio::runtime::Handle
    /// [`Handle::spawn`]: crate::custom_tokio::runtime::Handle::spawn
    #[track_caller]
    pub(crate) fn spawn_on<Fut>(
        self,
        future: Fut,
        handle: &Handle,
    ) -> io::Result<JoinHandle<Fut::Output>>
    where
        Fut: Future + Send + 'static,
        Fut::Output: Send + 'static,
    {
        Ok(handle.spawn_named(future, self.name))
    }

    /// Spawns `!Send` a task on the current [`LocalSet`] with this builder's
    /// settings.
    ///
    /// The spawned future will be run on the same thread that called `spawn_local`.
    /// This may only be called from the context of a [local task set][`LocalSet`].
    ///
    /// # Panics
    ///
    /// This function panics if called outside of a [local task set][`LocalSet`].
    ///
    /// See [`task::spawn_local`] for more details.
    ///
    /// [`task::spawn_local`]: crate::custom_tokio::task::spawn_local
    /// [`LocalSet`]: crate::custom_tokio::task::LocalSet
    #[track_caller]
    pub(crate) fn spawn_local<Fut>(self, future: Fut) -> io::Result<JoinHandle<Fut::Output>>
    where
        Fut: Future + 'static,
        Fut::Output: 'static,
    {
        Ok(super::local::spawn_local_inner(future, self.name))
    }

    /// Spawns `!Send` a task on the provided [`LocalSet`] with this builder's
    /// settings.
    ///
    /// See [`LocalSet::spawn_local`] for more details.
    ///
    /// [`LocalSet::spawn_local`]: crate::custom_tokio::task::LocalSet::spawn_local
    /// [`LocalSet`]: crate::custom_tokio::task::LocalSet
    #[track_caller]
    pub(crate) fn spawn_local_on<Fut>(
        self,
        future: Fut,
        local_set: &LocalSet,
    ) -> io::Result<JoinHandle<Fut::Output>>
    where
        Fut: Future + 'static,
        Fut::Output: 'static,
    {
        Ok(local_set.spawn_named(future, self.name))
    }

    /// Spawns blocking code on the blocking threadpool.
    ///
    /// # Panics
    ///
    /// This method panics if called outside of a Tokio runtime.
    ///
    /// See [`task::spawn_blocking`](crate::custom_tokio::task::spawn_blocking)
    /// for more details.
    #[track_caller]
    pub(crate) fn spawn_blocking<Function, Output>(
        self,
        function: Function,
    ) -> io::Result<JoinHandle<Output>>
    where
        Function: FnOnce() -> Output + Send + 'static,
        Output: Send + 'static,
    {
        let handle = Handle::current();
        self.spawn_blocking_on(function, &handle)
    }

    /// Spawns blocking code on the provided [runtime handle]'s blocking threadpool.
    ///
    /// See [`Handle::spawn_blocking`] for more details.
    ///
    /// [runtime handle]: crate::custom_tokio::runtime::Handle
    /// [`Handle::spawn_blocking`]: crate::custom_tokio::runtime::Handle::spawn_blocking
    #[track_caller]
    pub(crate) fn spawn_blocking_on<Function, Output>(
        self,
        function: Function,
        handle: &Handle,
    ) -> io::Result<JoinHandle<Output>>
    where
        Function: FnOnce() -> Output + Send + 'static,
        Output: Send + 'static,
    {
        use crate::custom_tokio::runtime::Mandatory;
        let (join_handle, spawn_result) = handle.inner.blocking_spawner().spawn_blocking_inner(
            function,
            Mandatory::NonMandatory,
            self.name,
            handle,
        );

        spawn_result?;
        Ok(join_handle)
    }
}
