use serde::{Deserialize, Serialize};

use crate::model::object::message::MessageResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCreateDispatchData {
    #[serde(flatten)]
    pub message_response: MessageResponse,
    // TODO: find out what this means
    pub channel_type: i32,
}
