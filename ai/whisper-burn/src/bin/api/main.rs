use burn_tch::TchBackend;
use endpoint::upload_router;

use std::{net::SocketAddr, sync::Arc};

use clap::Parser;

mod endpoint;
mod load_whisper;
mod transcribe;
mod types;
mod utils;

use crate::load_whisper::load_whisper;

cfg_if::cfg_if! {
    if #[cfg(feature = "wgpu-backend")] {
        type Backend = WgpuBackend<AutoGraphicsApi, f32, i32>;
    } else if #[cfg(feature = "torch-backend")] {
        type Backend = TchBackend<f32>;
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let model_name = "tiny_en";

    let whisper = Arc::new(load_whisper(model_name));

    println!("Loaded whisper");

    let app = upload_router(Arc::clone(&whisper));

    println!("Created app");

    hyper::Server::bind(&SocketAddr::from(([0, 0, 0, 0], args.port)))
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
