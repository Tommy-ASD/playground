use serde::{Deserialize, Serialize};
use poise::serenity_prelude::{Channel, ChannelId, Context, GetMessages, GuildChannel, GuildId, Message, MessageId, UserId};
use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
// use receive::Receiver;
use songbird::tracks::TrackHandle;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::broadcast::Sender;
use tokio::sync::Mutex;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use songbird::SerenityInit;

use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler};

use songbird::input::Input;

use reqwest::Client as HttpClient;
use poise::serenity_prelude::MessageReaction;
use serenity::{
    async_trait,
    client::Client,
    prelude::{GatewayIntents, TypeMapKey},
};

#[derive(Serialize, Deserialize, Debug)]
struct FormattedMessage {
    author: String,
    user_id: u64,
    timestamp: String,
    message: String,
    reactions: Vec<MessageReaction>,
    reply_to: Option<Box<poise::serenity_prelude::Message>>,
    message_id: MessageId,
}

pub async fn fetch_messages(ctx: &Context, channel: &GuildChannel) {
    let mut all_messages = Vec::new();
    // Last message is used to track the earliest fetched message
    // so we can later use GetMessages::new().before(last_message)
    let mut last_message = None;

    // Fetch messages from the channel
    let messages = match channel.messages(&ctx.http, GetMessages::new().limit(100)).await {
        Ok(messages) => {
            messages
        }
        Err(err) => {
            println!("Failed to fetch messages for channel {}: {:?}", channel.id, err);
            return;
        }
    };

    let mut latest_fetch_amount = messages.len();

    println!("Fetched {} messages from channel {}", messages.len(), channel.id);

    for message in messages {
        // println!("Message: {message:?}");
        println!("Message: {ts:?}", ts = message.timestamp);
        println!("Id: {ts:?}", ts = message.id);
        // set last_message to the earliest fetched message
        last_message = Some(message.id);

        all_messages.push(message);
    }

    while latest_fetch_amount == 100 {
        // if last_message is none, the channel history is empty
        let some_last_message = match last_message {
            Some(lm) => lm,
            None => break,
        };
        // Fetch messages from the channel
        let messages = match channel.messages(&ctx.http, GetMessages::new().before(some_last_message).limit(100)).await {
            Ok(messages) => {
                messages
            }
            Err(err) => {
                println!("Failed to fetch messages for channel {}: {:?}", channel.id, err);
                return;
            }
        };
        latest_fetch_amount = messages.len();
        println!("Fetched {} messages from channel {}", messages.len(), channel.id);

        for message in messages {
            // println!("Message: {message:?}");
            println!("Message: {ts:?}", ts = message.timestamp);
            println!("Id: {ts:?}", ts = message.id);
            last_message = Some(message.id);
    
            all_messages.push(message);
        }
    }
    

    // Save the messages to a file
    save_to_file(all_messages, &channel.id).await;
}

async fn save_to_file(messages: Vec<Message>, channel_id: &ChannelId) {
    // Open (or create) the file
    let mut file = File::create(format!("messages/discord_messages.{channel_id}.json")).await.unwrap();

    // Convert messages to JSON format
    let json_data = serde_json::to_string_pretty(&messages).unwrap();

    // Write the JSON data to the file
    file.write_all(json_data.as_bytes()).await.unwrap();

    println!("Messages saved to discord_messages.{channel_id}.json");
}
