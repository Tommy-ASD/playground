use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
use std::{sync::atomic::{AtomicU32, Ordering}, env};

use songbird::{events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent}, input::YoutubeDl, SerenityInit};

use reqwest::{Client as HttpClient, Url};

use serenity::{
    async_trait,
    client::Client,
    prelude::{GatewayIntents, TypeMapKey},
};

use crate::{Context, Error, HttpKey};


#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song URL (song search will be implemented at later point)"] song: String,
) -> Result<(), Error> {
    let url = match Url::parse(&song) {
        Ok(url) => url,
        Err(e) => {
            ctx.reply(format!("{song} is not a valid URL: {e}")).await;
            return Err(e.into());
        }
    };
    play_inner(&ctx, &url).await
}

async fn play_inner(ctx: &Context<'_>, url: &Url) -> Result<(), Error> {
    let opt_msg = ctx.channel_id().message(ctx.http(), ctx.id()).await;
    if let Ok(msg) = opt_msg {}

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
        let mut handler = handler_lock.lock().await;

        if url.host_str() == Some("youtu.be")
            || url.host_str() == Some("www.youtu.be")
            || url.host_str() == Some("youtube.com")
            || url.host_str() == Some("www.youtube.com")
        {
            let id = url.query();
            dbg!();
            println!("Id: {id:?}");

            let src = YoutubeDl::new(http_client, url.to_string());
            let thandle = handler.play_only_input(src.clone().into());
            ctx.reply("Playing song").await;
        } else {
            match reqwest::get(url.as_str()).await {
                Ok(req) => match req.bytes().await {
                    Ok(b) => {
                        let _ = handler.play(b.into());
                        ctx.reply("Playing song").await;
                    }
                    Err(e) => {
                        ctx.reply( format!("Failed to get bytestream; Maybe URL does not point directly to the file? Exact error for debugging purposes; {e}")).await;
                    }
                },
                Err(e) => {
                    ctx.reply(format!(
                        "Did not get a response from URL. Exact error for debugging purposes; {e}"
                    ))
                    .await;
                }
            };
        }
    } else {
        ctx.reply("Not in a voice channel to play in").await;
    }

    Ok(())
}