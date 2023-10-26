use axum::{
    extract::ConnectInfo,
    extract::Multipart,
    http::{header::HeaderMap, StatusCode},
    response::Redirect,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use hyper::{Body, Response};
use serde_json::json;
use std::{
    fs::File,
    io,
    net::SocketAddr,
    path::{Path, PathBuf},
};

pub fn upload_router() -> Router {
    Router::new()
        .route("/", post(accept_form_index))
        .route("/*uri", post(accept_form))
}

pub async fn accept_form_index(
    addr: ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    multipart: Multipart,
) -> Result<Redirect, Response<Body>> {
    accept_form(
        axum::extract::Path("".to_string()),
        addr,
        headers,
        multipart,
    )
    .await
}

pub async fn accept_form(
    axum::extract::Path(uri): axum::extract::Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<Redirect, Response<Body>> {
    println!("Got addr {addr:?}");
    println!("Got headers: {headers:?}");
    println!("Got uri: {uri}");
    dbg!();
    let int_serv_err = Err(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .unwrap());
    dbg!();
    let mut save_path = None;
    let path = PathBuf::from(&format!("{root}/{uri}", root = dotenv!("STORAGE_PATH")));
    if let Some(anc) = path.ancestors().last() {
        let _ = tokio::fs::create_dir_all(anc);
        println!("Recursively created {anc:?}");
    }
    while let Some(field) = multipart.next_field().await.unwrap() {
        dbg!();
        if field.file_name().is_some() {
            let file_name = field.file_name().unwrap().to_string();

            // let name = field.name().unwrap_or_default();
            // let content_type = field.content_type().unwrap_or_default();
            let data = field.bytes().await.unwrap_or_default();

            save_path = if uri.is_empty() {
                Some(format!(
                    "{path}/{file_name}",
                    path = dotenv!("STORAGE_PATH")
                ))
            } else {
                Some(format!(
                    "{path}/{uri}/{file_name}",
                    path = dotenv!("STORAGE_PATH")
                ))
            };

            if std::fs::metadata(save_path.clone().unwrap()).is_ok() {
                dbg!();
            }

            // create a new file and write the uploaded data
            match File::create(&save_path.clone().unwrap()) {
                Ok(mut file) => {
                    if let Err(e) = io::copy(&mut data.as_ref(), &mut file) {
                        dbg!(e);
                        return int_serv_err;
                    }
                }
                Err(e) => {
                    dbg!(e);
                    return int_serv_err;
                }
            }
        }
    }
    dbg!();

    if !Path::new(dotenv!("LOG_PATH")).exists() {
        tokio::fs::create_dir_all(dotenv!("LOG_PATH"))
            .await
            .unwrap();
    }
    dbg!();

    // log file uploaded and IP address of person uploading

    let log_entry = if let Some(real_ip) = headers.get("x-real-ip") {
        json!({
            "real-ip": real_ip.to_str().unwrap(),
            "peer": addr,
            "path": save_path,
            "uri": uri
        })
    } else {
        json!({
            "peer": addr,
            "path": save_path,
            "uri": uri
        })
    };

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
    dbg!();

    if let Some(path) = save_path {
        dbg!();
        Ok(Redirect::to(&format!(
            "/uploaded/{}",
            path.strip_prefix(&format!("{}/", dotenv!("STORAGE_PATH")))
                .unwrap()
        )))
    } else {
        dbg!();
        return int_serv_err;
    }
}
