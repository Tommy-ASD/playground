use minesweeper::PlayType;
use poise::serenity_prelude::{CacheHttp, Message};
use poise::serenity_prelude as serenity;
use crate::UserData;

pub mod minesweeper;
pub mod four_in_a_row;

pub async fn handle_games_message(
    ctx: &serenity::Context,
    user_data: &mut UserData,
    new_message: &Message
) {
    if let Some(ms) = user_data.minesweeper.as_mut() {
        if ms.origin_channel_id == new_message.channel_id {
            let mut split = new_message.content.split(" ");
            let y = split.next().unwrap().parse::<usize>().unwrap() - 1;
            let x = split.next().unwrap().parse::<usize>().unwrap() - 1;
            let flag = split.next().and_then(|f| Some(if f.starts_with("f") {
                PlayType::Flag
            } else {
                PlayType::Press
            })).unwrap_or(PlayType::Press);
            
            ms.board.play((y, x), flag);
            new_message.reply(ctx.http(), ms.board.to_emojis()).await.unwrap();
        };
    };
    if let Some(fiar) = user_data.four_in_a_row.as_mut() {
        if fiar.origin_channel_id == new_message.channel_id {
            let mut split = new_message.content.split(" ");
            let y = split.next().unwrap().parse::<usize>().unwrap() - 1;
            fiar.board.play(y);
            new_message.reply(ctx.http(), fiar.board.to_emojis()).await.unwrap();
        }
    }
}