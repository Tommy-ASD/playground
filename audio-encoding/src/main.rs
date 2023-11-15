use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{traits::SupportedStreamConfigRange, Sample};
use hound;
use lame::Lame;
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), anyhow::Error> {
    let host = cpal::default_host();

    // Set up the input device and stream with the default input config.
    let device = host
        .default_input_device()
        .expect("failed to find input device");

    let config = device
        .default_input_config()
        .expect("Failed to get default input config");

    println!("Input device: {}", device.name()?);
    println!("Default input config: {:?}", config);

    // The MP3 file we're recording to.
    const PATH: &str = "./recorded.mp3";

    // Set up lame encoder
    let mp3_writer = BufWriter::new(File::create(PATH)?);
    let lame = Lame::new()
        .sample_rate(config.sample_rate().0 as _)
        .channels(config.channels() as _)
        .bitrate(128) // Adjust the bitrate as needed
        .write_id3v2(false)
        .build(mp3_writer)?;

    let writer = Arc::new(Mutex::new(Some(lame)));

    // A flag to indicate that recording is in progress.
    println!("Begin recording...");

    // Run the input stream on a separate thread.
    let writer_2 = writer.clone();

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i16>(data, &writer_2),
            err_fn,
            None,
        )?,
        sample_format => {
            return Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{:?}' for MP3 encoding",
                sample_format
            )))
        }
    };

    stream.play()?;

    // Let recording go for roughly three seconds.
    std::thread::sleep(std::time::Duration::from_secs(3));
    drop(stream);
    writer.lock().unwrap().take().unwrap().finalize()?;
    println!("Recording {} complete!", PATH);
    Ok(())
}

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    if format.is_float() {
        hound::SampleFormat::Float
    } else {
        hound::SampleFormat::Int
    }
}

type Mp3WriterHandle = Arc<Mutex<Option<Lame<BufWriter<File>>>>>;

fn write_input_data<T>(input: &[T], writer: &Mp3WriterHandle)
where
    T: Sample + hound::Sample + lame::Sample,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(encoder) = guard.as_mut() {
            for &sample in input.iter() {
                let sample = lame::Sample::from_i16(sample.to_i16());
                encoder.encode_sample(sample).ok();
            }
        }
    }
}
