mod commands;
mod handler;

use handler::Handler;
use serenity::Client;
use serenity::all::GatewayIntents;
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env");
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN is not set");

    let intents = GatewayIntents::GUILDS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(error) = client.start().await {
        eprintln!("Client error: {error}");
    }
}
