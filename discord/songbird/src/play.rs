use poise::{
    serenity_prelude::{self as serenity, CacheHttp, ChannelId, Http},
    PrefixFrameworkOptions,
};
use tokio::sync::{Mutex, MutexGuard};
use std::{
    env,
    sync::{
        atomic::{AtomicU32, AtomicUsize, Ordering},
        Arc,
    },
};
use songbird::{
    events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent},
    input::{Input, YoutubeDl},
    Call, SerenityInit,
};
use reqwest::{Client as HttpClient, Url};
use serenity::{
    async_trait,
    client::Client,
    prelude::{GatewayIntents, TypeMapKey},
};
use crate::{Context, Error, GuildData, HttpKey};

#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song URL (song search will be implemented at later point)"] song: String,
) -> Result<(), Error> {
    let url = match Url::parse(&song) {
        Ok(url) => url,
        Err(e) => {
            ctx.reply(format!("{song} is not a valid URL: {e}")).await?;
            return Err(e.into());
        }
    };
    play_inner(&ctx, &url).await
}

async fn play_inner(ctx: &Context<'_>, url: &Url) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let http_client = {
        let data = ctx.serenity_context().data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .expect("Guaranteed to exist in the typemap.")
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        if ["youtu.be", "www.youtu.be", "youtube.com", "www.youtube.com"]
            .contains(&url.host_str().unwrap_or(""))
        {
            let src = YoutubeDl::new(http_client, url.to_string());
            let guild_data_arc = Arc::clone(ctx.data().guilds.get(&guild_id).unwrap());
            let mut guild_data = guild_data_arc.lock().await;
            if guild_data.current_song.is_none() {
                play_song(ctx.channel_id(), ctx.serenity_context().http.clone(), handler_lock, src.clone().into(), Arc::clone(&guild_data_arc)).await;
                ctx.reply("Playing song").await?;
            } else {
                guild_data.queue.push_back(src.clone().into());
                ctx.reply("Added song to queue").await?;
            }
        } else {
            let mut handler = handler_lock.lock().await;
            match reqwest::get(url.as_str()).await {
                Ok(req) => match req.bytes().await {
                    Ok(b) => {
                        handler.play(b.into());
                        ctx.reply("Playing song").await?;
                    }
                    Err(e) => {
                        ctx.reply(format!("Failed to get bytestream; Maybe URL does not point directly to the file? Exact error for debugging purposes: {e}")).await?;
                    }
                },
                Err(e) => {
                    ctx.reply(format!("Did not get a response from URL. Exact error for debugging purposes: {e}")).await?;
                }
            }
        }
    } else {
        ctx.reply("Not in a voice channel to play in").await?;
    }

    Ok(())
}

async fn play_song(
    chan_id: ChannelId,
    serenity_context: Arc<Http>,
    handler_lock: Arc<Mutex<Call>>,
    src: Input,
    guild_data_arc: Arc<Mutex<GuildData>>,
) {
    let mut handler = handler_lock.lock().await;
    let thandle = handler.play_only_input(src);
    thandle.add_event(
        Event::Track(TrackEvent::End),
        SongEndNotifier {
            chan_id,
            http: serenity_context,
            handler_lock: Arc::clone(&handler_lock),
            guild_data: Arc::clone(&guild_data_arc),
        },
    );
    let mut guild_data = guild_data_arc.lock().await;
    guild_data.current_song = Some(thandle);
}

struct SongEndNotifier {
    chan_id: ChannelId,
    http: Arc<Http>,
    handler_lock: Arc<Mutex<Call>>,
    guild_data: Arc<Mutex<GuildData>>,
}

#[async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let mut guild_data = self.guild_data.lock().await;
        guild_data.current_song = None;

        if let Some(next) = guild_data.queue.pop_front() {
            play_song(self.chan_id, Arc::clone(&self.http), Arc::clone(&self.handler_lock), next, Arc::clone(&self.guild_data)).await;
        }

        None
    }
}
