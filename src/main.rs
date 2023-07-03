use std::io::{Read, Seek};
use std::thread;
use std::sync::Mutex;

use songbird::{SerenityInit, input::{Input, Reader, Codec, reader::MediaSource, Container}};

use std::env;

use serenity::{async_trait, model::prelude::Activity};
use serenity::prelude::*;
use serenity::framework::standard::{StandardFramework};
use serde::{Serialize, Deserialize};
use std::sync::mpsc;

struct Handler {
    voice_channel_id: String,
    tx: Mutex<mpsc::Sender<bool>>
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: serenity::model::gateway::Ready) {
        println!("READY");
        context.online().await;
        context.set_activity(Activity::playing("THIS IS A TEST")).await;

        let cache_clone = context.cache.clone();
        let context_clone = context.clone();
        let gs = context.cache.guilds().clone();

        for g in gs {
            // hardcoded XD 
            // TODO: FIXME
            if g.to_string() == "232047335335526400"{
                println!("Found server {:?}", g.name(&cache_clone));

                let cs: Vec<_> = g.channels(&context_clone).await.unwrap().into_iter().filter(|el| el.0.to_string() == "232165171760594946").collect();

                if cs.len() == 1 {
                    let c = cs[0].1.to_owned();
                    println!("Were in bois");
                    match songbird::get(&context_clone).await {
                        Some(manager) => {
                            let (handler, result) = manager.join(c.guild_id, c.id).await;

                            match result {
                                Ok(_) => {
                                    let x = NullSource;
                                    println!("Now we're really in");

                                    // Tell the thread that is "wasting" stdin to stop
                                    self.tx.lock().expect("fail to acquire lock").send(true).expect("fail to send");

                                    // Play stdin via bot
                                    handler.lock().await.play_only_source(Input::float_pcm(true, Reader::Extension(Box::new(x))));
                                },
                                Err(e) => {
                                    println!("Failed to join: {}", e);
                                }
                            }
                        },
                        None => {
                            println!("Failed to join :((");
                        }
                    }
                }
            }
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
struct StdinmanConfig {
    bot_token: String,
    voice_channel_id: String,
}

#[tokio::main]
async fn main() {
    let cfg: StdinmanConfig = match confy::load("stdinman", "stdinman"){
        Ok(c) => c,
        Err(e) => {
            println!("fail to load config: {}", e);
            panic!("fail to load config: {}", e);
        }
    };

    // TODO: Config can be overridden via CLI params (i.e. value = CLI Param || Config)
    if cfg.bot_token == "" || cfg.voice_channel_id == "" {
        let cfg_path = confy::get_configuration_file_path("stdinman", "stdinman").expect("Fail to get config file path");
        println!("Missing bot_token / voice_channel_id in config! Please add it in {}", cfg_path.to_string_lossy());
        std::process::exit(-1);
    }

    let (tx, rx) = mpsc::channel::<bool>();

    let mut ignore_thread = thread::spawn(move || {
        let mut buf = vec![0; 1024];
        loop {
            // check for stop message
            if let Ok(_) = rx.try_recv() {
                println!("Got stop message, will stop.");
                break;
            }
            
            // read and discard all input
            if let Ok(n) = std::io::stdin().read(&mut buf) {
                if n == 0 {
                    // end of input
                    println!("End of input.");
                    break;
                }
            }
        }
    });

    let framework = StandardFramework::new();
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(cfg.bot_token, intents)
        .register_songbird()
        .event_handler(Handler{ 
            voice_channel_id: cfg.voice_channel_id,
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

struct NullSource;

impl Read for NullSource {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        std::io::stdin().read(buf)
    }
}

impl Seek for NullSource {
    fn seek(&mut self, _: std::io::SeekFrom) -> std::io::Result<u64> {
        unreachable!()
    }
}

impl MediaSource for NullSource {
    fn byte_len(&self) -> Option<u64> {
        None
    }

    fn is_seekable(&self) -> bool {
        false
    }
}