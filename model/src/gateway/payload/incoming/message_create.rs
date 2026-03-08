use serde::{Deserialize, Serialize};

use crate::channel::{ChannelType, message::Message};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageCreate {
    #[serde(flatten)]
    pub message: Message,
    pub channel_type: ChannelType,
}
