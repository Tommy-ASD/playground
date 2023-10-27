use std::path::PathBuf;

use axum::{
    body::StreamBody, extract::Query, http::HeaderValue, response::IntoResponse, routing::get,
    Router,
};
use hyper::HeaderMap;
use serde::Deserialize;
use tokio_util::io::ReaderStream;
use tower_http::services::ServeFile;

use axum::http::{header, StatusCode};
use std::net::SocketAddr;
use uuid::Uuid;
use zip::result::ZipError;

use crate::zip::zip_folder_to_file;

pub fn downloads_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/*uri", get(in_directory))
}

pub async fn index() -> Result<impl IntoResponse, impl IntoResponse> {
    in_directory(axum::extract::Path("".to_string())).await
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
        let mut temp_storage = PathBuf::from(format!(
            "{}/{}",
            dotenv!("TEMP_PATH"),
            Uuid::new_v4().to_string()
        ));
        let temp;

        let mime;

        if path.is_dir() {
            temp = true;
            dbg!();
            mime = "application/zip".to_string();
            match zip_folder_to_file(&path, &temp_storage, zip::CompressionMethod::Stored) {
                Ok(_) => {}
                Err(ZipError::FileNotFound) => {
                    return Err((StatusCode::NOT_FOUND, format!("File not found")))
                }
                Err(_) => return Err((StatusCode::NOT_FOUND, format!("Unknown error occured"))),
            };
            dbg!();
        } else {
            temp = false;
            dbg!();
            temp_storage = path.clone();
            mime = match mime_guess::from_path(&path).first_raw() {
                Some(mime) => mime.to_string(),
                None => "application/octet-stream".to_string(),
            };
            dbg!(&temp_storage);
        }

        dbg!();
        let filename: &std::ffi::OsStr = match path.file_name() {
            Some(name) => name,
            None => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "File name couldn't be determined".to_string(),
                ))
            }
        };

        dbg!();
        let file = match tokio::fs::File::open(&temp_storage).await {
            Ok(file) => file,
            Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
        };

        dbg!();
        // convert the `AsyncRead` into a `Stream`
        let stream = ReaderStream::new(file);

        dbg!();
        // convert the `Stream` into an `axum::body::HttpBody`
        let body = StreamBody::new(stream);

        if temp {
            let _ = tokio::fs::remove_file(temp_storage).await;
        }

        dbg!();
        let headers = [
            (header::CONTENT_TYPE, mime),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename={:?}", filename),
            ),
        ];

        dbg!(&headers);
        Ok((headers, body))
    };
    tokio::task::spawn(body).await.unwrap()
}
