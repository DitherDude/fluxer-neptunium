use std::ops::{Deref, DerefMut};

use neptunium_model::{
    channel::ChannelType,
    gateway::payload::incoming::{MessageCreate, MessageDelete},
};

use crate::{CacheValue, Cached, CachedMessage, gateway::cached_payload::CachedPayload};

pub struct CachedMessageCreate {
    pub message: Cached<CachedMessage>,
    pub channel_type: ChannelType,
}

impl CachedPayload for CachedMessageCreate {
    type NonCached = MessageCreate;
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        if let Some(channel) = cache.channels.get(&non_cached.channel_id) {
            let channel = channel.load();
            if let Some(last_message_id) = channel.last_message_id.get() {
                last_message_id.store(non_cached.id);
            } else {
                let _ = channel.last_message_id.set(non_cached.id.into());
            }
        }
        let message =
            CachedMessage::from_message(non_cached.message, cache).insert_and_return(cache);
        Self {
            message,
            channel_type: non_cached.channel_type,
        }
    }
}

impl CachedPayload for MessageDelete {
    type NonCached = Self;
    fn cache_payload(non_cached: Self::NonCached, cache: &std::sync::Arc<crate::Cache>) -> Self {
        cache.messages.invalidate(&non_cached.id);
        non_cached
    }
}

impl Deref for CachedMessageCreate {
    type Target = Cached<CachedMessage>;
    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

impl DerefMut for CachedMessageCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.message
    }
}
