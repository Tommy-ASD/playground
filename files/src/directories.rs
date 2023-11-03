use axum::{
    response::{Html, Redirect},
    routing::{get, post},
    Form, Router,
};
use hyper::HeaderMap;
use serde::Deserialize;
use yew::html;

use crate::{get_dir, main_page, types::FileType};

pub fn get_directories_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/*uri", get(in_directory))
}

pub async fn index() -> Html<String> {
    in_directory(axum::extract::Path("".to_string())).await
}

pub async fn in_directory(axum::extract::Path(uri): axum::extract::Path<String>) -> Html<String> {
    println!("Got uri {uri}");

    let rendered = main_page(&uri, html! {}).await;
    Html(rendered)
}

pub async fn render_files_and_directories(uri: &str) -> Option<String> {
    let intial = r#"
<table class="downloads-table">
    <tr>
        <th>File Name</th>
        <th>File Size</th>
        <th>Created</th>
        <th>Last Accessed</th>
    </tr>"#;
    let mut directories: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    let dir = match get_dir(&uri).await {
        Some(dir) => dir,
        None => return None,
    };
    for file in dir {
        match file.filetype {
            FileType::Directory => {
                directories.push(file.to_list_element().await);
            }
            FileType::File => {
                files.push(file.to_list_element().await);
            }
            _ => {
                println!(
                    "Path {path:?} has filetype {filetype:?}",
                    path = file.path,
                    filetype = file.filetype
                )
            }
        }
    }
    let directories: String = directories.into_iter().collect::<String>();
    let files: String = files.into_iter().collect::<String>();

    let contents = format!("{intial}{directories}{files}</table>");

    Some(contents)
}

pub fn create_directories_router() -> Router {
    Router::new()
        .route("/", post(create_directory_index))
        .route("/*uri", post(create_directory))
        .route("/", get(create_directory_index))
        .route("/*uri", get(create_directory))
}

#[derive(Deserialize, Debug)]
pub struct CreateDirForm {
    pub path: String,
}

pub async fn create_directory_index(form: Form<CreateDirForm>) -> Redirect {
    dbg!();
    create_directory(axum::extract::Path("".to_string()), form).await
}

pub async fn create_directory(
    axum::extract::Path(uri): axum::extract::Path<String>,
    Form(input): Form<CreateDirForm>,
) -> Redirect {
    dbg!();
    let uri = match uri.strip_suffix("input") {
        Some(stripped) => stripped.to_string(),
        None => uri,
    };
    dbg!();
    println!("Form: {input:?}");
    let path = input.path.clone();
    println!("Making '{path}'");
    println!("From uri '{uri}'");
    dbg!();
    let full_path = if uri.is_empty() {
        format!("{}/{path}", dotenv!("STORAGE_PATH"))
    } else {
        format!("{}/{uri}/{path}", dotenv!("STORAGE_PATH"))
    };
    dbg!();
    println!("Final path: {full_path}");
    let _ = tokio::fs::create_dir_all(full_path.clone()).await;
    dbg!();
    let redirect = if uri.is_empty() {
        format!("/directory/{path}")
    } else {
        format!("/directory/{uri}/{path}")
    };
    dbg!();
    Redirect::to(&redirect)
}
