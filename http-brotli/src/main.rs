use axum::{extract::DefaultBodyLimit, routing::get, Router};
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[macro_use]
extern crate dotenv_codegen;

use axum::{body::StreamBody, response::IntoResponse};
use tokio_util::io::ReaderStream;

use axum::http::StatusCode;

#[tokio::main]
async fn main() {
    println!("{}", dotenv!("STORAGE_PATH"));
    println!("{}", dotenv!("PORT"));
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "files=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // create the STORAGE_PATH directory if it doesn't exist
    if !Path::new(dotenv!("STORAGE_PATH")).exists() {
        tokio::fs::create_dir_all(dotenv!("STORAGE_PATH"))
            .await
            .unwrap();
    }

    let app = Router::new()
        .route("/", get(index))
        .route("/*uri", get(in_directory))
        .layer(DefaultBodyLimit::disable())
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let port = dotenv!("PORT").parse::<u16>().unwrap();

    hyper::Server::bind(&SocketAddr::from(([127, 0, 0, 1], port)))
        .serve(
            app.clone()
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
}

pub async fn index() -> Result<impl IntoResponse, impl IntoResponse> {
    in_directory(axum::extract::Path("/index.html".to_string())).await
}

pub async fn in_directory(
    axum::extract::Path(mut uri): axum::extract::Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let body = async move {
        if !uri.starts_with("/") {
            uri = format!("/{uri}"); // add / prefix if not there
        }
        dbg!(&uri);
        let path = PathBuf::from(format!("{}{uri}", dotenv!("STORAGE_PATH")));
        dbg!(&path);

        dbg!();
        let mime = match mime_guess::from_path(&path).first_raw() {
            Some(mime) => mime.to_string(),
            None => "application/octet-stream".to_string(),
        };

        dbg!();
        let file = match tokio::fs::File::open(&path).await {
            Ok(file) => file,
            Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
        };

        dbg!();
        // convert the `AsyncRead` into a `Stream`
        let stream = ReaderStream::new(file);

        dbg!();
        // convert the `Stream` into an `axum::body::HttpBody`
        let body = StreamBody::new(stream);

        dbg!();
        println!("EXT: {ext:?}", ext = path.extension());
        let headers = if path.extension().map(|ext| ext.to_str()) == Some(Some("br")) {
            [
                ("Content-Type".to_string(), "application/wasm".to_string()),
                ("Content-Encoding".to_string(), "br".to_string()),
            ]
        } else {
            [
                ("Content-Type".to_string(), mime),
                ("placeholder".to_string(), "value".to_string()),
            ]
        };

        dbg!(&headers);
        Ok((headers, body))
    };
    tokio::task::spawn(body).await.unwrap()
}
