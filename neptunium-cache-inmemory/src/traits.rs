use std::sync::Arc;

use crate::{Cache, Cached, CachedChannel, CachedMessage};
use async_trait::async_trait;
use neptunium_http::{
    client::HttpClient,
    endpoints::{Endpoint, ExecuteEndpointRequestError},
};
use neptunium_model::{
    gateway::payload::incoming::UserPrivateResponse,
    guild::{Guild, permissions::GuildRole},
    invites::{GroupDmInvite, GuildInvite, InviteWithMetadata, PackInvite},
    user::PartialUser,
};

pub mod cachable_endpoints;

pub(crate) trait CacheValue: Sized {
    fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self>;
}

#[async_trait]
pub trait CachableEndpoint: Endpoint {
    type Response;
    /// Either get the result from the cache or execute the request.
    /// # Errors
    /// Returns an error if the HTTP request fails or parsing the response fails.
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as CachableEndpoint>::Response, Box<ExecuteEndpointRequestError>>;
}

impl CacheValue for CachedChannel {
    fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let channel_id = self.id;
        if let Some(existing_channel) = cache.channels.get(&channel_id) {
            return existing_channel.store_and_return(self);
        }
        let value = Cached::new(self);
        cache.channels.insert(channel_id, value.clone());
        value
    }
}

impl CacheValue for CachedMessage {
    fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let message_id = self.id;
        if let Some(existing_message) = cache.messages.get(&message_id) {
            return existing_message.store_and_return(self);
        }
        let value = Cached::new(self);
        cache.messages.insert(message_id, value.clone());
        value
    }
}

impl CacheValue for UserPrivateResponse {
    fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        if let Some(existing_user) = cache.current_user.get() {
            existing_user.store_and_return(self)
        } else {
            cache.current_user.get_or_init(|| Cached::new(self)).clone()
        }
    }
}

impl CacheValue for InviteWithMetadata {
    fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let code = match &self {
            Self::EmojiPack(PackInvite { code, .. }, _)
            | Self::GroupDm(GroupDmInvite { code, .. }, _)
            | Self::Guild(GuildInvite { code, .. }, _)
            | Self::StickerPack(PackInvite { code, .. }, _) => code.clone(),
        };
        if let Some(cached_invite) = cache.invites.get(&code) {
            return cached_invite.store_and_return(self);
        }
        let cached = Cached::new(self);
        cache.invites.insert(code, cached.clone());
        cached
    }
}

impl CacheValue for PartialUser {
    fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let user_id = self.id;
        if let Some(existing_user) = cache.users.get(&user_id) {
            return existing_user.store_and_return(self);
        }
        let cached = Cached::new(self);
        cache.users.insert(user_id, cached.clone());
        cached
    }
}

impl CacheValue for neptunium_model::user::settings::UserSettings {
    fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        if let Some(existing_settings) = cache.current_user_settings.get() {
            existing_settings.store_and_return(self)
        } else {
            cache
                .current_user_settings
                .get_or_init(|| Cached::new(self))
                .clone()
        }
    }
}

impl CacheValue for Guild {
    fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let guild_id = self.id;
        if let Some(existing_guild) = cache.guilds.get(&guild_id) {
            return existing_guild.store_and_return(self);
        }
        let cached = Cached::new(self);
        cache.guilds.insert(guild_id, cached.clone());
        cached
    }
}

impl CacheValue for GuildRole {
    fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let role_id = self.id;
        if let Some(existing_role) = cache.roles.get(&role_id) {
            return existing_role.store_and_return(self);
        }
        let cached = Cached::new(self);
        cache.roles.insert(role_id, cached.clone());
        cached
    }
}
