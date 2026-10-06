pub mod general;

use serenity::all::{CommandInteraction, Context, CreateCommand, GuildId};


//list for all commands
fn all() -> Vec<CreateCommand> {
    vec![general::register_ping()]
}

//save commands on a guild
pub async fn register(ctx: &Context, guild_id: GuildId) {
    match guild_id.set_commands(&ctx.http, all()).await {
        Ok(commands) => println!("Registered {} guild command(s)", commands.len()),
        Err(error) => eprintln!("Failed to register commands: {error}"),
    }
}

//redirection -> good commands
pub async fn handle(ctx: &Context, command: &CommandInteraction) {
    match command.data.name.as_str() {
        "ping" => general::ping(ctx, command).await,
        other => eprintln!("Unknown command: {other}"),
    }
}
