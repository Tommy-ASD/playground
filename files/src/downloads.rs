use std::{
    fs::File,
    io,
    net::SocketAddr,
    path::{Path, PathBuf},
};

use axum::{
    extract::ConnectInfo,
    extract::{DefaultBodyLimit, Multipart},
    http::{header::HeaderMap, StatusCode},
    response::{Html, Redirect},
    routing::get,
    routing::post,
    Router,
};
use chrono::Utc;
use hyper::{Body, Response};
use serde_json::json;
use tower_http::{limit::RequestBodyLimitLayer, services::ServeDir};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{get_all_files, get_dir};

pub fn downloads_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/*uri", get(in_directory))
}

#[derive(Debug)]
pub enum FileType {
    Directory,
    File,
    Symlink,
    Unknown,
}

impl From<PathBuf> for FileType {
    fn from(value: PathBuf) -> Self {
        if value.is_dir() {
            Self::Directory
        } else if value.is_file() {
            Self::File
        } else if value.is_symlink() {
            Self::Symlink
        } else {
            Self::Unknown
        }
    }
}

#[derive(Debug)]
pub struct FileDownloadData {
    pub name: String,
    pub path: PathBuf,
    pub filetype: FileType,
}

impl FileDownloadData {
    pub fn to_list_element(&self) -> String {
        let binding = self.path.to_string_lossy();
        let mut path = binding
            .strip_prefix(dotenv!("STORAGE_PATH"))
            .unwrap_or(&self.name);
        if let Some(inner_path) = path.strip_prefix("\\") {
            path = inner_path;
        }
        if let Some(inner_path) = path.strip_prefix("/") {
            path = inner_path;
        }

        match self.filetype {
            FileType::File => {
                format!(
                    r#"
<li class="file-item">
    <a href="/downloads/{path}" class="download-link" download>{name}</a>
</li>"#,
                    name = self.name
                )
            }
            FileType::Directory => {
                format!(
                    r#"
<li class="file-item">
    <a href="/downloads/{path}" class="directory-link" id={name}>{name}</a>
</li>"#,
                    name = self.name
                )
            }
            _ => {
                format!(
                    r#"
<li class="file-item">
    <a href="/downloads/{path}" class="download-link" download>{name}</a>
</li>"#,
                    name = self.name
                )
            }
        }
    }
}

pub async fn index() -> Html<String> {
    in_directory(axum::extract::Path("".to_string())).await
}

pub async fn in_directory(axum::extract::Path(uri): axum::extract::Path<String>) -> Html<String> {
    println!("Got uri {uri}");
    let mut directories: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    let dir = match get_dir(&uri) {
        Some(dir) => dir,
        None => return Html::from("404 not found".to_string()),
    };
    dir.iter().for_each(|file| match file.filetype {
        FileType::Directory => {
            directories.push(file.to_list_element());
        }
        FileType::File => {
            files.push(file.to_list_element());
        }
        _ => {
            println!(
                "Path {path:?} has filetype {filetype:?}",
                path = file.path,
                filetype = file.filetype
            )
        }
    });
    let directories: String = directories.into_iter().collect::<String>();
    let files: String = files.into_iter().collect::<String>();

    let rendered = format!(
        r#"
        <!doctype html>
        <html>
            <head>
                <link rel="stylesheet" type="text/css" href="/static/style.css">
            </head>
            <body>
                <div id="file-download">
                    <h2>Available Files</h2>
                    <ul id="file-list">
                    {directories}
                    {files}
                    </ul>
                </div>
            </body>
        </html>
        "#,
    );
    Html(rendered)
}
