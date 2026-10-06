use crate::commands::general;

use serenity::all::{Context, EventHandler, Interaction, Ready};

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot connected as {}", ready.user.name);

        let command = serenity::all::Command::create_global_command(
            &ctx.http,
            serenity::all::CreateCommand::new("ping").description("Respond w Pong!"),
        )
        .await;

        match command {
            Ok(command) => {
                println!("Registered command: /{}", command.name);
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
