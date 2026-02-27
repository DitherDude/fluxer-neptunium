//! Types only used in the gateway

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::model::snowflake::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomStatusResponse {
    text: String,
    emoji_id: Option<Snowflake>,
    emoji_name: Option<String>,
    #[serde(with = "time::serde::iso8601::option")]
    expires_at: Option<OffsetDateTime>,
}
