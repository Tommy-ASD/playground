use std::task::Poll;

/// Consumes a unit of budget and returns the execution back to the Tokio
/// runtime *if* the task's coop budget was exhausted.
///
/// The task will only yield if its entire coop budget has been exhausted.
/// This function can be used in order to insert optional yield points into long
/// computations that do not use Tokio resources like sockets or semaphores,
/// without redundantly yielding to the runtime each time.
///
/// **Note**: This is an [unstable API][unstable]. The public API of this type
/// may break in 1.x releases. See [the documentation on unstable
/// features][unstable] for details.
///
/// # Examples
///
/// Make sure that a function which returns a sum of (potentially lots of)
/// iterated values is cooperative.
///
/// ```
/// async fn sum_iterator(input: &mut impl std::iter::Iterator<Item=i64>) -> i64 {
///     let mut sum: i64 = 0;
///     while let Some(i) = input.next() {
///         sum += i;
///         crate::custom_tokio::task::consume_budget().await
///     }
///     sum
/// }
/// ```
/// [unstable]: crate#unstable-features
#[cfg_attr(docsrs, doc(cfg(all(tokio_unstable, feature = "rt"))))]
pub(crate) async fn consume_budget() {
    let mut status = Poll::Pending;

    crate::custom_tokio::future::poll_fn(move |cx| {
        ready!(crate::custom_tokio::trace::trace_leaf(cx));
        if status.is_ready() {
            return status;
        }
        status = crate::custom_tokio::runtime::coop::poll_proceed(cx).map(|restore| {
            restore.made_progress();
        });
        status
    })
    .await
}
