use std::sync::Mutex;

use log::{error, debug, info};
use songbird::input::{Input, Reader};

use serenity::{async_trait, model::prelude::Activity};
use serenity::prelude::*;
use std::sync::mpsc;

use super::stdin;

pub struct Handler {
    pub voice_channel_id: String,
    pub tx: Mutex<mpsc::Sender<bool>>
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, _ready: serenity::model::gateway::Ready) {
        info!("discord bot is ready");
        context.online().await;
        context.set_activity(Activity::playing("THIS IS A TEST")).await;

        let cache_clone = context.cache.clone();
        let context_clone = context.clone();
        let guilds = context.cache.guilds().clone();

        for guild in guilds {
            // Don't filter by guild_id, just check all channels in the guild 
            // Ideally stdinman is intended to be used by a solo bot in just few servers,
            // but only one VC at a time anyway.
            let channels: Vec<_> = guild.channels(&context_clone).await.unwrap().into_iter().filter(|channel| channel.0.to_string() == self.voice_channel_id).collect();

            if channels.len() != 1 {
                continue;
            }

            let channel = channels[0].1.to_owned();
            info!("found the channel (in {}), attempting to connect...", guild.name(&cache_clone).unwrap_or("(failed to get server name)".to_string()));

            let manager = songbird::get(&context_clone).await.expect("failed to get songbird manager");
            let (handler, result) = manager.join(channel.guild_id, channel.id).await;

            if let Err(e) = result {
                error!("Failed to join channel: {}", e);
                return;
            }

            info!("joined channel sucessfully!");
            debug!("Telling early-stdin consumer to stop...");
            self.tx.lock().expect("fail to acquire lock on early-stdin consumer").send(true).expect("fail to signal early-stdin consumer");

            // Now we have access to stdin
            info!("going to pipe stdin to the bot");
            let stdin_reader = stdin::StdinReader;
            handler.lock().await.play_only_source(Input::float_pcm(true, Reader::Extension(Box::new(stdin_reader))));
            info!("piping successfully...");
        }
    }
}
