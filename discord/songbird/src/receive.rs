//! Requires the "client", "standard_framework", and "voice" features be enabled
//! in your Cargo.toml, like so:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["client", "standard_framework", "voice"]
//! ```
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
    driver::DecodeMode, input::Input, model::{
        id::UserId,
        payload::{ClientDisconnect, Speaking},
    }, packet::Packet, Config, CoreEvent, Event, EventContext, EventHandler as VoiceEventHandler, SerenityInit
};


#[derive(Clone)]
pub struct Receiver {
    inner: Arc<InnerReceiver>,
}

pub struct InnerReceiver {
    last_tick_was_empty: AtomicBool,
    known_ssrcs: DashMap<u32, UserId>,
}

impl Receiver {
    pub fn new() -> Self {
        // You can manage state here, such as a buffer of audio packet bytes so
        // you can later store them in intervals.
        Self {
            inner: Arc::new(InnerReceiver {
                last_tick_was_empty: AtomicBool::default(),
                known_ssrcs: DashMap::new(),
            }),
        }
    }
}

#[async_trait]
impl VoiceEventHandler for Receiver {
    #[allow(unused_variables)]
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        use EventContext as Ctx;
        match ctx {
            Ctx::SpeakingStateUpdate(Speaking {
                speaking,
                ssrc,
                user_id,
                ..
            }) => {
                // Discord voice calls use RTP, where every sender uses a randomly allocated
                // *Synchronisation Source* (SSRC) to allow receivers to tell which audio
                // stream a received packet belongs to. As this number is not derived from
                // the sender's user_id, only Discord Voice Gateway messages like this one
                // inform us about which random SSRC a user has been allocated. Future voice
                // packets will contain *only* the SSRC.
                //
                // You can implement logic here so that you can differentiate users'
                // SSRCs and map the SSRC to the User ID and maintain this state.
                // Using this map, you can map the `ssrc` in `voice_packet`
                // to the user ID and handle their audio packets separately.
                println!(
                    "Speaking state update: user {:?} has SSRC {:?}, using {:?}",
                    user_id, ssrc, speaking,
                );

                if let Some(user) = user_id {
                    self.inner.known_ssrcs.insert(*ssrc, *user);
                }
            },
            Ctx::VoiceTick(tick) => {
                let speaking = tick.speaking.len();
                let total_participants = speaking + tick.silent.len();
                let last_tick_was_empty = self.inner.last_tick_was_empty.load(Ordering::SeqCst);

                if speaking == 0 && !last_tick_was_empty {
                    println!("No speakers");

                    self.inner.last_tick_was_empty.store(true, Ordering::SeqCst);
                } else if speaking != 0 {
                    self.inner
                        .last_tick_was_empty
                        .store(false, Ordering::SeqCst);

                    println!("Voice tick ({speaking}/{total_participants} live):");

                    // You can also examine tick.silent to see users who are present
                    // but haven't spoken in this tick.
                    for (ssrc, data) in &tick.speaking {
                        let user_id_str = if let Some(id) = self.inner.known_ssrcs.get(ssrc) {
                            format!("{:?}", *id)
                        } else {
                            "?".into()
                        };

                        // This field should *always* exist under DecodeMode::Decode.
                        // The `else` allows you to see how the other modes are affected.
                        if let Some(decoded_voice) = data.decoded_voice.as_ref() {
                            let voice_len = decoded_voice.len();
                            let audio_str = format!(
                                "first samples from {}: {:?}",
                                voice_len,
                                &decoded_voice[..voice_len.min(5)]
                            );

                            if let Some(packet) = &data.packet {
                                let rtp = packet.rtp();
                                println!(
                                    "\t{ssrc}/{user_id_str}: packet seq {} ts {} -- {audio_str}",
                                    rtp.get_sequence().0,
                                    rtp.get_timestamp().0
                                );
                            } else {
                                println!("\t{ssrc}/{user_id_str}: Missed packet -- {audio_str}");
                            }
                        } else {
                            println!("\t{ssrc}/{user_id_str}: Decode disabled.");
                        }
                    }
                }
            },
            Ctx::RtpPacket(packet) => {
                // An event which fires for every received audio packet,
                // containing the decoded data.
                let rtp = packet.rtp();
                dbg!();
                println!(
                    "Received voice packet from SSRC {}, sequence {}, timestamp {} -- {}B long",
                    rtp.get_ssrc(),
                    rtp.get_sequence().0,
                    rtp.get_timestamp().0,
                    rtp.payload().len()
                );
                dbg!();
                let pl = rtp.payload();
                let user_id_str = if let Some(id) = self.inner.known_ssrcs.get(&rtp.get_ssrc()) {
                    format!("{}", id.0)
                } else {
                    "?".into()
                };
                let now = chrono::offset::Local::now();
                dbg!();
                tokio::fs::write(format!("./files/{now}.{user_id_str}"), format!("{}: {sequence}", bytes_to_hexadecimal_string(pl), sequence = rtp.get_sequence().0)).await.unwrap();
                dbg!();
            },
            Ctx::RtcpPacket(data) => {
                // An event which fires for every received rtcp packet,
                // containing the call statistics and reporting information.
                println!("RTCP packet received: {:?}", data.packet);
            },
            Ctx::ClientDisconnect(ClientDisconnect { user_id, .. }) => {
                // You can implement your own logic here to handle a user who has left the
                // voice channel e.g., finalise processing of statistics etc.
                // You will typically need to map the User ID to their SSRC; observed when
                // first speaking.

                println!("Client disconnected: user {:?}", user_id);
            },
            
            _ => {
                // We won't be registering this struct for any more event classes.
                unimplemented!()
            },
        }

        None
    }
}


fn bytes_to_binary_string(bytes: &[u8]) -> String {
    bytes.iter()
         .map(|byte| format!("{:08b} ", byte))
         .collect::<Vec<String>>()
         .concat()
}

fn bytes_to_hexadecimal_string(bytes: &[u8]) -> String {
    bytes.iter()
         .map(|byte| format!("{} ", hex::encode(&[*byte])))
         .collect::<Vec<String>>()
         .concat()
}