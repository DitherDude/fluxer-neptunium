use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::model::{object::user::UserPartialResponse, snowflake::Snowflake};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelResponse {
    /// The bitrate of the voice channel in bits per second
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guild_id: Option<Snowflake>,
    /// The icon hash of the channel (for group DMs)
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_message_id: Option<Snowflake>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "time::serde::iso8601::option"
    )]
    last_pin_timestamp: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// Custom nicknames for users in this channel (for group DMs)
    #[serde(skip_serializing_if = "Option::is_none")]
    nicks: Option<HashMap<Snowflake, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_overwrites: Option<Vec<ChannelOverwriteResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_per_user: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipients: Option<Vec<UserPartialResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rtc_region: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<String>,
    // TODO: figure out what this number means (its the type of the channel)
    #[serde(rename = "type")]
    r#type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelOverwriteResponse {
    allow: String,
    deny: String,
    id: Snowflake,
    // TODO: figure out what this number means (the type of entity the overwrite applies to)
    /// The type of entity the overwrite applies to (0 or 1)
    #[serde(rename = "type")]
    r#type: u8,
}
