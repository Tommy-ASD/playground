use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

use chrono::Utc;
use reqwest::{self, header, multipart};
use serde::Serialize;

use serde_json::json;

use tokio::io::AsyncReadExt;
use traceback_error::{traceback, TracebackError};

use std::{
    ffi::{OsStr, OsString},
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    os::windows::process::CommandExt,
};

use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    reqwest::{async_http_client, Error as OAuthError},
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, RedirectUrl, RequestTokenError, Scope, StandardErrorResponse,
    StandardTokenResponse, TokenResponse, TokenUrl,
};

use reqwest::Error as ReqwestError;
use url::Url;

use crate::{GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET};

type Token = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
type FailedToken =
    RequestTokenError<OAuthError<ReqwestError>, StandardErrorResponse<BasicErrorResponseType>>;
type TokenResult = Result<Token, FailedToken>;

const CREATE_NO_WINDOW: u32 = 0x08000000;

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

#[derive(Serialize)]
struct VideoData {
    snippet: VideoSnippet,
    status: VideoStatus,
}

#[traceback_derive::traceback]
pub async fn handler(
    reciever: Arc<Mutex<tokio::sync::mpsc::Receiver<PathBuf>>>,
) -> Result<(), TracebackError> {
    traceback!();
    let username = whoami::username();
    traceback!();
    let entry = keyring::Entry::new("AUTOMATIC_YOUTUBE_UPLOAD", &username)?;
    traceback!();
    match entry.get_password() {
        Ok(pass) => pass,
        // token has not yet been set
        Err(outer_e) => match get_token().await {
            Ok(token) => {
                traceback!();
                let _ = entry.set_password(&token);
                token
            }
            Err(e) => {
                traceback!();
                return Err(traceback!(err e).with_extra_data(json!({
                    "outer": outer_e.to_string()
                })));
            }
        },
    };
    traceback!();
    let mut rx = reciever.lock().await;
    let mut uploaded_paths = vec![];
    while let Some(path) = rx.recv().await {
        traceback!();
        if !uploaded_paths.contains(&path) {
            initilize_upload(&path).await?;
            uploaded_paths.push(path);
        }
    }
    traceback!();
    return Err(traceback!("Exited handler"));
}

#[traceback_derive::traceback]
pub async fn initilize_upload(path: &PathBuf) -> Result<(), TracebackError> {
    traceback!(format!("Uploading {path:?}"));
    let username = whoami::username();
    let entry = keyring::Entry::new("AUTOMATIC_YOUTUBE_UPLOAD", &username)?;
    let token = match entry.get_password() {
        Ok(pass) => pass,
        // token has not yet been set
        Err(outer_e) => match get_token().await {
            Ok(token) => {
                let _ = entry.set_password(&token);
                token
            }
            Err(e) => {
                return Err(traceback!(err e).with_extra_data(json!({
                    "outer": outer_e.to_string()
                })))
            }
        },
    };
    // Read video metadata and file path from command line arguments or configuration
    let video_title = Utc::now().naive_local();
    let video_description = "";
    let video_tags = vec![];
    let video_category_id = "22";
    let snippet = VideoSnippet {
        title: video_title.to_string(),
        description: video_description.to_string(),
        tags: video_tags,
        category_id: video_category_id.to_string(),
    };

    let status = VideoStatus {
        privacy_status: "private".to_string(),
    };

    let vdata = VideoData { snippet, status };

    let video_url = upload_video(path.to_str().unwrap(), &vdata, &token).await?;

    traceback!(format!(
        "Video uploaded successfully. Video URL: {}",
        video_url
    ));

    Ok(())
}

#[traceback_derive::traceback]
async fn upload_video(
    file_path: &str,
    vdata: &VideoData,
    token: &str,
) -> Result<String, TracebackError> {
    let http_client = reqwest::Client::new();

    // Open and read the video file
    let mut video_file = tokio::fs::File::open(file_path).await?;
    let mut video_data = Vec::new();
    video_file.read_to_end(&mut video_data).await?;

    // Create a multipart form for video upload
    let form = reqwest::multipart::Form::new()
        .text("part", serde_json::to_string(vdata)?)
        .part(
            "media",
            multipart::Part::bytes(video_data)
                .file_name(file_path.to_string())
                .mime_str("video/mp4")?,
        );

    // Upload the video using the YouTube API
    let response = http_client
        .post("https://www.googleapis.com/upload/youtube/v3/videos")
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
    traceback!(format!("{:?}", &response_data));
    let video_id = response_data["id"]
        .as_str()
        .ok_or("Video ID not found in the response")?;

    // Wait for a moment to ensure the video is processed
    sleep(Duration::from_secs(10));

    // Generate the video URL
    let video_url = format!("https://www.youtube.com/watch?v={}", video_id);

    Ok(video_url)
}

fn wrap_in_quotes<T: AsRef<OsStr>>(path: T) -> OsString {
    let mut result = OsString::from("\"");
    result.push(path);
    result.push("\"");

    result
}

#[traceback_derive::traceback]
async fn get_token() -> Result<String, TracebackError> {
    traceback!(format!("No token found; creating token"));
    let client = BasicClient::new(
        ClientId::new(GOOGLE_CLIENT_ID.to_string()),
        Some(ClientSecret::new(GOOGLE_CLIENT_SECRET.to_string())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?,
        Some(TokenUrl::new(
            "https://oauth2.googleapis.com/token".to_string(),
        )?),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new("http://localhost:13425".to_string())?);

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/youtube.upload".to_string(),
        ))
        .add_scope(Scope::new("openid".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    let mut token_response: Option<TokenResult> = None;

    let mut command = Command::new("cmd");
    command
        .arg("/C")
        .arg("start")
        .raw_arg("\"\"")
        .raw_arg(wrap_in_quotes(auth_url.to_string()))
        .creation_flags(CREATE_NO_WINDOW);

    match command.spawn() {
        Ok(_) => {}
        Err(e) => {
            traceback!(format!(
                "Failed to open browser, please browse to: {}",
                auth_url
            ));
            traceback!(err e);
        }
    }
    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:13425")?;
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line)?;

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url))?;

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes())?;

            // Exchange the code with a token.
            token_response = Some(
                client
                    .exchange_code(code)
                    .set_pkce_verifier(pkce_verifier)
                    .request_async(async_http_client)
                    .await,
            );

            // The server will terminate itself after revoking the token.
            break;
        }
    }

    let token = token_response.unwrap()?.access_token().secret().to_string();
    Ok(token)
}
