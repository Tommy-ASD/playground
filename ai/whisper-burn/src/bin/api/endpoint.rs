use axum::{
    extract::Multipart,
    extract::{ConnectInfo, State},
    http::header::HeaderMap,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io, net::SocketAddr, sync::Arc};
use whisper::{model::Whisper, transcribe::TranscribeStateForDebugging};

use crate::{
    transcribe::{transcribe, TranscriptionError},
    types::Response,
    utils::get_available_languages,
    Backend,
};

#[derive(Clone)]
pub struct ApiState {
    model: Arc<Whisper<Backend>>,
}

pub fn upload_router(model: Arc<Whisper<Backend>>) -> Router {
    Router::new()
        .route("/", post(accept_form_index))
        .with_state(ApiState { model })
}

pub async fn accept_form_index(
    addr: ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    state: State<ApiState>,
    multipart: Multipart,
) -> Result<Json<Transcription>, Json<Response>> {
    accept_form(
        axum::extract::Path("".to_string()),
        addr,
        headers,
        state,
        multipart,
    )
    .await
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transcription {
    in_file: String,
    result: String,
}

pub async fn accept_form(
    axum::extract::Path(uri): axum::extract::Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    State(ApiState { model }): State<ApiState>,
    mut multipart: Multipart,
) -> Result<Json<Transcription>, Json<Response>> {
    let mut response = Response::new(200);

    let incompetence_disclaimer =
        "I honestly know very little about the .wav standard, you'll need to figure this one out"
            .to_string();
    let audio_spec_message =
        "I can tell you that this API only supports 16000 bitrate single-channel files."
            .to_string();

    println!("Got addr {addr:?}");
    println!("Got headers: {headers:?}");
    println!("Got uri: {uri}");
    dbg!();
    let int_serv_err = Err(Json(Response::new(400)));
    while let Some(field) = multipart.next_field().await.unwrap() {
        dbg!();
        if let Some(file_name) = field.file_name() {
            let file_name = file_name.to_string();

            // let name = field.name().unwrap_or_default();
            // let content_type = field.content_type().unwrap_or_default();
            let data = field.bytes().await.unwrap_or_default();

            let save_path = format!("./audio/{file_name}");

            // create a new file and write the uploaded data
            match File::create(&save_path.clone()) {
                Ok(mut file) => {
                    if let Err(e) = io::copy(&mut data.as_ref(), &mut file) {
                        dbg!(e);
                        return int_serv_err;
                    }
                }
                Err(e) => {
                    dbg!(e);
                    return int_serv_err;
                }
            }
            let state = TranscribeStateForDebugging {
                idx: 0,
                running_since: std::time::Instant::now(),
            };
            let tx = match transcribe(&save_path, Arc::clone(&model), "en", state) {
                Ok(tx) => tx,
                Err(e) => {
                    match e {
                        TranscriptionError::TranscriptingError(_e) => {
                            response = response.with_code(500);
                        }
                        TranscriptionError::LoadTokenizer(_e) => {
                            response = response.with_code(500);
                        }
                        TranscriptionError::InvalidLanguage(e) => {
                            response = response
                                .with_code(400)
                                .with_details(&format!("We do not support the language code '{e}'. View notes for supported languages."))
                                .with_notes(get_available_languages());
                        }
                        TranscriptionError::LoadWaveform(e) => {
                            match &e {
                                hound::Error::FormatError(_e) => {
                                    dbg!();
                                    response = response.with_code(400).with_details(
                                        "We only support the .wav format at this time.",
                                    );
                                }
                                hound::Error::InvalidSampleFormat => {
                                    dbg!();
                                    response = response.with_code(400);
                                }
                                hound::Error::IoError(_e) => {
                                    dbg!();
                                    response = response
                                        .with_code(500)
                                        .with_details("Server-side IO error.")
                                }
                                hound::Error::TooWide => {
                                    dbg!();
                                    response = response
                                        .with_code(400)
                                        .with_details("Too wide?")
                                        .with_notes(vec![
                                            incompetence_disclaimer,
                                            audio_spec_message,
                                        ]);
                                }
                                hound::Error::UnfinishedSample => {
                                    dbg!();
                                    response = response
                                        .with_code(400)
                                        .with_details("Unfinished sample?")
                                        .with_notes(vec![
                                            incompetence_disclaimer,
                                            audio_spec_message,
                                        ]);
                                }
                                hound::Error::Unsupported => {
                                    dbg!();
                                    response = response
                                        .with_code(400)
                                        .with_details("Unsupported")
                                        .with_notes(vec![
                                            incompetence_disclaimer,
                                            audio_spec_message,
                                        ]);
                                }
                            };
                        }
                    };
                    return Err(Json(response));
                }
            };
            println!("Transcribed file; '{tx}'");
            return Ok(Json(Transcription {
                in_file: file_name,
                result: tx,
            }));
        }
    }

    dbg!();
    int_serv_err
}
