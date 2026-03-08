use serde::{Deserialize, Serialize};

use crate::gateway::payload::incoming::{
    guild_create::GuildCreate, guild_delete::GuildDelete, message_create::MessageCreate,
    ready::Ready, typing_start::TypingStart,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "t", content = "d", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DispatchEvent {
    Ready(Ready),
    GuildDelete(GuildDelete),
    GuildCreate(GuildCreate),
    TypingStart(TypingStart),
    MessageCreate(MessageCreate),
    // TODO: Other variants
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DispatchEventPayload {
    #[serde(flatten)]
    pub event: DispatchEvent,
    #[serde(rename = "s")]
    pub sequence_number: u64,
}
