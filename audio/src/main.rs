#![windows_subsystem = "windows"]

use rodio::decoder::DecoderError;
use rodio::{Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::thread;

#[tokio::main]
async fn main() {
    println!("Starting");
    infinite_stream().await;
    loop {}
}

use stream_download::http::reqwest::Client;
use stream_download::http::HttpStream;
use stream_download::source::SourceStream;
use stream_download::storage::bounded::BoundedStorageProvider;
use stream_download::storage::memory::MemoryStorageProvider;
use stream_download::{Settings, StreamDownload};
use tracing::info;
use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;

use std::num::NonZeroUsize;

async fn infinite_stream() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::default().add_directive(LevelFilter::DEBUG.into()))
        .with_line_number(true)
        .with_file(true)
        .init();

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    sink.set_volume(0.1);

    let mut streaming = false;
    while !streaming {
        let stream = HttpStream::<Client>::create(
            "https://us2.internet-radio.com/proxy/mattjohnsonradio?mp=/stream"
                .parse()
                .unwrap(),
        )
        .await
        .unwrap();

        info!("content type={:?}", stream.content_type());
        let bitrate: u64 = stream.header("Icy-Br").unwrap().parse().unwrap();
        info!("bitrate={bitrate}");

        // buffer 5 seconds of audio
        // bitrate (in kilobits) / bits per byte * bytes per kilobyte * 5 seconds
        let prefetch_bytes = bitrate / 8 * 1024 * 5;
        info!("prefetch bytes={prefetch_bytes}");
        info!("bitrate={bitrate}kb");

        let reader = StreamDownload::from_stream(
            stream,
            // use bounded storage to keep the underlying size from growing indefinitely
            BoundedStorageProvider::new(
                MemoryStorageProvider {},
                // be liberal with the buffer size, you need to make sure it holds enough space to
                // prevent any out-of-bounds reads
                NonZeroUsize::new(512 * 1024).unwrap(),
            ),
            Settings::default().prefetch_bytes(prefetch_bytes),
        )
        .await
        .unwrap();
        let decoded = match rodio::Decoder::new(reader) {
            Err(DecoderError::UnrecognizedFormat) => {
                continue;
            }
            Ok(decoded) => decoded,
            Err(_) => panic!(),
        };
        sink.append(decoded);
        streaming = true;
    }

    let handle = tokio::task::spawn(async move {
        sink.sleep_until_end();
    });
    handle.await.unwrap();
}
