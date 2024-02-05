use axum::{
    extract::{ConnectInfo, Query},
    http::{header::HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use chrono::Utc;
use hyper::{Body, Response};
use serde::Deserialize;
use serde_json::json;
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::Mutex;
use traceback_error::traceback;
use url::Url;

#[derive(Deserialize)]
pub struct DownloadLink {
    link: Url,
}

pub fn initialize_download_router() -> Router {
    Router::new()
        .route("/", post(initialize_download_index))
        .route("/*uri", post(initialize_download))
}

pub async fn initialize_download_index(
    addr: ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    link: Query<DownloadLink>,
) -> impl IntoResponse {
    initialize_download(axum::extract::Path("".to_string()), addr, headers, link).await
}

pub async fn initialize_download(
    axum::extract::Path(uri): axum::extract::Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Query(DownloadLink { link }): Query<DownloadLink>,
) -> impl IntoResponse {
    println!("Got addr {addr:?}");
    println!("Got headers: {headers:?}");
    println!("Got uri: {uri}");
    let log_entry = if let Some(real_ip) = headers.get("x-real-ip") {
        json!({
            "real-ip": real_ip.to_str().unwrap(),
            "peer": addr,
            "url": link,
            "uri": uri
        })
    } else {
        json!({
            "peer": addr,
            "url": link,
            "uri": uri
        })
    };

    let int_serv_err = Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .unwrap();

    {
        let path = PathBuf::from(format!("{}{uri}", dotenv!("STORAGE_PATH")));
        let process = match tokio::process::Command::new("wget")
            .arg(link.as_str())
            .current_dir(path)
            .spawn()
        {
            Ok(proc) => Arc::new(Mutex::new(proc)), // Wrap the process in an Arc
            Err(e) => {
                traceback!(err e, "Failed to start wget").with_extra_data(log_entry);
                return Err(int_serv_err);
            }
        };

        let process_clone = Arc::clone(&process); // Clone the Arc to use in the spawned future

        tokio::spawn(async move {
            process_clone.lock().await.wait().await // Use the cloned process in the future
        });
    }
    dbg!();
    let path = PathBuf::from(&format!("{root}/{uri}", root = dotenv!("STORAGE_PATH")));
    if let Some(anc) = path.ancestors().last() {
        let _ = tokio::fs::create_dir_all(anc);
        println!("Recursively created {anc:?}");
    }
    dbg!();

    if !Path::new(dotenv!("LOG_PATH")).exists() {
        tokio::fs::create_dir_all(dotenv!("LOG_PATH"))
            .await
            .unwrap();
    }
    dbg!();

    // log file uploaded and IP address of person uploading

    dbg!();
    let now = Utc::now().naive_local();
    let timestamp = now.format("%Y-%m-%d.%H-%M-%S").to_string();
    let nanos = now.timestamp_nanos_opt().unwrap_or(now.timestamp());

    dbg!();
    let file_name = format!("{timestamp}.{nanos}");

    let _ = std::fs::write(
        format!("{logs}/{file_name}", logs = dotenv!("LOG_PATH"),),
        log_entry.to_string(),
    );
    Ok(())
}
