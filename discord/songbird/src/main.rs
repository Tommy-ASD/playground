use poise::serenity_prelude::GuildId;
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

use crate::{play::play, deafen::{deafen, undeafen}};

type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    poise_mentions: AtomicU32,
    guilds: Mutex<HashMap<GuildId, Arc<Mutex<GuildData>>>>
}

#[derive(Default)]
enum LoopState {
    #[default]
    NoLoop,
    LoopSong,
}

#[derive(Default)]
enum PauseState {
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

struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            // let Message { content, .. } = new_message;
            if new_message.content.to_lowercase().contains("poise") {
                let mentions = data.poise_mentions.load(Ordering::SeqCst) + 1;
                data.poise_mentions.store(mentions, Ordering::SeqCst);
                new_message
                    .reply(ctx, format!("Poise has been mentioned {} times", mentions))
                    .await?;
            }
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
            commands: vec![age(), join(), play(), skip(), leave(), toggle_loop(), deafen(), undeafen(), pause()],
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
                    guilds: Mutex::new(HashMap::new())
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

#[poise::command(slash_command, prefix_command)]
async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    if let Some(guild_lock) = ctx.data().guilds.lock().await.get(&guild_id) {
        dbg!();
        println!("Skip song now: {:?}", std::time::Instant::now());
        let data = guild_lock.lock().await;
        dbg!();
        if let Some(song) = &data.current_song {
            song.stop();
            ctx.reply("Skipped").await.unwrap();
        } else {
            ctx.reply("No song currently playing").await.unwrap();
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn toggle_loop(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    if let Some(guild_lock) = ctx.data().guilds.lock().await.get(&guild_id) {
        dbg!();
        println!("Skip song now: {:?}", std::time::Instant::now());
        let mut data = guild_lock.lock().await;
        dbg!();
        let new_loop_state;
        if let Some(song) = &data.current_song {
            match data.loop_state {
                LoopState::NoLoop => {
                    new_loop_state = LoopState::LoopSong;
                    song.enable_loop();
                    ctx.reply("Enabled loop").await.unwrap();
                }
                LoopState::LoopSong => {
                    new_loop_state = LoopState::NoLoop;
                    song.disable_loop();
                    ctx.reply("Disabled loop").await.unwrap();
                }
            }
        } else {
            new_loop_state = LoopState::NoLoop;
            ctx.reply("No song currently playing").await.unwrap();
        }
        data.loop_state = new_loop_state;
        
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn rewind(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    if let Some(guild_lock) = ctx.data().guilds.lock().await.get(&guild_id) {
        dbg!();
        println!("Skip song now: {:?}", std::time::Instant::now());
        let mut data = guild_lock.lock().await;
        dbg!();
        let new_loop_state;
        if let Some(song) = &data.current_song {
                song.enable_loop();
                song.stop();
        } else {
            new_loop_state = LoopState::NoLoop;
            ctx.reply("No song currently playing").await.unwrap();
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    if let Some(guild_lock) = ctx.data().guilds.lock().await.get(&guild_id) {
        dbg!();
        println!("Skip song now: {:?}", std::time::Instant::now());
        let mut data = guild_lock.lock().await;
        dbg!();
        let new_pause_state;
        if let Some(song) = &data.current_song {
            match data.pause_state {
                PauseState::Playing => {
                    new_pause_state = PauseState::Paused;
                    song.pause();
                    ctx.reply("Paused song").await.unwrap();
                }
                PauseState::Paused => {
                    new_pause_state = PauseState::Playing;
                    song.play();
                    ctx.reply("Playing song").await.unwrap();
                }
            }
        } else {
            new_pause_state = PauseState::Playing;
            ctx.reply("No song currently playing").await.unwrap();
        }
        data.pause_state = new_pause_state;
    }

    Ok(())
}

struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}

#[poise::command(slash_command, prefix_command)]
async fn join(ctx: Context<'_>) -> Result<(), Error> {
    join_inner(&ctx).await
}

async fn join_inner(ctx: &Context<'_>) -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.reply("Not in a voice channel").await.unwrap();

            return Ok(());
        }
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        // Attach an event handler to see notifications of all track errors.
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);

        // let evt_receiver = Receiver::new();
    
        // handler.add_global_event(CoreEvent::SpeakingStateUpdate.into(), evt_receiver.clone());
        // handler.add_global_event(CoreEvent::RtpPacket.into(), evt_receiver.clone());
        // handler.add_global_event(CoreEvent::RtcpPacket.into(), evt_receiver.clone());
        // handler.add_global_event(CoreEvent::ClientDisconnect.into(), evt_receiver.clone());
        // handler.add_global_event(CoreEvent::VoiceTick.into(), evt_receiver);

        ctx.reply("Joined").await.unwrap();
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    leave_inner(&ctx).await
}

async fn leave_inner(ctx: &Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild().unwrap().id;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    ctx.reply("Left vc").await;

    if let Err(e) = manager.leave(guild_id).await {}

    Ok(())
}