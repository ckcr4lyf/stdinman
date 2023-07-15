use std::io::{Read, Seek};
use std::thread;
use std::sync::Mutex;

use log::{warn, error, debug, info};
use songbird::{SerenityInit, input::{Input, Reader, Codec, reader::MediaSource, Container}};
use clap::Parser;

use std::env;

use serenity::{async_trait, model::prelude::Activity};
use serenity::prelude::*;
use serenity::framework::standard::{StandardFramework};
use serde::{Serialize, Deserialize};
use std::sync::mpsc;

mod stdin;

struct Handler {
    voice_channel_id: String,
    tx: Mutex<mpsc::Sender<bool>>
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: serenity::model::gateway::Ready) {
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

#[derive(Default, Serialize, Deserialize)]
struct StdinmanConfig {
    bot_token: String,
    voice_channel_id: String,
}

#[derive(Parser)]
struct StdinmanArgs {
    bot_token: Option<String>,
    voice_channel_id: Option<String>
}

#[tokio::main]
async fn main() { 
    env_logger::init();

    let cfg_path = confy::get_configuration_file_path("stdinman", "stdinman").expect("fail to get config file path");
    let cfg: StdinmanConfig = match confy::load("stdinman", "stdinman"){
        Ok(c) => c,
        Err(e) => {
            println!("fail to load config: {}", e);
            panic!("fail to load config: {}", e);
        }
    };

    // First try CLI, then config on disk
    // (to allow overrides on via CLI)
    let args = StdinmanArgs::parse();
    let bot_token = match args.bot_token {
        Some(token) => token,
        None => {
            if cfg.bot_token == "" {
                error!("missing bot_token in config! Please provide it via CLI arg or add it in {}", cfg_path.to_string_lossy());
                std::process::exit(-1);
            }

            cfg.bot_token
        }
    };

    let voice_channel_id = match args.voice_channel_id {
        Some(token) => token,
        None => {
            if cfg.voice_channel_id == "" {
                error!("missing voice_channel_id in config! Please provide it via CLI arg or add it in {}", cfg_path.to_string_lossy());
                std::process::exit(-1);
            }

            cfg.voice_channel_id
        }
    };

    let (tx, rx) = mpsc::channel::<bool>();

    debug!("starting early-stdin consumer thread");
    let mut ignore_thread = thread::spawn(move || {
        let mut buf = vec![0; 1024];
        loop {
            // check for stop message
            if let Ok(_) = rx.try_recv() {
                debug!("recevied stop instruction, will stop consuming stdin");
                break;
            }
            
            // read and discard all input
            if let Ok(n) = std::io::stdin().read(&mut buf) {
                if n == 0 {
                    break;
                }
            }
        }
    });

    let framework = StandardFramework::new();
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(bot_token, intents)
        .register_songbird()
        .event_handler(Handler{ 
            voice_channel_id: voice_channel_id,
            tx: tx.into(),
        })
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
