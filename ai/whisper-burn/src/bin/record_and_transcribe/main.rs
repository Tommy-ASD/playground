fn main() {
    let bitrate = 16000;
    let duration = 10; // Duration in seconds
    let channels = 1;
    let dest = "recording.wav";

    let mut child = std::process::Command::new("arecord")
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
        .spawn()
        .unwrap();
    child.wait().unwrap();

    transcribe(dest);
}
use whisper::model::*;
use whisper::token::Language;
use whisper::transcribe::waveform_to_text;

use strum::IntoEnumIterator;

cfg_if::cfg_if! {
    if #[cfg(feature = "wgpu-backend")] {
        use burn_wgpu::{WgpuBackend, WgpuDevice, AutoGraphicsApi};
    } else if #[cfg(feature = "torch-backend")] {
        use burn_tch::{TchBackend, TchDevice};
    }
}

use burn::{config::Config, module::Module, tensor::backend::Backend};

use hound::{self, SampleFormat};

fn load_audio_waveform<B: Backend>(filename: &str) -> hound::Result<(Vec<f32>, usize)> {
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

fn load_whisper_model_file<B: Backend>(
    config: &WhisperConfig,
    filename: &str,
) -> Result<Whisper<B>, RecorderError> {
    DefaultRecorder::new()
        .load(filename.into())
        .map(|record| config.init().load_record(record))
}

use std::{env, fs, process};

fn transcribe(wav_file: &str) {
    cfg_if::cfg_if! {
        if #[cfg(feature = "wgpu-backend")] {
            type Backend = WgpuBackend<AutoGraphicsApi, f32, i32>;
        } else if #[cfg(feature = "torch-backend")] {
            type Backend = TchBackend<f32>;
        }
    }
    let model_name = "tiny_en";
    let lang_str = "en";
    let text_file = "transcript.txt";

    let lang = match Language::iter().find(|lang| lang.as_str() == lang_str) {
        Some(lang) => lang,
        None => {
            eprintln!("Invalid language abbreviation: {}", lang_str);
            process::exit(1);
        }
    };

    println!("Loading waveform...");
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

    let whisper = load_whisper(model_name);

    let (text, _tokens) = match waveform_to_text(&whisper, &bpe, lang, waveform, sample_rate) {
        Ok((text, tokens)) => (text, tokens),
        Err(e) => {
            eprintln!("Error during transcription: {}", e);
            process::exit(1);
        }
    };

    fs::write(text_file, text).unwrap_or_else(|e| {
        eprintln!("Error writing transcription file: {}", e);
        process::exit(1);
    });

    println!("Transcription finished.");
}

fn load_whisper(model_name: &str) -> whisper::model::Whisper<TchBackend<f32>> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "wgpu-backend")] {
            type Backend = WgpuBackend<AutoGraphicsApi, f32, i32>;
            let device = WgpuDevice::BestAvailable;
        } else if #[cfg(feature = "torch-backend")] {
            type Backend = TchBackend<f32>;
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
