use serenity::all::{Context, EventHandler, GuildId, Interaction, Ready};
use serenity::async_trait;
use std::env;

use crate::commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot connected as {}", ready.user.name);

        let guild_id = env::var("GUILD_ID")
            .expect("GUILD_ID is not set")
            .parse::<u64>()
            .expect("Invalid GUILD_ID");

        commands::register(&ctx, GuildId::new(guild_id)).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            commands::handle(&ctx, &command).await;
        }
    }
}
