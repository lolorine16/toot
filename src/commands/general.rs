use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};

pub async fn ping(ctx: &Context, command: &CommandInteraction) {
    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Pong!"),
    );

    if let Err(error) = command.create_response(&ctx.http, response).await {
        eprintln!("Failed to respond to /ping: {error}");
    }
}
