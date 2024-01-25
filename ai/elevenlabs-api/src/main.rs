use std::io::Write;

use reqwest::Url;
use serde_json::json;

#[tokio::main]
async fn main() {
    let key = std::env::var("ELEVENLABS_API_KEY").unwrap();
    println!("Key; {key}");

    let vid = "bVMeCyTHy58xNoL34h3p";

    let url = Url::parse(&format!(
        "https://api.elevenlabs.io/v1/text-to-speech/{vid}"
    ))
    .unwrap();

    let body = json!({
      "model_id": "eleven_multilingual_v1",
      "text": "This is pretty cool and awesome!!!",
      "voice_settings": {
        "similarity_boost": 0.75,
        "stability": 0.5,
        "style": 1,
        "use_speaker_boost": true
      }
    });

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .body(body.to_string())
        .header("xi-api-key", key)
        .send()
        .await
        .unwrap();
    let bytes = res.bytes().await.unwrap();
    let mut file = std::fs::File::create("./aaa.wav").unwrap();
    file.write_all(&bytes).unwrap();
}
