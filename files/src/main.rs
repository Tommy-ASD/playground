use axum::{extract::DefaultBodyLimit, response::Html, routing::get, routing::post, Router};
use directories::render_files_and_directories;
use std::{net::SocketAddr, path::Path};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use types::FileDownloadData;

use crate::{directories::directories_router, downloads::downloads_router, uploads::accept_form};

#[macro_use]
extern crate dotenv_codegen;

mod directories;
mod downloads;
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
        std::fs::create_dir(dotenv!("STORAGE_PATH")).unwrap();
    }

    // build our application with some routes
    let app = Router::new()
        .route("/", get(show_form))
        .route("/upload", post(accept_form))
        .route("/uploaded/:uri", get(successfully_uploaded))
        .nest("/directory", directories_router())
        .route("/directory/", get(directories::index)) // to support trailing slash for url
        .nest("/download", downloads_router())
        .route("/download/", get(downloads::index)) // to support trailing slash for url
        .nest_service("/static", ServeDir::new(dotenv!("STATIC_PATH")))
        .layer(DefaultBodyLimit::disable())
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let port = dotenv!("PORT").parse::<u16>().unwrap();

    hyper::Server::bind(&SocketAddr::from(([127, 0, 0, 1], port)))
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

fn get_all_files() -> Vec<FileDownloadData> {
    get_dir("").unwrap()
}

fn get_dir(path: &str) -> Option<Vec<FileDownloadData>> {
    match std::fs::read_dir(format!("{}/{path}", dotenv!("STORAGE_PATH"))) {
        Ok(dir) => Some(
            dir.into_iter()
                .filter_map(|opt_entry| opt_entry.ok())
                .map(|file| {
                    let mime = match mime_guess::from_path(&file.path()).first_raw() {
                        Some(mime) => mime.to_string(),
                        None => "application/octet-stream".to_string(),
                    };
                    let file_data = FileDownloadData {
                        name: file.file_name().to_str().unwrap().to_string(),
                        path: file.path().to_path_buf(),
                        filetype: file.path().to_path_buf().into(),
                        mime_type: mime,
                    };
                    println!("Adding {file_data:?}");
                    file_data
                })
                .collect(),
        ),
        Err(_) => None,
    }
}

async fn show_form() -> Html<String> {
    let lis = render_files_and_directories("").await.unwrap();
    let rendered = format!(
        r#"
        <!doctype html>
        <html>
            <head>
                <link rel="stylesheet" type="text/css" href="/static/style.css">
            </head>
            <body>
              <!--  <h1>Hiii :3</h1>
                <h2>So if you're here, you probably know this website exists. Please keep it to yourself.</h2>
                <h3>Also stay legal</h3>
                <img src="https://files.tommyasd.com/downloads/disclaimer.gif" alt="This website logs IP addresses when you upload or download anything"> -->
                <div id="file-upload">
                    <h2>Upload Files</h2>
                    <form action="/upload" method="post" enctype="multipart/form-data">
                        <input type="file" name="file">
                        <input type="submit" value="Upload">
                    </form>
                </div>
                            
                <div id="file-download">
                    <h2>Available Files</h2>
                    <ul id="file-list">
                    {lis}
                    </ul>
                </div>
            </body>
        </html>
        "#,
    );
    Html(rendered)
}

async fn successfully_uploaded(
    axum::extract::Path(uri): axum::extract::Path<String>,
) -> Html<String> {
    let uri = uri.replace(" ", "%20");
    let url = format!("{url}/downloads/{uri}", url = dotenv!("URL"));
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
