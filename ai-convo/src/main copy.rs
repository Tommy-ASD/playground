
use chrono::NaiveDateTime;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio;
use utils::input;


#[derive(Serialize, Deserialize, Debug)]
enum Model {
    #[serde(rename="llama3")]
    Llama3,
}

#[derive(Serialize, Deserialize, Debug)]
struct Personality {
    model: Model,
    system: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaResponse {
    model: Model,
    created_at: String,
    response: String,
    done: bool,
    done_reason: String,
    context: Vec<u128>,
    total_duration: u64,
    load_duration: u64,
    prompt_eval_count: u64,
    prompt_eval_duration: u64,
    eval_count: u64,
    eval_duration: u64
}

#[derive(Serialize, Deserialize, Debug)]
enum Role {
    System,
    User,
    Assistant
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: Role,
    content: String
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let trump = Personality {
        model: Model::Llama3,
        // system: "You are a not-very-helpful assistant, and always reply in the style of evangelical Donald Trump, talking as if he's a biblical character, but also using fancy old english and making responses unnecessarily long to sound smart.".to_string()
        system: "You are a helpful assistant".to_string()
    };
    // Create a reqwest client
    let client = Client::new();

    let mut context = vec![];
    loop {
        let message = input!("Write your message");

        trump.send_message(&mut context, &message, &client).await;
    }
    Ok(())
}

impl Personality {
    async fn send_message(&self, context: &mut Vec<u128>, prompt: &str, client: &Client) {
        // Build the request body
        let body = json!({
            "model": self.model,
            "prompt": prompt,
            "context": context,
            "system": self.system,
            "stream": false
        });

        // Make the POST request
        let response = client
            .post("https://ollama.tommyasd.com/api/generate")
            .json(&body)  // Send JSON body
            .send()
            .await.unwrap();
        
        // Check if the request was successful and print the response body
        if response.status().is_success() {
            // let body_text = response.text().await?;
            // println!("Response: {}", body_text);
            let body_object: OllamaResponse = response.json().await.unwrap();
            println!("{}", body_object.response);
            *context = body_object.context;
        } else {
            println!("Request failed with status: {}", response.status());
        }
    }
}
