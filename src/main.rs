use std::thread;

use log::{error, debug};
use songbird::SerenityInit;
use clap::Parser;

use serenity::prelude::*;
use serde::{Serialize, Deserialize};
use std::sync::mpsc;

mod stdin;
mod discord;


#[derive(Default, Serialize, Deserialize)]
#[serde(default)] // we need this if we add new fields, otherwise confy will throw error on load (https://github.com/rust-cli/confy/issues/34)
struct StdinmanConfig {
    bot_token: String,
    voice_channel_id: String,
    bot_activity: String
}

#[derive(Parser)]
struct StdinmanArgs {
    #[arg(long)]
    bot_token: Option<String>,
    #[arg(long)]
    voice_channel_id: Option<String>,
    #[arg(long)]
    bot_activity: Option<String>,
}

#[tokio::main]
async fn main() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

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

    let bot_activity = match args.bot_activity {
        Some(activity) => activity,
        None => {
            if cfg.bot_activity == "" {
                String::from("stdinman")
            } else {
                cfg.bot_activity
            }
        }
    };

    let (tx, rx) = mpsc::channel::<bool>();

    // As soon as stdinman is started, data will start to get piped to it (e.g. from ffmpeg)
    // But it'll take some time to actually connect to discord and the voice channel 
    // (3-5s in practice). To avoid these 3-5s of audio getting buffered and then causing some weird
    // behavior, we create a "dummy" stdin consumer, which reads from stdin and just discards the data
    debug!("starting early-stdin consumer thread");
    thread::spawn(|| stdin::early_stdin_consumer(rx));

    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(bot_token, intents)
        .register_songbird()
        .event_handler(discord::Handler{ 
            voice_channel_id: voice_channel_id,
            bot_activity: bot_activity,
            tx: tx.into(),
        })
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
