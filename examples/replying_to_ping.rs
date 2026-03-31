use std::{env, sync::Arc};

use fluxer_neptunium::{model::gateway::payload::incoming::MessageCreate, prelude::*};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn on_message_create(
        &self,
        ctx: Context,
        message: Arc<MessageCreate>,
    ) -> Result<(), EventError> {
        if !message.author.bot && message.content == "n?ping" {
            message.reply(&ctx, "Pong!").await?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("FLUXER_TOKEN").unwrap();
    let mut client = Client::new(token);

    client.register_event_handler(Handler);

    client.start().await.unwrap();
}
