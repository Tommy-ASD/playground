use std::{
    fs::File,
    io,
    net::SocketAddr,
    path::{Path, PathBuf},
};

use axum::{
    extract::ConnectInfo,
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
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

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    println!("{}", dotenv!("STORAGE_PATH"));
    println!("{}", dotenv!("STATIC_PATH"));
    println!("{}", dotenv!("LOG_PATH"));
    println!("{}", dotenv!("URL"));
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
        .nest_service("/downloads", ServeDir::new(dotenv!("STORAGE_PATH")))
        .nest_service("/static", ServeDir::new(dotenv!("STATIC_PATH")))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, // 250mb
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    hyper::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8080)))
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

#[derive(Debug)]
struct FileDownloadData {
    pub name: String,
    pub path: PathBuf,
}

impl FileDownloadData {
    pub fn to_list_element(&self) -> String {
        format!(
            r#"
<li class="file-item">
    <a href="/downloads/{path}" class="download-link" download>Download {name}</a>
</li>"#,
            path = self
                .path
                .to_string_lossy()
                .strip_prefix("./uploads\\")
                .unwrap(),
            name = self.name
        )
    }
}

fn get_all_files() -> Vec<FileDownloadData> {
    std::fs::read_dir(dotenv!("STORAGE_PATH"))
        .unwrap()
        .into_iter()
        .filter_map(|opt_entry| opt_entry.ok())
        .map(|file| {
            let file_data = FileDownloadData {
                name: file.file_name().to_str().unwrap().to_string(),
                path: file.path().to_path_buf(),
            };
            println!("Adding {file_data:?}");
            file_data
        })
        .collect()
}

async fn show_form() -> Html<String> {
    let lis = get_all_files()
        .iter()
        .map(|file| file.to_list_element())
        .collect::<Vec<String>>()
        .join("\n");
    let rendered = format!(
        r#"
        <!doctype html>
        <html>
            <head>
                <link rel="stylesheet" type="text/css" href="/static/style.css">
            </head>
            <body>
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

async fn accept_form(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut multipart: Multipart,
) -> Result<Redirect, Response<Body>> {
    let int_serv_err = Err(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .unwrap());
    let mut save_path = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.file_name().is_some() {
            let file_name = field.file_name().unwrap().to_string();

            // let name = field.name().unwrap_or_default();
            // let content_type = field.content_type().unwrap_or_default();
            let data = field.bytes().await.unwrap_or_default();

            save_path = Some(format!(
                "{path}/{file_name}",
                path = dotenv!("STORAGE_PATH")
            ));

            if std::fs::metadata(save_path.clone().unwrap()).is_ok() {
                return Err(Response::builder()
                    .status(StatusCode::CONFLICT)
                    .body(Body::empty())
                    .unwrap());
            }

            // create a new file and write the uploaded data
            match File::create(&save_path.clone().unwrap()) {
                Ok(mut file) => {
                    if let Err(_) = io::copy(&mut data.as_ref(), &mut file) {
                        return int_serv_err;
                    }
                }
                Err(_) => {
                    return int_serv_err;
                }
            }
        }
    }

    if !Path::new(dotenv!("LOG_PATH")).exists() {
        std::fs::create_dir(dotenv!("LOG_PATH")).unwrap();
    }

    // log file uploaded and IP address of person uploading
    let log_entry = json!({
        "peer": addr,
        "path": save_path
    });

    let _ = std::fs::write(
        format!(
            "{logs}/{timestamp}",
            logs = dotenv!("LOG_PATH"),
            timestamp = Utc::now().naive_local()
        ),
        log_entry.to_string(),
    );

    if let Some(path) = save_path {
        Ok(Redirect::to(&format!(
            "/uploaded/{}",
            path.strip_prefix(&format!("{}/", dotenv!("STORAGE_PATH")))
                .unwrap()
        )))
    } else {
        return int_serv_err;
    }
}
