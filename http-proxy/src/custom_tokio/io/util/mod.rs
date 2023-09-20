#![allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411

cfg_io_util! {
    pub(crate)mod async_buf_read_ext;
    pub(crate)use async_buf_read_ext::AsyncBufReadExt;

    pub(crate)mod async_read_ext;
    pub(crate)use async_read_ext::AsyncReadExt;

    pub(crate)mod async_seek_ext;
    pub(crate)use async_seek_ext::AsyncSeekExt;

    pub(crate)mod async_write_ext;
    pub(crate)use async_write_ext::AsyncWriteExt;

    pub(crate)mod buf_reader;
    pub(crate)use buf_reader::BufReader;

    pub(crate)mod buf_stream;
    pub(crate)use buf_stream::BufStream;

    pub(crate)mod buf_writer;
    pub(crate)use buf_writer::BufWriter;

    pub(crate)mod chain;

    pub(crate)mod copy;
    pub(crate)use copy::copy;

    pub(crate)mod copy_bidirectional;
    pub(crate)use copy_bidirectional::copy_bidirectional;

    pub(crate)mod copy_buf;
    pub(crate)use copy_buf::copy_buf;

    pub(crate)mod empty;
    pub(crate)use empty::{empty, Empty};

    pub(crate)mod flush;

    pub(crate)mod lines;
    pub(crate)use lines::Lines;

    pub(crate)mod mem;
    pub(crate)use mem::{duplex, DuplexStream};

    pub(crate)mod read;
    pub(crate)mod read_buf;
    pub(crate)mod read_exact;
    pub(crate)mod read_int;
    pub(crate)mod read_line;
    pub(crate)mod fill_buf;

    pub(crate)mod read_to_end;
    pub(crate)mod vec_with_initialized;
    cfg_process! {
        pub(crate) use read_to_end::read_to_end;
    }

    pub(crate)mod read_to_string;
    pub(crate)mod read_until;

    pub(crate)mod repeat;
    pub(crate)use repeat::{repeat, Repeat};

    pub(crate)mod shutdown;

    pub(crate)mod sink;
    pub(crate)use sink::{sink, Sink};

    pub(crate)mod split;
    pub(crate)use split::Split;

    pub(crate)mod take;
    pub(crate)use take::Take;

    pub(crate)mod write;
    pub(crate)mod write_vectored;
    pub(crate)mod write_all;
    pub(crate)mod write_buf;
    pub(crate)mod write_all_buf;
    pub(crate)mod write_int;


    // used by `BufReader` and `BufWriter`
    // https://github.com/rust-lang/rust/blob/master/library/std/src/sys_common/io.rs#L1
    pub(crate)const DEFAULT_BUF_SIZE: usize = 8 * 1024;
}
