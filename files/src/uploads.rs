use axum::{
    extract::ConnectInfo,
    extract::Multipart,
    http::{header::HeaderMap, StatusCode},
    response::Redirect,
};
use chrono::Utc;
use hyper::{Body, Response};
use serde_json::json;
use std::{fs::File, io, net::SocketAddr, path::Path};

pub async fn accept_form(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<Redirect, Response<Body>> {
    println!("Got addr {addr:?}");
    println!("Got headers: {headers:?}");
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

    let log_entry = if let Some(real_ip) = headers.get("x-real-ip") {
        json!({
            "real-ip": real_ip.to_str().unwrap(),
            "peer": addr,
            "path": save_path
        })
    } else {
        json!({
            "peer": addr,
            "path": save_path
        })
    };

    let now = Utc::now().naive_local();
    let timestamp = now.format("%Y-%m-%d.%H-%M-%S").to_string();
    let nanos = now.timestamp_nanos_opt().unwrap_or(now.timestamp());

    let file_name = format!("{timestamp}.{nanos}");

    let _ = std::fs::write(
        format!("{logs}/{file_name}", logs = dotenv!("LOG_PATH"),),
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
