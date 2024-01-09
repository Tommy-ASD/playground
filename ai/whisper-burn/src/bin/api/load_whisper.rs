use burn_tch::TchDevice;
use whisper::model::Whisper;
use whisper::model::WhisperConfig;

use burn::{config::Config, module::Module};

use burn::record::{DefaultRecorder, Recorder, RecorderError};

use std::process;

use crate::Backend;

pub fn load_whisper(model_name: &str) -> whisper::model::Whisper<Backend> {
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

fn load_whisper_model_file<B: burn::tensor::backend::Backend>(
    config: &WhisperConfig,
    filename: &str,
) -> Result<Whisper<B>, RecorderError> {
    DefaultRecorder::new()
        .load(filename.into())
        .map(|record| config.init().load_record(record))
}
