use crate::custom_tokio::io::util::read_until::read_until_internal;
use crate::custom_tokio::io::AsyncBufRead;

use pin_project_lite::pin_project;
use std::io;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    /// Splitter for the [`split`](crate::custom_tokio::io::AsyncBufReadExt::split) method.
    ///
    /// A `Split` can be turned into a `Stream` with [`SplitStream`].
    ///
    /// [`SplitStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.SplitStream.html
    #[derive(Debug)]
    #[must_use = "streams do nothing unless polled"]
    #[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
    pub(crate)struct Split<R> {
        #[pin]
        reader: R,
        buf: Vec<u8>,
        delim: u8,
        read: usize,
    }
}

pub(crate) fn split<R>(reader: R, delim: u8) -> Split<R>
where
    R: AsyncBufRead,
{
    Split {
        reader,
        buf: Vec::new(),
        delim,
        read: 0,
    }
}

impl<R> Split<R>
where
    R: AsyncBufRead + Unpin,
{
    /// Returns the next segment in the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::custom_tokio::io::AsyncBufRead;
    /// use crate::custom_tokio::io::AsyncBufReadExt;
    ///
    /// # async fn dox(my_buf_read: impl AsyncBufRead + Unpin) -> std::io::Result<()> {
    /// let mut segments = my_buf_read.split(b'f');
    ///
    /// while let Some(segment) = segments.next_segment().await? {
    ///     println!("length = {}", segment.len())
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub(crate) async fn next_segment(&mut self) -> io::Result<Option<Vec<u8>>> {
        use crate::custom_tokio::future::poll_fn;

        poll_fn(|cx| Pin::new(&mut *self).poll_next_segment(cx)).await
    }
}

impl<R> Split<R>
where
    R: AsyncBufRead,
{
    /// Polls for the next segment in the stream.
    ///
    /// This method returns:
    ///
    ///  * `Poll::Pending` if the next segment is not yet available.
    ///  * `Poll::Ready(Ok(Some(segment)))` if the next segment is available.
    ///  * `Poll::Ready(Ok(None))` if there are no more segments in this stream.
    ///  * `Poll::Ready(Err(err))` if an IO error occurred while reading the
    ///    next segment.
    ///
    /// When the method returns `Poll::Pending`, the `Waker` in the provided
    /// `Context` is scheduled to receive a wakeup when more bytes become
    /// available on the underlying IO resource.
    ///
    /// Note that on multiple calls to `poll_next_segment`, only the `Waker`
    /// from the `Context` passed to the most recent call is scheduled to
    /// receive a wakeup.
    pub(crate) fn poll_next_segment(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<Option<Vec<u8>>>> {
        let me = self.project();

        let n = ready!(read_until_internal(
            me.reader, cx, *me.delim, me.buf, me.read,
        ))?;
        // read_until_internal resets me.read to zero once it finds the delimiter
        debug_assert_eq!(*me.read, 0);

        if n == 0 && me.buf.is_empty() {
            return Poll::Ready(Ok(None));
        }

        if me.buf.last() == Some(me.delim) {
            me.buf.pop();
        }

        Poll::Ready(Ok(Some(mem::take(me.buf))))
    }
}
