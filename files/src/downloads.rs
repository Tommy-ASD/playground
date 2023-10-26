use axum::{response::Html, routing::get, Router};

use crate::{get_dir, types::FileType};

pub fn downloads_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/*uri", get(in_directory))
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
