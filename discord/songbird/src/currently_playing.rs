use crate::{Context, Error, LoopState, PauseState};

#[poise::command(slash_command, prefix_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
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
pub async fn toggle_loop(ctx: Context<'_>) -> Result<(), Error> {
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
pub async fn rewind(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    if let Some(guild_lock) = ctx.data().guilds.lock().await.get(&guild_id) {
        dbg!();
        println!("Skip song now: {:?}", std::time::Instant::now());
        let mut data = guild_lock.lock().await;
        dbg!();
        if let Some(song) = &data.current_song {
        } else {
            ctx.reply("No song currently playing").await.unwrap();
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
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