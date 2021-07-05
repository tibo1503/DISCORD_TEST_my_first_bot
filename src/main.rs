use std::{path::Path, time::Duration, fs};

//Serenity
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    http::AttachmentType,
};

use tokio::time::sleep;

use std::{
    sync::{
        Arc, atomic::{
            AtomicUsize, 
            Ordering
        }
    }, 
//    collections::HashMap
};

//Deserialize
use serde::{Serialize, Deserialize};
use serde_json::{Result, json};


//Global data
struct CounterCommand;

impl TypeMapKey for CounterCommand {
    type Value = Arc<AtomicUsize>;
}




struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase().contains("owo") {
            let count = {
                let data_read = ctx.data.read().await;
                data_read.get::<CounterCommand>().expect("Expected MessageCount in TypeMap.").clone()
            };

            count.fetch_add(1, Ordering::SeqCst);
        }


        let prefix = "r!";
        
        if msg.content == format!("{}hello", prefix) {
            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("Hello, World!");
                m.embed(|e| {
                    e.title("Salutation ");
                    e.description("Ceci est un teste Rust");
                    e.image("attachment://937245.jpg");
                    e.fields(vec![
                        ("This is the first field", "This is a field body", true),
                        ("This is the second field", "Both of these fields are inline", true),
                    ]);
                    e.field("This is the third field", "This is not an inline field", false);
                    e.footer(|f| {
                        f.text("par tibo1503 via un exemple copié à 99.99%");

                        f
                    });

                    e
                });
                m.add_file(AttachmentType::Path(Path::new("./ressources/937245.jpg")));
                m
            }).await;

            if let Err(why) = msg {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == format!("{}doge_value", prefix) {
            // -=Structs block to deserialize DogeCoin API values=-
            #[derive(Deserialize, Debug)]
            struct Base {
                status: String,
                data: Data,
            }

            #[derive(Deserialize, Debug)]
            struct Data {
                network: String,
                prices: Vec<Prices>,
            }

            #[derive(Deserialize, Debug)]
            struct Prices {
                price: String,
                price_base: String,
                exchange: String,
                time: u64
            }

            // -=Request block for any doge API: "https://sochain.com//api/v2/get_price/"=-
            let resp: Base = reqwest::get("https://sochain.com//api/v2/get_price/DOGE")
                .await.unwrap()
                .json().await.unwrap();
            //println!("{:#?}", resp);

            //let deserilised_data = ;

            /*let mut all_estimed_value: Vec<(&str, &str, bool)> = resp.data.prices
                .iter()
                .map(|x| (&x.price,&x.exchange,true))
                .collect();*/

            // -=Vec String to create vec &str=-
            let mut all_estimed_value_ownership: Vec<(String, String)> = vec![];

            for x in resp.data.prices.iter() {
                all_estimed_value_ownership.push((format!("Selon {}:", x.exchange),format!("{} {}", &x.price, x.price_base)));
            }
            
            // -=Vec &str to create fields=-
            let mut all_estimed_value: Vec<(&str, &str, bool)> = vec![];

            for x in all_estimed_value_ownership.iter() {
                all_estimed_value.push((&x.0,&x.1,true));
            }

            // -=Embed block=-
            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("");
                m.embed(|e| {
                    e.color(serenity::utils::Colour::ORANGE);
                    e.title("Valeur STONKS du DogeCoin");
                    e.description(format!("Le cours actuelle du doge selon {} sources", all_estimed_value.len()));
                    e.image("attachment://doge_stonks.jpg");
                    e.fields(all_estimed_value);
                    e.footer(|f| {
                        f.text("Une commande faite pour la secte du doge (et oui, ceci participe au délire ...)");

                        f
                    });

                    e
                });
                m.add_file(AttachmentType::Path(Path::new("./ressources/doge_stonks.jpg")));
                m
            }).await;
            
            // -=Error block=-
            if let Err(why) = msg {
                println!("Error sending message: {:?}", why);
            }
        }

        if (msg.content == format!("{}owo_count", prefix)) {
            let raw_count = {
                let data_read = ctx.data.read().await;
                data_read.get::<CounterCommand>().expect("Expected MessageCount in TypeMap.").clone()
            };
        
            let count = raw_count.load(Ordering::Relaxed);
        
            if count == 1 {
                msg.reply(&ctx, "You are the first one to say owo this session! *because it's on the command name* :P").await;
            } else {
                msg.reply(&ctx, format!("OWO Has been said {} times with it's command ... OWO !!!", count)).await;
            }
            
        }


        if msg.content == "!ping" {
            println!("Shard {}", ctx.shard_id);

            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

}

#[tokio::main]
async fn main() {
    let token = fs::read_to_string("token.txt")
        .expect("Something went wrong reading the file");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

        let manager = client.shard_manager.clone();

        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(30)).await;
    
                let lock = manager.lock().await;
                let shard_runners = lock.runners.lock().await;
    
                for (id, runner) in shard_runners.iter() {
                    match runner.latency {
                        Some(x) => {
                            println!(
                                "Shard ID {} is {} with a latency of {:?}",
                                id,
                                runner.stage,
                                x,
                            );
                        }
                        _ => println!("Latency value not found")
                    }
                }
            }
        });

    {
        // Open the data lock in write mode, so keys can be inserted to it.
        let mut data = client.data.write().await;
    
        // The CommandCounter Value has the following type:
        // Arc<RwLock<HashMap<String, u64>>>
        // So, we have to insert the same type to it.
        data.insert::<CounterCommand>(Arc::new(AtomicUsize::new(0)));
    }

        
    if let Err(why) = client.start_shards(2).await {
        println!("Client error: {:?}", why);
    }
}