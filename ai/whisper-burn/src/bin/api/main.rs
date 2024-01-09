use burn_tch::TchBackend;
use endpoint::upload_router;

use std::{net::SocketAddr, sync::Arc};

mod endpoint;
mod load_whisper;
mod transcribe;

use crate::load_whisper::load_whisper;

cfg_if::cfg_if! {
    if #[cfg(feature = "wgpu-backend")] {
        type Backend = WgpuBackend<AutoGraphicsApi, f32, i32>;
    } else if #[cfg(feature = "torch-backend")] {
        type Backend = TchBackend<f32>;
    }
}

#[tokio::main]
async fn main() {
    let model_name = "tiny_en";

    let whisper = Arc::new(load_whisper(model_name));

    println!("Loaded whisper");

    let app = upload_router(Arc::clone(&whisper));

    println!("Created app");

    hyper::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
