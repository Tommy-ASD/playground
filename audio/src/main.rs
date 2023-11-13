use rodio::{Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::thread;

#[tokio::main]
async fn main() {
    println!("Starting");
    infinite_stream().await;

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    println!("Made stream");

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("./test.mp3").unwrap());
    println!("Read file to BufReader");

    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    println!("Decoded file");

    use rodio::Sink;
    use std::time::Duration;

    let sink = Sink::try_new(&stream_handle).unwrap();
    println!("Made sink");

    // Add a dummy source of the sake of the example.
    // let source = SineWave::new(440.0)
    //     .take_duration(Duration::from_secs_f32(0.25))
    //     .amplify(0.20);
    println!("Added sound to sink");

    input!("Continue");
    sink.append(source);
    input!("Continue");
    thread::sleep(Duration::from_secs(5));
    sink.pause();
    input!("Continue");
    sink.play();

    println!("Volume: {vol}", vol = sink.volume());

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
    input!("Continue");
    sink.skip_one();
    sink.len();
    sink.pause();
}

pub fn input_inner() -> String {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => {}
        Err(e) => {
            println!("Error reading line: {}", e);
            println!("Please try again");
            return input_inner();
        }
    }
    line.trim().to_string()
}

#[macro_export]
macro_rules! input {
    ($($arg:expr),*) => {{
        $(print!("{} ", $arg);)* // Print each argument followed by a space
        println!(); // Print a newline at the end

        $crate::input_inner()
    }};
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
        .with_env_filter(EnvFilter::default().add_directive(LevelFilter::INFO.into()))
        .with_line_number(true)
        .with_file(true)
        .init();

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
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
    sink.set_volume(0.1);
    sink.append(rodio::Decoder::new(reader).unwrap());

    let handle = tokio::task::spawn(async move {
        sink.sleep_until_end();
    });
    handle.await.unwrap();
}
