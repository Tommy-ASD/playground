use axum::{extract::DefaultBodyLimit, response::Html, routing::get, routing::post, Router};
use directories::render_files_and_directories;
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use types::FileDownloadData;
use yew::html;

use crate::{
    directories::{create_directories_router, get_directories_router},
    downloads::downloads_router,
    initialize_download::{initialize_download_index, initialize_download_router},
    // oauth::oauth_router,
    uploads::{accept_form_index, upload_router},
};

#[macro_use]
extern crate dotenv_codegen;

mod directories;
mod downloads;
mod initialize_download;
// mod oauth;
mod types;
mod uploads;
mod zip;

#[tokio::main]
async fn main() {
    println!("{}", dotenv!("STORAGE_PATH"));
    println!("{}", dotenv!("STATIC_PATH"));
    println!("{}", dotenv!("LOG_PATH"));
    println!("{}", dotenv!("URL"));
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

    // build our application with some routes
    let app = Router::new()
        .route("/", get(show_form))
        .nest("/upload", upload_router())
        .route("/upload/", post(accept_form_index))
        .route("/uploaded/*uri", get(successfully_uploaded))
        .nest("/directory", get_directories_router())
        .route("/directory/", get(directories::index)) // to support trailing slash for url
        .nest("/create-dir", create_directories_router())
        .route("/create-dir/", post(directories::create_directory_index))
        .nest("/download", downloads_router())
        .route("/download/", get(downloads::index)) // to support trailing slash for url
        .nest("/initialize_download", initialize_download_router())
        .route("/initialize_download/", post(initialize_download_index))
        // .nest("/oauth", oauth_router())
        .nest_service("/static", ServeDir::new(dotenv!("STATIC_PATH")))
        .layer(DefaultBodyLimit::disable())
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let port = dotenv!("PORT").parse::<u16>().unwrap();

    hyper::Server::bind(&SocketAddr::from(([127, 0, 0, 1], port)))
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn get_all_files() -> Vec<FileDownloadData> {
    get_dir("").await.unwrap()
}

async fn get_dir(path: &str) -> Option<Vec<FileDownloadData>> {
    let dir = match std::fs::read_dir(format!("{}/{path}", dotenv!("STORAGE_PATH"))) {
        Ok(dir) => dir,
        Err(_) => return None,
    };
    let mut data = vec![];
    for opt_file in dir {
        if opt_file.is_err() {
            continue;
        }
        let file = opt_file.unwrap();

        let file_data = FileDownloadData::from_file(file).await;
        println!("Adding {file_data:?}");
        data.push(file_data)
    }
    Some(data)
}

async fn show_form() -> Html<String> {
    let rendered = main_page(
        "",
        r#"
                <h1>Hiii :3</h1>
                <h2>So if you're here, you probably know this website exists. Please keep it to yourself.</h2>
                <h3>Also stay legal</h3>
                <img src="https://files.tommyasd.com/download/disclaimer.gif" alt="This website logs IP addresses when you upload or download anything">"#,
    ).await;
    Html(rendered)
}

async fn main_page(uri: &str, header: &str) -> String {
    let mut split = uri.split("/").into_iter().collect::<Vec<&str>>();
    split.pop();
    println!("Split: {split:?}");
    let back = split
        .iter()
        .map(|part| part.to_string())
        .collect::<Vec<String>>()
        .join("/");
    println!("Back: {back}");
    let lis = render_files_and_directories(uri).await.unwrap();
    format!(
        r#"
        <!doctype html>
        <html>
            <head>
                <link rel="stylesheet" type="text/css" href="/static/style.css">
            </head>
            <body>
{header}
                <div id="file-upload">
                <h1>You are currently at /{uri}</h1>
                    <a href="/directory/{back}">Go up one</a>
                    <h2>Upload Files</h2>
                    <form id="file-form" action="/upload/{uri}" method="post" enctype="multipart/form-data">
                        <input type="file" name="file" id="file-input" accept="*" multiple>
                        <label for="file-input" id="file-label">Choose a file</label>
                        <input type="submit" value="Upload">
                    </form>
                    <h2>Create Directory</h2>
                    <form action="/create-dir/{uri}" method="post" id="createDirForm">
                        <label for="directory_name">Directory Name:</label>
                        <input type="text" id="directory_name" name="path">
                        <br>
                        <input type="submit" value="Create Directory">
                    </form>
                </div>
                            
                <div id="file-download">
                    <h2>Available Files</h2>
                    {lis}
                </div>
                <script>
const fileInput = document.getElementById('file-input');
fileInput.addEventListener('dragover', (e) => {{
    e.preventDefault();
}});

fileInput.addEventListener('dragenter', (e) => {{
    e.preventDefault();
}});
                </script>
            </body>
        </html>
        "#,
    )
}

async fn successfully_uploaded(
    axum::extract::Path(uri): axum::extract::Path<String>,
) -> Html<String> {
    let uri = uri.replace(" ", "%20");
    let url = format!("{url}/download/{uri}", url = dotenv!("URL"));
    let rendered = format!(
        r#"
        <!doctype html>
        <html>
            <head>
                <link rel="stylesheet" type="text/css" href="/static/style.css">
            </head>
            <body>
                <h1>Successfully uploaded!</h1>
                <p>Available at <a href={url}>{url}</a></p>

                <button onclick="window.location.href = '/';">Back</button>
            </body>
        </html>
        "#,
    );
    Html(rendered)
}
