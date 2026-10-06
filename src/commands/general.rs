use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

pub fn register_ping() -> CreateCommand {
    CreateCommand::new("ping").description("Respond w Pong!")
}

pub async fn ping(ctx: &Context, command: &CommandInteraction) {
    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Pong!"),
    );

    if let Err(error) = command.create_response(&ctx.http, response).await {
        eprintln!("Failed to respond to /ping: {error}");
    }
}
