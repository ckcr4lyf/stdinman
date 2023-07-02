use std::{io::{self, Read}, thread, sync::{Arc, Mutex}};

use ringbuf::{HeapRb, HeapProducer, HeapConsumer};
use songbird::SerenityInit;

use std::env;

use serenity::{async_trait, model::prelude::Activity};
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(ping)]
struct General;

struct Handler;

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
            if g.to_string() == "232047335335526400"{
                println!("Found server {:?}", g.name(&cache_clone));

                let cs: Vec<_> = g.channels(&context_clone).await.unwrap().into_iter().filter(|el| el.0.to_string() == "232047335335526401").collect();

                if cs.len() == 1 {
                    let c = cs[0].1.to_owned();
                    println!("Were in bois");
                    match songbird::get(&context_clone).await {
                        Some(manager) => {
                            let (handler, result) = manager.join(c.guild_id, c.id).await;
                            println!("Now we're really in");
                        },
                        None => {
                            println!("Failed to join :((");
                        }
                    }
                }



            }
        }

        // for guild in ready.guilds {
        //     println!("Found guild {} (Unavailable: {})", guild.id, guild.id.name(c.clone()).expect("no name"));
            
        //     match guild.id.channels(context.clone()).await {
        //         Ok(c) => {
        //             println!("found channels");
        //             let i = c.into_iter();
        //             for (gi, gc) in i {
        //                 println!("Name: {} (ID={})", gc.name(), gc.id);
        //                 // https://discord.com/channels/232047335335526400/232047335335526401
        //                 // self.conn
        //                 if gc.id.to_string() == "232047335335526401" {
                            // println!("Were in bois");
                            // match songbird::get(&c2).await {
                            //     Some(manager) => {
                            //         let (handler, result) = manager.join(gc.guild_id, gc.id).await;
                            //         println!("Now we're really in");
                            //     },
                            //     None => {
                            //         println!("Failed to join :((");
                            //     }
                            // }
                            
        //                 }
        //             }
        //         },
        //         Err(e)=> {
        //             println!("Fail to get channels: {}", e);
        //         }
        //     }
        // }
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(token, intents)
        .register_songbird()
        .event_handler(Handler)
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
