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
mod discord;


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
    thread::spawn(|| stdin::early_stdin_consumer(rx));

    let framework = StandardFramework::new();
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(bot_token, intents)
        .register_songbird()
        .event_handler(discord::Handler{ 
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
