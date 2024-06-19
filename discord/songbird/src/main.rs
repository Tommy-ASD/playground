use games::{four_in_a_row, handle_games_message};
use games::minesweeper::{self, minesweeper, PlayType};
use poise::serenity_prelude::{CacheHttp, ChannelId, GuildId, UserId};
use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
// use receive::Receiver;
use songbird::tracks::TrackHandle;
use tokio::sync::Mutex;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use songbird::{CoreEvent, SerenityInit};

use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};

use songbird::input::Input;

use reqwest::Client as HttpClient;

use serenity::{
    async_trait,
    client::Client,
    prelude::{GatewayIntents, TypeMapKey},
};

mod play;
mod receive;
mod deafen;
mod rtp_stream;
mod currently_playing;
mod join;
mod games;

use crate::{play::play, deafen::{deafen, undeafen}, currently_playing::{skip, toggle_loop, pause}, join::{join, leave}};

type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    poise_mentions: AtomicU32,
    guilds: Mutex<HashMap<GuildId, Arc<Mutex<GuildData>>>>,
    users: Mutex<HashMap<UserId, Arc<Mutex<UserData>>>>
}

#[derive(Default)]
pub enum LoopState {
    #[default]
    NoLoop,
    LoopSong,
}

#[derive(Default)]
pub enum PauseState {
    #[default]
    Playing,
    Paused,
}

#[derive(Default)]
pub struct GuildData {
    pub queue: VecDeque<Input>,
    pub current_song: Option<TrackHandle>,
    pub loop_state: LoopState,
    pub pause_state: PauseState,
}

#[derive(Default)]
pub struct MinesweeperManager {
    pub board: minesweeper::Board,
    pub origin_channel_id: ChannelId,
}

#[derive(Default)]
pub struct FourInARowManager {
    pub board: four_in_a_row::Board,
    pub origin_channel_id: ChannelId,
}

#[derive(Default)]
pub struct UserData {
    pub minesweeper: Option<MinesweeperManager>,
    pub four_in_a_row: Option<FourInARowManager>
}

struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            let mut users_lock = framework.user_data.users.lock().await;
            let udata = users_lock.entry(new_message.author.id).or_insert_with(|| Arc::new(Mutex::new(UserData::default())));
            let user_data_arc = Arc::clone(udata);
            // let Message { content, .. } = new_message;
            if new_message.content.to_lowercase().contains("poise") {
                let mentions = data.poise_mentions.load(Ordering::SeqCst) + 1;
                data.poise_mentions.store(mentions, Ordering::SeqCst);
                new_message
                    .reply(ctx, format!("Poise has been mentioned {} times", mentions))
                    .await?;
            }
            let mut user_data_lock = user_data_arc.lock().await;
            handle_games_message(ctx, &mut user_data_lock, new_message).await;
        }
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut prefix = PrefixFrameworkOptions::default();
    prefix.prefix = Some("!".to_string());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            commands: vec![age(), join(), play(), skip(), leave(), toggle_loop(), deafen(), undeafen(), pause(), minesweeper(), four_in_a_row::four_in_a_row()],
            prefix_options: prefix,
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                framework.options().commands.iter().for_each(|c| {
                    println!("{cname}", cname = c.name);
                });
                Ok(Data {
                    poise_mentions: AtomicU32::new(0),
                    guilds: Mutex::new(HashMap::new()),
                    users: Mutex::new(HashMap::new())
                })
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Err creating client");

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
    });

    let _signal_err = tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await.unwrap();
    Ok(())
}

struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::Track(track_list) => {
                for (state, handle) in *track_list {
                    println!(
                        "Track {:?} encountered an error: {:?}",
                        handle.uuid(),
                        state.playing
                    );
                }
            }
            _ => {}
        }

        None
    }
}