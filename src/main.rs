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

#[group]
#[commands(ping)]
struct General;

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

                let cs: Vec<_> = g.channels(&context_clone).await.unwrap().into_iter().filter(|el| el.0.to_string() == "232047335335526401").collect();

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
                                    // let s = songbird::ffmpeg("path.mp3").await.expect("fail to open");
                                    // results in
                                    // ffmpeg -i path.mp3 -f s16le -ac 2 -ar 48000 -acodec pcm_f32le -
                                    // we dont need ignore thread to hijack stdin anymore.
                                    self.tx.lock().expect("fail to acquire lock").send(true).expect("fail to send");
                                    handler.lock().await.play_only_source(x.into_input());
                                    // handler.lock().await.play_only_source(s);
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



    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

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

    // client.co

    // let x = client.data.read().await.get::<>.

    // x.lock().await.?
    // let manager = songbird::get(x).await.unwrap();
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

fn notmain() {

    let (mut producer, mut consumer) = HeapRb::<u8>::new(1024).split();
    // let rb = HeapRb::<u8>::new(1024);
    // let stdin = io::stdin();


    // producer.


    let h = thread::spawn(move || {
        let mut buf: [u8; 10] = [0; 10];
        loop {
            while let Ok(v) = io::stdin().read(&mut buf) {
                println!("[PRODUCER] Read {} bytes", v);
                producer.push_slice(&buf);  
            }
        }
        // match io::stdin().read(&mut buf) {
        //     Ok(v) => {
        //         println!("[PRODUCER] Read {} bytes", v);
        //         producer.push_slice(&buf);
        //     }
        //     Err(e) => {
        //         println!("cant read coz {}", e);
        //     }
        // }
    });
    
    // h.join().unwrap();

    let mut buf2: [u8; 10] = [0; 10];
    
    loop {
        while let Ok(v) = consumer.read(&mut buf2) {
            println!("[CONSUMER] Read {} bytes", v);
        }
    }
    //     match 
    //     Ok(v) => {
    //         // println!("Buffer is {:?}", buf2);
    //     }
    //     Err(e) => {
    //         println!("cant read coz {}", e);
    //     }
    // }



    // println!("Buf is {:?}", buf);
    // buf[0] = 4;
    // println!("Buf is {:?}", buf);

}


struct NullSource;

impl NullSource {
    pub fn into_input(self) -> Input {

        Input::new(
            true,
            Reader::Extension(Box::new(self)),
            Codec::FloatPcm,
            Container::Raw,
            None,
        )
    }
}

impl Read for NullSource {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        std::io::stdin().read(buf)
        // let safe_length = buf.len();
        // println!("Reading {} bytes", safe_length);
        // buf.fill(0x00);
        // Ok(safe_length)
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