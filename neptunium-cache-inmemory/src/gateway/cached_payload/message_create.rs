use neptunium_model::{channel::ChannelType, gateway::payload::incoming::MessageCreate};

use crate::{CacheValue, Cached, CachedMessage, gateway::cached_payload::FromNonCached};

pub struct CachedMessageCreate {
    pub message: Cached<CachedMessage>,
    pub channel_type: ChannelType,
}

impl FromNonCached for CachedMessageCreate {
    type NonCached = MessageCreate;
    fn from_noncached(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        let message =
            CachedMessage::from_message(non_cached.message, cache).insert_and_return(cache);
        Self {
            message,
            channel_type: non_cached.channel_type,
        }
    }
}
