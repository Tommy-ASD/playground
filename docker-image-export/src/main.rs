use std::net::SocketAddr;

use axum::extract::Multipart;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use bollard::image::ImportImageOptions;
use bollard::Docker;
use clap::Parser;
use futures_util::stream::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short, action, default_value = "8080")]
    port: u16,
}

async fn fetch_and_save_image(image_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Connecting to Docker...");
    let docker = Docker::connect_with_socket_defaults()?;

    println!("Connected to Docker, exporting");
    let mut stream = docker.export_image(image_name);

    println!("Exported, writing to file");
    let file_path = format!("./{}.tar", image_name);
    let mut file = File::create(&file_path).await.unwrap();

    while let Some(bytes_result) = stream.next().await {
        let bytes = bytes_result.unwrap();
        file.write_all(&bytes).await.unwrap();
    }

    println!("File written, flushing");
    file.flush().await.unwrap();

    println!(":D");
    Ok(file_path)
}

async fn import_image(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to Docker...");
    let docker = Docker::connect_with_socket_defaults()?;

    println!("Connected to Docker Daemon");
    let options = ImportImageOptions::default();

    println!("Reading image file: {}", file_path);
    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    println!("File read successfully");

    println!("Importing image...");
    let mut stream = docker.import_image(options, buffer.into(), None);
    println!("Image import stream created");

    while let Some(result) = stream.next().await {
        match result {
            Ok(_) => println!("Image import in progress..."), // You can add more detailed logging if needed
            Err(e) => eprintln!("Error importing image: {}", e),
        }
    }

    println!("Image import completed");
    Ok(())
}

async fn import_handler(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();

        let data = field.bytes().await.unwrap();
        let file_path = format!("/tmp/{}", file_name);
        let mut file = File::create(&file_path).await.unwrap();
        file.write_all(&data).await.unwrap();
        file.flush().await.unwrap();

        if let Err(e) = import_image(&file_path).await {
            eprintln!("Error importing Docker image: {}", e);
            return Html("Failed to import Docker image".to_string());
        }
    }
    Html("Docker image imported successfully".to_string())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // let file = fetch_and_save_image("scpsl_server").await;
    // println!("{file:?}");
    import_image("./scpsl_server.tar").await.unwrap();

    let app = Router::new().route("/import", post(import_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
