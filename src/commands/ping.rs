use std::time::Instant;

use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed, Timestamp};

use crate::{CmdContext, Error};


fn build_description(ping: &u128) -> String {
    format!("Latency: `{}ms`", ping)
}

// Shows ping of the bot!
#[poise::command(
    slash_command,
    rename = "ping"
)]
pub async fn execute(ctx: CmdContext<'_>) -> Result<(), Error> {
        let embed = CreateEmbed::new()
            .title("🏓 Pong!")
            .color(Colour::GOLD)
            .timestamp(Timestamp::now());


        let cached_ping: Option<u128> = if let Some((last_call, ping)) = *ctx.data().last_ping.read().unwrap() {
            if last_call.elapsed().as_secs() < 10 {
                Some(ping)
            } else { None }
        } else { None };

        if let Some(ping) = &cached_ping {
            let embed = embed.description(build_description(ping));
            
            ctx.send(CreateReply {
                embeds: vec![embed],
                ephemeral: Some(true),
                ..Default::default()
            }).await?;
            return Ok(())
        }

        let before = Instant::now();
        let msg = ctx.send(CreateReply {
            content: Some(String::from("Wait a bit...")),
            ephemeral: Some(true),
            ..Default::default()
        }).await?;
        let ping = before.elapsed().as_millis();

        let embed = embed.description(build_description(&ping));

        msg.edit(ctx, CreateReply {
            content: Some(String::new()),
            embeds: vec![embed],
            ..Default::default()
        }).await?;

        *ctx.data().last_ping.write().unwrap() = Some((Instant::now(), ping));

    Ok(())
}