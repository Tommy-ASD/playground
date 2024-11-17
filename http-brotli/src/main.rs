use axum::{
    body::StreamBody,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use clap::Parser;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio_util::io::ReaderStream;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long, short, action, default_value = "8080")]
    port: u16,
    #[clap(long, short, action, default_value = ".")]
    storage_path: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let shared_state = Arc::new(args);
    println!("{}", shared_state.storage_path);
    println!("{}", shared_state.port);

    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "files=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Ensure the storage path directory exists
    if !std::path::Path::new(&shared_state.storage_path).exists() {
        tokio::fs::create_dir_all(&shared_state.storage_path)
            .await
            .unwrap();
    }

    let app = Router::new()
        .route("/", get(index))
        .route("/*uri", get(in_directory))
        .with_state(Arc::clone(&shared_state))
        .layer(axum::extract::DefaultBodyLimit::disable())
        .layer(tower_http::trace::TraceLayer::new_for_http());

    hyper::Server::bind(&SocketAddr::from(([127, 0, 0, 1], shared_state.port)))
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

pub async fn index(
    state: State<Arc<Args>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    in_directory(state, Path("/index.html".to_string())).await
}

pub async fn in_directory(
    State(state): State<Arc<Args>>,
    Path(mut uri): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let body = async move {
        if !uri.starts_with("/") {
            uri = format!("/{uri}"); // add / prefix if not there
        }
        dbg!(&uri);
        let path = PathBuf::from(format!("{}{uri}", state.storage_path));
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
