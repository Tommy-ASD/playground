use whisper::model::*;
use whisper::token::Language;
use whisper::transcribe::waveform_to_text;

use strum::IntoEnumIterator;

use hound::{self, SampleFormat};

use whisper::token::Gpt2Tokenizer;

use std::{process, sync::Arc};

use crate::Backend;

fn load_audio_waveform<B: burn::tensor::backend::Backend>(
    filename: &str,
) -> hound::Result<(Vec<f32>, usize)> {
    let reader = match hound::WavReader::open(filename) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
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

#[derive(Debug)]
pub enum TranscriptionError {
    InvalidLanguage(String),
    LoadWaveform(hound::Error),
    LoadTokenizer(Box<dyn std::error::Error + Send + Sync>),
    TranscriptingError(Box<dyn std::error::Error + Send + Sync>),
}

pub fn transcribe(
    wav_file: &str,
    model: Arc<Whisper<Backend>>,
    lang_str: &str,
) -> Result<String, TranscriptionError> {
    let lang = match Language::iter().find(|lang| lang.as_alpha2() == lang_str) {
        Some(lang) => lang,
        None => {
            eprintln!("Invalid language abbreviation: {}", lang_str);
            return Err(TranscriptionError::InvalidLanguage(lang_str.to_string()));
        }
    };

    println!("Loading waveform...");
    let (waveform, sample_rate) = match load_audio_waveform::<Backend>(wav_file) {
        Ok((w, sr)) => (w, sr),
        Err(e) => {
            eprintln!("Failed to load audio file: {}", e);
            return Err(TranscriptionError::LoadWaveform(e));
        }
    };

    let bpe = match Gpt2Tokenizer::new() {
        Ok(bpe) => bpe,
        Err(e) => {
            eprintln!("Failed to load tokenizer: {}", e);
            return Err(TranscriptionError::LoadTokenizer(e));
        }
    };

    let (text, _tokens) = match waveform_to_text(&model.as_ref(), &bpe, lang, waveform, sample_rate)
    {
        Ok((text, tokens)) => (text, tokens),
        Err(e) => {
            eprintln!("Error during transcription: {}", e);
            return Err(TranscriptionError::TranscriptingError(e));
        }
    };

    Ok(text)
}
