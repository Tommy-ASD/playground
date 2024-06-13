use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use async_trait::async_trait;
use chrono::Utc;
use dashmap::DashMap;

use songbird::{
    driver::DecodeMode,
    model::{
        id::UserId,
        payload::{ClientDisconnect, Speaking},
    },
    packet::Packet,
    Config,
    CoreEvent,
    Event,
    EventContext,
    EventHandler as VoiceEventHandler,
    SerenityInit,
};

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn deafen(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            ctx.reply("Not in a voice channel").await;

            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        ctx.reply("Already deafened").await;
    } else {
        if let Err(e) = handler.deafen(true).await {
            ctx.reply(format!("Failed: {:?}", e)).await;
        }

        ctx.reply("Deafened").await;
    }

    Ok(())
}


#[poise::command(slash_command, prefix_command)]
pub async fn undeafen(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            ctx.reply("Not in a voice channel").await;

            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        ctx.reply("Not deafened").await;
    } else {
        if let Err(e) = handler.deafen(true).await {
            ctx.reply(format!("Failed: {:?}", e)).await;
        }

        ctx.reply("Undefeaned").await;
    }

    Ok(())
}