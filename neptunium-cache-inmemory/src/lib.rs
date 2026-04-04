use std::{fmt::Debug, sync::Arc};

use bon::Builder;
use mini_moka::sync::Cache as MokaCache;
use neptunium_http::endpoints::users::UserProfileFullResponse;
use neptunium_model::{
    channel::{Channel, message::Message}, id::{Id, marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker}}, user::PartialUser
};

mod traits;
pub use traits::*;

pub type Cached<T> = Arc<tokio::sync::RwLock<T>>;

#[expect(clippy::type_complexity)]
pub struct Cache {
    pub users: MokaCache<Id<UserMarker>, Cached<PartialUser>>,
    pub user_profiles: MokaCache<(Id<UserMarker>, Option<Id<GuildMarker>>), Cached<UserProfileFullResponse>>,
    pub channels: MokaCache<Id<ChannelMarker>, Cached<Channel>>,
    pub messages: MokaCache<Id<MessageMarker>, Cached<Message>>,
}

#[derive(Builder, Copy, Clone, Debug)]
pub struct CacheConfig {
    #[builder(default = 1024)]
    pub users: u64,
    #[builder(default = 256)]
    pub user_profiles: u64,
    #[builder(default = 1024)]
    pub channels: u64,
    #[builder(default = 1024)]
    pub messages: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Cache {
    #[must_use]
    pub fn new(config: CacheConfig) -> Self {
        Self {
            users: MokaCache::new(config.users),
            user_profiles: MokaCache::new(config.user_profiles),
            channels: MokaCache::new(config.channels),
            messages: MokaCache::new(config.messages),
        }
    }
}

impl Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Cache { ... }")
    }
}
