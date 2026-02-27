use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::model::{object::user::UserPartialResponse, snowflake::Snowflake};

// Not officially documented?
// https://docs.fluxer.app/api-reference/channels/send-a-message ?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub author: UserPartialResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<Snowflake>,
    #[serde(rename = "type")]
    pub r#type: i32,
    pub flags: i32,
    pub content: String,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
    pub pinned: bool,
    pub mention_everyone: bool,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "time::serde::iso8601::option"
    )]
    pub edited_timestamp: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<UserPartialResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_roles: Option<Vec<Snowflake>>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<serde_json::Value>>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<serde_json::Value>>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<serde_json::Value>>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<serde_json::Value>>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<serde_json::Value>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_snapshots: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call: Option<serde_json::Value>,
    // TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_message: Option<serde_json::Value>,
}
