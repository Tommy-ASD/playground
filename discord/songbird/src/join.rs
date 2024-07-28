use songbird::events::TrackEvent;

use crate::{Context, Error, TrackErrorNotifier};

#[poise::command(slash_command, prefix_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
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
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    leave_inner(&ctx).await
}

async fn leave_inner(ctx: &Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild().unwrap().id;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    ctx.reply("Left vc").await.unwrap();

    if let Err(_e) = manager.leave(guild_id).await {}

    Ok(())
}