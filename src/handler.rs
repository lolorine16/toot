use crate::commands::general;

use serenity::all::{Context, EventHandler, Interaction, Ready};

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot connected as {}", ready.user.name);

        let guild_id = std::env::var("GUILD_ID")
            .expect("GUILD_ID is not set")
            .parse::<u64>()
            .expect("Invalid GUILD_ID");

        let guild_id = serenity::all::GuildId::new(guild_id);

        let command = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    serenity::all::CreateCommand::new("ping").description("Respond w Pong!"),
                ],
            )
            .await;

        match command {
            Ok(commands) => {
                println!("Registered {} guild command(s)", commands.len());
            }
            Err(error) => {
                eprintln!("Failed to register command: {error}");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "ping" => {
                    general::ping(&ctx, &command).await;
                }

                _ => {}
            }
        }
    }
}
