mod commands;

use serenity::all::{Context, EventHandler, GatewayIntents, Ready};
use serenity::{async_trait, Client};
use std::env::var;
use std::sync::{Arc, RwLock};
use std::time::Instant;

pub struct UserData {
    last_ping: Arc<RwLock<Option<(Instant, u128)>>>
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type CmdContext<'a> = poise::Context<'a, UserData, Error>;

// Handler
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        if let Some(shard) = data_about_bot.shard {
            if shard.id.get() == 0 {

                println!(
                    "Hewwo! I'm alive as {}!! :3 ({} shard{})",
                    &data_about_bot.user.name,
                    shard.total,
                    if shard.total > 1 { "s" } else { "" }
                )

            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = var("TOKEN").expect("TOKEN is not set!");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .setup(|_, _, _| {
            Box::pin(async move {
                Ok(UserData {
                    last_ping: Arc::new(RwLock::new(None)),
                })
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::execute()
            ],
            ..Default::default()
        })
        .build();

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Failed to make client qwq");

    if let Err(why) = client.start_autosharded().await {
        println!("NOO qwq the client did oopsie qwq:\n{why:?}");
    }
}
