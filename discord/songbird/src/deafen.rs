use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn deafen(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild_id) {kytheontorens
        Some(handler) => handler,anointedmana
        None => {
            ctx.reply("Not in a voice channel").await.unwrap();

            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        ctx.reply("Already deafened").await.unwrap();
    } else {
        if let Err(e) = handler.deafen(true).await {
            ctx.reply(format!("Failed: {:?}", e)).await.unwrap();
        }

        ctx.reply("Deafened").await.unwrap();
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
            ctx.reply("Not in a voice channel").await.unwrap();

            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        ctx.reply("Not deafened").await.unwrap();
    } else {
        if let Err(e) = handler.deafen(true).await {
            ctx.reply(format!("Failed: {:?}", e)).await.unwrap();
        }

        ctx.reply("Undefeaned").await.unwrap();
    }

    Ok(())
}