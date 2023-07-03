use std::{io::{self, Read, Seek}, thread, sync::{Arc, Mutex, mpsc::Sender}};

use ringbuf::{HeapRb, HeapProducer, HeapConsumer};
use songbird::{SerenityInit, input::{Input, Reader, Codec, reader::MediaSource, Container}};

use std::env;

use serenity::{async_trait, model::prelude::Activity};
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

use std::sync::mpsc;

struct Handler {
    tx: Mutex<Sender<bool>>
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
                                    handler.lock().await.play_only_source(x.into_input());
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

#[tokio::main]
async fn main() {

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

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(token, intents)
        .register_songbird()
        .event_handler(Handler{ 
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

impl NullSource {
    pub fn into_input(self) -> Input {
        Input::new(
            true,
            Reader::Extension(Box::new(self)),
            Codec::FloatPcm, // TODO: Try PCM (s16le)?
            Container::Raw,
            None,
        )
    }
}

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