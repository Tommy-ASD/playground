use std::path::PathBuf;

use axum::{body::StreamBody, response::IntoResponse, routing::get, Router};
use tokio_util::io::ReaderStream;
use zip::CompressionMethod;

use tokio::fs::File;

use axum::http::{header, StatusCode};
use uuid::Uuid;

use crate::zip::{zip_folder_to_file, zip_folder_to_file_taking};

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
        dbg!(dotenv!("STORAGE_PATH"), dotenv!("TEMP_PATH"));
        if !uri.starts_with("/") {
            uri = format!("/{uri}"); // add / prefix if not there
        }
        dbg!(&uri);
        let path = PathBuf::from(format!("{}{uri}", dotenv!("STORAGE_PATH")));
        dbg!(&path);
        let mut temp_storage = PathBuf::from(format!(
            "{}/{}.zip",
            dotenv!("TEMP_PATH"),
            Uuid::new_v4().to_string()
        ));
        tokio::fs::create_dir_all(dotenv!("TEMP_PATH"))
            .await
            .unwrap();
        let temp_file = File::create(&temp_storage).await.unwrap();
        dbg!(&temp_storage);
        let temp;

        let mime;

        if path.is_dir() {
            temp = true;
            dbg!();
            mime = "application/zip".to_string();

            let (tx, mut rx) = tokio::sync::mpsc::channel(64);

            // zipping the folder may take a while and we want stuff to keep happening, so we'll move it to another thread
            let zipping_task = tokio::task::spawn(zip_folder_to_file_taking(
                path.clone(),
                temp_file.into_std().await,
                CompressionMethod::Stored,
                Some(tx),
            ));

            dbg!();

            tokio::task::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    println!("MESSAGE!!!! {msg}");
                }
            });

            zipping_task.await.unwrap().unwrap();

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
        let filename = match path.file_name() {
            Some(name) => {
                if temp {
                    format!("{filename}.zip", filename = name.to_string_lossy())
                } else {
                    name.to_string_lossy().to_string()
                }
            }
            None => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "File name couldn't be determined".to_string(),
                ))
            }
        };

        dbg!();
        let file = match File::open(&temp_storage).await {
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
                format!("attachment; filename={}", filename),
            ),
        ];

        dbg!(&headers);
        Ok((headers, body))
    };
    tokio::task::spawn(body).await.unwrap()
}
