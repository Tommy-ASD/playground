use poise::serenity_prelude::{self as serenity, ChannelId, Http};
use tokio::sync::Mutex;
use std::sync::Arc;
use songbird::{
    events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent},
    input::{Compose, Input, YoutubeDl},
    Call,
};
use reqwest::Url;
use serenity::async_trait;
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
    let guild_data_arc = Arc::clone(ctx.data().guilds.lock().await.entry(guild_id).or_insert_with(|| Arc::new(Mutex::new(GuildData::default()))));

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
            let mut src = YoutubeDl::new(http_client, url.to_string());
            src.aux_metadata().await.map(|metadata| metadata.title.map(|title| println!("Playing {title}")));
            let mut guild_data = guild_data_arc.lock().await;
            if guild_data.current_song.is_none() {
                dbg!();
                drop(guild_data);
                play_song(ctx.channel_id(), ctx.serenity_context().http.clone(), handler_lock, src.clone().into(), Arc::clone(&guild_data_arc)).await;
                dbg!();
                ctx.reply("Playing song").await?;
            } else {
                dbg!();
                println!("Queue song now: {:?}", std::time::Instant::now());
                guild_data.queue.push_back(src.clone().into());
                drop(guild_data);
                dbg!();
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
    dbg!();
    println!("Play song now: {:?}", std::time::Instant::now());
    let mut handler = handler_lock.lock().await;
    let thandle = handler.play_only_input(src);
    dbg!();
    thandle.add_event(
        Event::Track(TrackEvent::End),
        SongEndNotifier {
            chan_id,
            http: serenity_context,
            handler_lock: Arc::clone(&handler_lock),
            guild_data: Arc::clone(&guild_data_arc),
        },
    );
    dbg!();
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
        dbg!();
        println!("End song now: {:?}", std::time::Instant::now());
        let mut guild_data = self.guild_data.lock().await;
        guild_data.current_song = None;

        if let Some(next) = guild_data.queue.pop_front() {
            drop(guild_data);
            dbg!();
            play_song(self.chan_id, Arc::clone(&self.http), Arc::clone(&self.handler_lock), next, Arc::clone(&self.guild_data)).await;
        }
        dbg!();

        None
    }
}
