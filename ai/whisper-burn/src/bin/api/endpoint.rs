use axum::{
    extract::Multipart,
    extract::{ConnectInfo, State},
    http::{header::HeaderMap, StatusCode},
    response::Redirect,
    routing::post,
    Router,
};
use hyper::{Body, Response};
use std::{fs::File, io, net::SocketAddr, sync::Arc};
use whisper::model::Whisper;

use crate::{transcribe::transcribe, Backend};

#[derive(Clone)]
pub struct ApiState {
    model: Arc<Whisper<Backend>>,
}

pub fn upload_router(model: Arc<Whisper<Backend>>) -> Router {
    Router::new()
        .route("/", post(accept_form_index))
        .route("/*uri", post(accept_form))
        .with_state(ApiState { model })
}

pub async fn accept_form_index(
    addr: ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    state: State<ApiState>,
    multipart: Multipart,
) -> Result<Redirect, Response<Body>> {
    accept_form(
        axum::extract::Path("".to_string()),
        addr,
        headers,
        state,
        multipart,
    )
    .await
}

pub async fn accept_form(
    axum::extract::Path(uri): axum::extract::Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    State(ApiState { model }): State<ApiState>,
    mut multipart: Multipart,
) -> Result<Redirect, Response<Body>> {
    println!("Got addr {addr:?}");
    println!("Got headers: {headers:?}");
    println!("Got uri: {uri}");
    dbg!();
    let int_serv_err = Err(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::default())
        .unwrap());
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
            let tx = transcribe(&save_path, Arc::clone(&model), "en");
            println!("Transcribed file; '{tx}'")
        }
    }

    dbg!();
    int_serv_err
}
