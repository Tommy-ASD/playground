cfg_if::cfg_if! {
    if #[cfg(feature = "wgpu-backend")] {
        type Backend = WgpuBackend<AutoGraphicsApi, f32, i32>;
    } else if #[cfg(feature = "torch-backend")] {
        type Backend = TchBackend<f32>;
    }
}

//// POSSSIBLE SOLUTION :D
/// when the program starts, start a recording running in the background
/// recording will abruptly end when the program ends
///
///  
#[tokio::main]
async fn main() {
    let model_name = "tiny_en";

    let whisper = Arc::new(load_whisper(model_name));

    dbg!();

    let mut interval = interval(Duration::from_secs(2));

    let (task_send, mut task_recv) = tokio::sync::mpsc::channel(2305843009213693951);

    let task_send = Arc::new(task_send);

    let mut transcribed = vec![];

    tokio::spawn(async move {
        while let Some(next) = task_recv.recv().await {
            let (text, _tokens) = tokio::spawn(next).await.unwrap();

            transcribed.push(text);

            println!("{transcribed:#?}");
        }
    });

    let mut state = TranscribeStateForDebugging {
        idx: 0,
        running_since: std::time::Instant::now(),
    };

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let state_clone = state.clone();
                let whisper_clone = Arc::clone(&whisper);
                let task_send_clone = Arc::clone(&task_send);

                // Spawn a new Tokio task in a separate thread
                // Your task logic goes here
                // println!("Hi :D");
                tokio::spawn(async move {
                    let dest = record(&state_clone).await;
                    task_send_clone.send(transcribe_and_remove(dest, whisper_clone, state_clone)).await.unwrap();
                });
                state.idx += 1;
            }
        }
    }
}

async fn record(_state: &TranscribeStateForDebugging) -> String {
    let bitrate = 16000;
    let duration = 10; // Duration in seconds
    let channels = 1;
    let id = uuid::Uuid::new_v4();
    let dest = &format!("{id}.wav");

    tokio::process::Command::new("arecord")
        .args([
            "-f",
            "cd",
            "-t",
            "wav",
            "-D",
            "default",
            "-r",
            &format!("{bitrate}"),
            "-d",
            &format!("{duration}"),
            "-c",
            &format!("{channels}"),
            "-i",
            dest,
        ])
        .output()
        .await
        .unwrap();
    return dest.to_string();
}

async fn transcribe_and_remove(
    dest: String,
    whisper: Arc<Whisper<Backend>>,
    state: TranscribeStateForDebugging,
) -> (String, Vec<usize>) {
    let result = transcribe(&dest, Arc::clone(&whisper), state).await;

    tokio::fs::remove_file(dest).await.unwrap();

    result
}

use tokio::time::interval;
use whisper::token::Language;
use whisper::transcribe::waveform_to_text;
use whisper::{model::*, transcribe::TranscribeStateForDebugging};

use strum::IntoEnumIterator;

cfg_if::cfg_if! {
    if #[cfg(feature = "wgpu-backend")] {
        use burn_wgpu::{WgpuBackend, WgpuDevice, AutoGraphicsApi};
    } else if #[cfg(feature = "torch-backend")] {
        use burn_tch::{TchBackend, TchDevice};
    }
}

use burn::{config::Config, module::Module};

use hound::{self, SampleFormat};

fn load_audio_waveform<B: burn::tensor::backend::Backend>(
    filename: &str,
) -> hound::Result<(Vec<f32>, usize)> {
    let reader = hound::WavReader::open(filename)?;
    let spec = reader.spec();

    let _duration = reader.duration() as usize;
    let channels = spec.channels as usize;
    let sample_rate = spec.sample_rate as usize;
    let _bits_per_sample = spec.bits_per_sample;
    let sample_format = spec.sample_format;

    assert_eq!(sample_rate, 16000, "The audio sample rate must be 16k.");
    assert_eq!(channels, 1, "The audio must be single-channel.");

    let max_int_val = 2_u32.pow(spec.bits_per_sample as u32 - 1) - 1;

    let floats = match sample_format {
        SampleFormat::Float => reader.into_samples::<f32>().collect::<hound::Result<_>>()?,
        SampleFormat::Int => reader
            .into_samples::<i32>()
            .map(|s| s.map(|s| s as f32 / max_int_val as f32))
            .collect::<hound::Result<_>>()?,
    };

    return Ok((floats, sample_rate));
}

use whisper::token::Gpt2Tokenizer;

use burn::record::{DefaultRecorder, Recorder, RecorderError};

fn load_whisper_model_file<B: burn::tensor::backend::Backend>(
    config: &WhisperConfig,
    filename: &str,
) -> Result<Whisper<B>, RecorderError> {
    DefaultRecorder::new()
        .load(filename.into())
        .map(|record| config.init().load_record(record))
}

use std::{fs, process, sync::Arc, time::Duration};

async fn transcribe(
    wav_file: &str,
    model: Arc<Whisper<Backend>>,
    state: TranscribeStateForDebugging,
) -> (String, Vec<usize>) {
    let lang_str = "en";
    let text_file = "transcript.txt";

    let lang = match Language::iter().find(|lang| lang.as_alpha2() == lang_str) {
        Some(lang) => lang,
        None => {
            eprintln!("Invalid language abbreviation: {}", lang_str);
            process::exit(1);
        }
    };

    // println!("Loading waveform...");
    let (waveform, sample_rate) = match load_audio_waveform::<Backend>(wav_file) {
        Ok((w, sr)) => (w, sr),
        Err(e) => {
            eprintln!("Failed to load audio file: {}", e);
            process::exit(1);
        }
    };

    let bpe = match Gpt2Tokenizer::new() {
        Ok(bpe) => bpe,
        Err(e) => {
            eprintln!("Failed to load tokenizer: {}", e);
            process::exit(1);
        }
    };

    let state_clone = state.clone();

    let (text, tokens) = tokio::task::spawn_blocking(move || {
        match waveform_to_text(
            &model.as_ref(),
            &bpe,
            lang,
            waveform,
            sample_rate,
            state_clone,
        ) {
            Ok((text, tokens)) => (text, tokens),
            Err(e) => {
                eprintln!("Error during transcription: {}", e);
                process::exit(1);
            }
        }
    })
    .await
    .unwrap();

    fs::write(text_file, text.to_string()).unwrap_or_else(|e| {
        eprintln!("Error writing transcription file: {}", e);
        process::exit(1);
    });

    (text, tokens)

    // println!("Transcription finished.");
}

fn load_whisper(model_name: &str) -> whisper::model::Whisper<TchBackend<f32>> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "wgpu-backend")] {
            let device = WgpuDevice::BestAvailable;
        } else if #[cfg(feature = "torch-backend")] {
            let device = TchDevice::Cpu;
        }
    }

    let whisper_config = match WhisperConfig::load(&format!("{}.cfg", model_name)) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load whisper config: {}", e);
            process::exit(1);
        }
    };

    println!("Loading model...");
    let whisper: Whisper<Backend> = match load_whisper_model_file(&whisper_config, model_name) {
        Ok(whisper_model) => whisper_model,
        Err(e) => {
            eprintln!("Failed to load whisper model file: {}", e);
            process::exit(1);
        }
    };

    let whisper: Whisper<_> = whisper.to_device(&device);

    whisper
}
