use serenity::{
    Client, 
    all::{
        Context, EventHandler, GatewayIntents, Interaction, Ready
    },
};
use std::env;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot connected as {}", ready.user.name);

        // register /ping 
        let command = serenity::all::Command::create_global_command(
            &ctx.http,
            serenity::all::CreateCommand::new("ping")
                .description("pong!"),
        )
        .await;

        match command {
            Ok(command) => println!("Registered command: /{}", command.name),
            Err(error) => eprintln!("Failed to register command: {error}"),
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            if command.data.name == "ping" {
                if let Err(error) = command
                    .create_response(
                        &ctx.http,
                        serenity::all::CreateInteractionResponse::Message(
                            serenity::all::CreateInteractionResponseMessage::new()
                            .content("Pong!"),
                        ),
                    )
                    .await
                {
                    eprintln!("Failed to respond to /ping: {error}");
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env");
    let token = env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN is not set");

    let intents = GatewayIntents::GUILDS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    
    if let Err(error) = client.start().await {
        eprintln!("Client error: {error}");
    }
}
