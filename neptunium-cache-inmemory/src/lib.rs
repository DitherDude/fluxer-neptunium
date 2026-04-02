use std::{collections::HashSet, hash::Hash, num::NonZeroUsize};

use bon::Builder;
use lru::LruCache;
use neptunium_model::{
    channel::message::Message,
    guild::Guild,
    id::{
        Id,
        marker::{GuildMarker, MessageMarker, UserMarker},
    },
    user::PartialUser,
};

pub struct InMemoryCache {
    pub users: Option<LruCache<Id<UserMarker>, PartialUser>>,
    pub guild_member_ids: Option<LruCache<Id<GuildMarker>, HashSet<Id<UserMarker>>>>,
    pub guilds: Option<LruCache<Id<GuildMarker>, Guild>>,
    pub messages: Option<LruCache<Id<MessageMarker>, Message>>,
}

/// Configuration for a specific item type in the cache.
#[derive(Copy, Clone, Debug)]
pub enum CachedItemConfig {
    /// The cache for this type of item is disabled.
    Disabled,
    /// The cache for this type of item has a specified limited number of elements.
    /// If the limit is reached and a new item is added to the cache, the least recently used
    /// item will be evicted.
    WithCapacity(NonZeroUsize),
    /// The cache for this type of item has an unlimited size.
    Unbounded,
}

#[derive(Builder, Copy, Clone, Debug)]
#[builder(const)]
pub struct InMemoryCacheConfig {
    pub users: CachedItemConfig,
    pub guild_member_ids: CachedItemConfig,
    pub guilds: CachedItemConfig,
    pub messages: CachedItemConfig,
}

impl InMemoryCache {
    #[must_use]
    pub fn new(config: InMemoryCacheConfig) -> Self {
        fn create_lru_cache_from_config<K: Hash + Eq, V>(
            config: CachedItemConfig,
        ) -> Option<LruCache<K, V>> {
            match config {
                CachedItemConfig::Disabled => None,
                CachedItemConfig::WithCapacity(cap) => Some(LruCache::new(cap)),
                CachedItemConfig::Unbounded => Some(LruCache::unbounded()),
            }
        }
        Self {
            users: create_lru_cache_from_config(config.users),
            guild_member_ids: create_lru_cache_from_config(config.guild_member_ids),
            guilds: create_lru_cache_from_config(config.guilds),
            messages: create_lru_cache_from_config(config.messages),
        }
    }
}

impl Default for InMemoryCacheConfig {
    fn default() -> Self {
        Self {
            users: CachedItemConfig::Unbounded,
            guild_member_ids: CachedItemConfig::Unbounded,
            guilds: CachedItemConfig::Unbounded,
            messages: CachedItemConfig::WithCapacity(NonZeroUsize::new(4096).unwrap()),
        }
    }
}
