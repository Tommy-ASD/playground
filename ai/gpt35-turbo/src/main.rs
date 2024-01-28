use reqwest::{self, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{io::BufReader, path::PathBuf};
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Debug, Serialize, Deserialize)]
struct ChatGPTRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatGPTResponse {
    #[serde(default)]
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseMessage {
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Set your ChatGPT API key
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();

    // Set the API endpoint
    let api_url = "https://api.openai.com/v1/chat/completions";

    // Create a new ChatGPTRequest with the conversation
    let mut request = ChatGPTRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
            role: "system".to_string(),
            content: "You are a helpful assistant.".to_string(),
        }],
    };

    loop {
        let message = input!("> ");

        request.messages.push(Message {
            role: "user".to_string(),
            content: message.to_string(),
        });

        // Make a POST request to the ChatGPT API
        let client = reqwest::Client::new();
        let response = client
            .post(api_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request)
            .send()
            .await
            .unwrap();
        // println!("Response: {response:?}");
        let parsed: Value = response.json().await.unwrap();

        // println!("Parsed: {parsed:?}");

        let parsed: ChatGPTResponse = serde_json::from_value(parsed).unwrap();

        // Extract and print the response message
        let reply = parsed
            .choices
            .get(0)
            .map(|choice| &choice.message.content)
            .unwrap();
        println!("ChatGPT response: {}", reply);

        request.messages.push(Message {
            role: "system".to_string(),
            content: reply.to_string(),
        });

        let path = "./aaa.mp3".into();

        elevenlabs(&reply, &path).await;

        playback_audio(&path).await;
    }
}

async fn elevenlabs(text: &str, output: &PathBuf) {
    let key = std::env::var("ELEVENLABS_API_KEY").unwrap();
    // println!("Key; {key}");

    let vid = "bVMeCyTHy58xNoL34h3p";

    let url = Url::parse(&format!(
        "https://api.elevenlabs.io/v1/text-to-speech/{vid}"
    ))
    .unwrap();

    let body = json!({
      "model_id": "eleven_multilingual_v1",
      "text": text,
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
    let mut file = tokio::fs::File::create(output).await.unwrap();
    file.write_all(&bytes).await.unwrap();
}

async fn playback_audio(path: &PathBuf) {
    use rodio::{source::Source, Decoder, OutputStream};

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).await.unwrap().into_std().await);
    // println!("Opened file, decoding");

    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    // println!("Sending bytes to output");
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).unwrap();

    let duration = mp3_duration::from_path(path).unwrap();

    // println!("Duration; {duration:?}");

    // println!("Finished playing back!");
    tokio::time::sleep(duration).await;
}

/*
This function is simply the input() function from Python, as getting input is a bit annoying in Rust.
*/
pub fn input_inner() -> String {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => {}
        Err(e) => {
            println!("Error reading line: {}", e);
            println!("Please try again");
            return input_inner();
        }
    }
    line.trim().to_string()
}

/// Simulates the behavior of Python's `input()` function to capture user input.
///
/// The `input!` macro simplifies the process of capturing user input from the standard input (stdin).
/// It takes one or more prompts (strings) as arguments, displays them to the user, and then waits for user input.
///
/// # Usage
/// ```rust
/// fn main() {
///     let name: String = utils::input!("Enter your name: ");
///     println!("Hello, {}!", name);
/// }
/// ```
///
/// In this example, the `input!` macro displays the "Enter your name: " prompt and waits for the user to enter their name.
/// The entered value is stored in the `name` variable.
///
/// # Returns
///
/// - `String` - The user's input as a string, with leading and trailing whitespace removed.
#[macro_export]
macro_rules! input {
    ($($arg:expr),*) => {{
        $(print!("{} ", $arg);)* // Print each argument followed by a space
        println!(); // Print a newline at the end

        $crate::input_inner()
    }};
}
