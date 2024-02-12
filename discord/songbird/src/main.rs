use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
use std::env;
use std::sync::atomic::{AtomicU32, Ordering};

use songbird::SerenityInit;

use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};

use songbird::input::YoutubeDl;

use reqwest::{Client as HttpClient, Url};

use serenity::{
    async_trait,
    client::{Client, EventHandler},
    model::{channel::Message, gateway::Ready},
    prelude::{GatewayIntents, TypeMapKey},
    Result as SerenityResult,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    poise_mentions: AtomicU32,
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

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut prefix = PrefixFrameworkOptions::default();
    prefix.prefix = Some("!".to_string());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            commands: vec![age(), join(), play() /*test()*/],
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
                })
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .register_songbird()
        // We insert our own HTTP client here to make use of in
        // `~play`. If we wanted, we could supply cookies and auth
        // details ahead of time.
        //
        // Generally, we don't want to make a new Client for every request!
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
    // println!(
    //     "Got age command by {author} for {user}",
    //     author = ctx.author().name,
    //     user = u.name
    // );
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await.unwrap();
    Ok(())
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

// #[poise::command(slash_command, prefix_command)]
// async fn test(ctx: Context<'_>) -> Result<(), Error> {
//     println!("{}", ctx.id());
//     let msg = ctx
//         .channel_id()
//         .message(ctx.http(), ctx.id())
//         .await
//         .unwrap();
//     let manager = songbird::get(ctx.serenity_context())
//         .await
//         .expect("Songbird Voice client placed in at initialisation.")
//         .clone();
//     for attachment in msg.attachments {
//         println!("A URL: {}", attachment.url);
//         println!("P URL: {}", attachment.proxy_url);
//         println!("CT: {:?}", attachment.content_type);
//         println!("A: {:?}", attachment);
//         let b = reqwest::get(attachment.proxy_url)
//             .await
//             .unwrap()
//             .bytes()
//             .await
//             .unwrap();
//         if let Some(handler_lock) = manager.get(ctx.guild_id().unwrap()) {
//             let mut handler = handler_lock.lock().await;

//             let _ = handler.play_input(b.into());

//             check_msg(ctx.channel_id().say(&ctx.http(), "Playing song").await);
//         } else {
//             check_msg(
//                 ctx.channel_id()
//                     .say(&ctx.http(), "Not in a voice channel to play in")
//                     .await,
//             );
//         }
//     }

//     Ok(())
// }

#[poise::command(slash_command, prefix_command)]
async fn play(
    ctx: Context<'_>,
    #[description = "Song URL (song search will be implemented at later point)"] song: String,
) -> Result<(), Error> {
    play_inner(&ctx, &song).await
}

async fn play_inner(ctx: &Context<'_>, url: &str) -> Result<(), Error> {
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

        if url.starts_with("https://youtu.be") {
            let src = YoutubeDl::new(http_client, url.to_string());
            let _ = handler.play_input(src.clone().into());
            check_msg(ctx.channel_id().say(&ctx.http(), "Playing song").await);
        } else {
            let req = match reqwest::get(url).await {
                Ok(req) => match req.bytes().await {
                    Ok(b) => {
                        let _ = handler.play(b.into());
                        check_msg(ctx.channel_id().say(&ctx.http(), "Playing song").await);
                    }
                    Err(e) => {
                        check_msg(ctx.channel_id().say(&ctx.http(), format!("Failed to get bytestream; Maybe URL does not point directly to the file? Exact error for debugging purposes; {e}")).await);
                    }
                },
                Err(e) => {
                    check_msg(
                        ctx.channel_id()
                            .say(&ctx.http(), format!("Did not get a response from URL. Exact error for debugging purposes; {e}"))
                            .await,
                    );
                }
            };
        }
    } else {
        check_msg(
            ctx.channel_id()
                .say(&ctx.http(), "Not in a voice channel to play in")
                .await,
        );
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
    }

    Ok(())
}
