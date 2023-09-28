use std::fs::File;
use std::io::{self, Read};
use std::thread::sleep;
use std::time::Duration;

use reqwest::{self, header, multipart};
use serde::Serialize;

use serde_json::json;

use traceback_error::{traceback, TracebackError};

#[derive(Serialize)]
struct VideoSnippet {
    title: String,
    description: String,
    tags: Vec<String>,
    category_id: String,
}

#[derive(Serialize)]
struct VideoStatus {
    #[serde(rename = "privacyStatus")]
    privacy_status: String,
}

#[traceback_derive::traceback]
pub async fn primary(token: &str) -> Result<(), TracebackError> {
    // Read video metadata and file path from command line arguments or configuration
    let video_title = "Test Title";
    let video_description = "Test Description";
    let video_tags = vec!["tag1".to_string(), "tag2".to_string()];
    let video_category_id = "22";
    let snippet = VideoSnippet {
        title: video_title.to_string(),
        description: video_description.to_string(),
        tags: video_tags,
        category_id: video_category_id.to_string(),
    };
    let video_file_path = "./cpp.mp4"; // Update with your video file path

    // Initialize a Reqwest HTTP client with the access token

    // Upload the video
    let video_url = upload_video(video_file_path, &snippet, token).await?;

    println!("Video uploaded successfully. Video URL: {}", video_url);

    Ok(())
}

#[traceback_derive::traceback]
async fn upload_video(
    file_path: &str,
    snippet: &VideoSnippet,
    token: &str,
) -> Result<String, TracebackError> {
    let http_client = reqwest::Client::new();

    // Open and read the video file
    let file = File::open(file_path)?;
    let file_len = file.metadata()?.len();
    let mut buffer = Vec::with_capacity(file_len as usize);
    let _ = io::BufReader::new(file).read_to_end(&mut buffer)?;

    // Create a multipart form for video upload
    let form = reqwest::multipart::Form::new()
        .text("snippet", serde_json::to_string(snippet)?)
        .part(
            "video",
            multipart::Part::bytes(buffer).file_name(file_path.to_string()),
        );
    println!("Form: {form:?}");

    // Upload the video using the YouTube API
    let response = http_client
        .post("http://localhost:13425")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .multipart(form)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_msg = response.text().await?;
        return Err(traceback!(err error_msg));
    }

    // Parse the video ID from the response
    let response_data: serde_json::Value =
        serde_json::from_str(response.text().await.unwrap().as_str())?;
    dbg!(&response_data);
    let video_id = response_data["id"]
        .as_str()
        .ok_or("Video ID not found in the response")?;

    // Wait for a moment to ensure the video is processed
    sleep(Duration::from_secs(10));

    // Generate the video URL
    let video_url = format!("https://www.youtube.com/watch?v={}", video_id);

    Ok(video_url)
}
