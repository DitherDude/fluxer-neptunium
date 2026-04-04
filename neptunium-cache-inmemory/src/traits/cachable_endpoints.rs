use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use neptunium_http::{
    client::HttpClient,
    endpoints::{
        ExecuteEndpointRequestError,
        channel::{
            BulkDeleteMessages, CreatePrivateChannel, DeleteChannel, FetchChannel,
            ListChannelMessages, ListPrivateChannels, PreloadMessagesForChannels, UpdateCallRegion,
            UpdateChannelSettings,
        },
        users::{
            GetCurrentUserProfile, GetUserById, GetUserProfile, ListCurrentUserMentions,
            UpdateCurrentUserProfile,
        },
    },
};
use neptunium_model::{
    channel::{Channel, message::Message},
    id::{Id, marker::ChannelMarker},
};
use tokio::sync::RwLock;

use crate::{
    BatchCachableEndpoint, CachableEndpoint, Cache, Cached, NoReturnCachableEndpoint,
    traits::CacheValue,
};

#[async_trait]
impl CachableEndpoint for GetUserById {
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<Cached<Self::Response>, Box<ExecuteEndpointRequestError>> {
        if let Some(cached_user) = cache.users.get(&self.user_id) {
            return Ok(cached_user);
        }
        let res = client.execute(self).await?;
        let user_id = res.id;
        let user = Arc::new(RwLock::new(res));
        cache.users.insert(user_id, Arc::clone(&user));
        Ok(user)
    }
}

#[async_trait]
impl CachableEndpoint for GetUserProfile {
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<Cached<Self::Response>, Box<ExecuteEndpointRequestError>> {
        let cache_key = (self.user_id, self.params.guild_id);
        let cached_profile = cache.user_profiles.get(&cache_key);
        let return_cached_profile = 'blk: {
            let Some(cached_profile) = &cached_profile else {
                break 'blk false;
            };
            let guard = cached_profile.read().await;
            if self.params.with_mutual_friends && guard.mutual_friends.is_none() {
                false
            } else {
                !(self.params.with_mutual_guilds && guard.mutual_guilds.is_none())
            }
        };
        if return_cached_profile {
            // Will never panic because the code that determines whether to return the cached profile already checks for Some(...)
            return Ok(cached_profile.unwrap());
        }

        let mut res = client.execute(self).await?;
        if let Some(cached_profile) = cached_profile {
            {
                let guard = cached_profile.read().await;
                if res.mutual_friends.is_none()
                    && let Some(mutual_friends) = &guard.mutual_friends
                {
                    let mutual_friends = mutual_friends.clone();
                    res.mutual_friends = Some(mutual_friends);
                }
                if res.mutual_guilds.is_none()
                    && let Some(mutual_guilds) = &guard.mutual_guilds
                {
                    let mutual_guilds = mutual_guilds.clone();
                    res.mutual_guilds = Some(mutual_guilds);
                }
            }
            {
                let mut guard = cached_profile.write().await;
                *guard = res;
            }
            Ok(cached_profile)
        } else {
            let id = res.user.id;
            let guild_id = self.params.guild_id;
            let arc = Arc::new(RwLock::new(res));
            cache.user_profiles.insert((id, guild_id), Arc::clone(&arc));
            Ok(arc)
        }
    }
}

#[async_trait]
impl NoReturnCachableEndpoint for DeleteChannel {
    async fn noreturn_execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<Self::Response, Box<ExecuteEndpointRequestError>> {
        let channel_id = self.channel_id;
        client.execute(self).await?;
        cache.channels.invalidate(&channel_id);
        Ok(())
    }
}

#[async_trait]
impl CachableEndpoint for FetchChannel {
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<Cached<Self::Response>, Box<ExecuteEndpointRequestError>> {
        if let Some(cached_channel) = cache.channels.get(&self.channel_id) {
            return Ok(cached_channel);
        }
        let res = client.execute(self).await?;
        Ok(res.insert_and_return(cache).await)
    }
}

#[async_trait]
impl CachableEndpoint for UpdateChannelSettings {
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<Cached<Self::Response>, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        Ok(res.insert_and_return(cache).await)
    }
}

#[async_trait]
impl NoReturnCachableEndpoint for UpdateCallRegion {
    async fn noreturn_execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<Self::Response, Box<ExecuteEndpointRequestError>> {
        let channel_id = self.channel_id;
        let region_clone = self.region.clone();
        client.execute(self).await?;
        if let Some(cached_channel) = cache.channels.get(&channel_id) {
            let mut guard = cached_channel.write().await;
            guard.rtc_region = Some(region_clone);
        }
        Ok(())
    }
}

#[async_trait]
impl NoReturnCachableEndpoint for BulkDeleteMessages {
    async fn noreturn_execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<Self::Response, Box<ExecuteEndpointRequestError>> {
        let messages = self.messages.clone();
        client.execute(self).await?;
        for message in messages {
            cache.messages.invalidate(&message);
        }
        Ok(())
    }
}

#[async_trait]
impl BatchCachableEndpoint for ListChannelMessages {
    type Response = Vec<Cached<Message>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as BatchCachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        let mut cached_messages = Vec::with_capacity(res.len());
        for message in res {
            cached_messages.push(message.insert_and_return(cache).await);
        }
        Ok(cached_messages)
    }
}

#[async_trait]
impl CachableEndpoint for GetCurrentUserProfile {
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<Cached<Self::Response>, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        Ok(res.insert_and_return(cache).await)
    }
}

#[async_trait]
impl CachableEndpoint for UpdateCurrentUserProfile {
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<Cached<Self::Response>, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        Ok(res.insert_and_return(cache).await)
    }
}

#[async_trait]
impl BatchCachableEndpoint for ListPrivateChannels {
    type Response = Vec<Cached<Channel>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as BatchCachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        let mut cached_channels = Vec::with_capacity(res.len());
        for channel in res {
            cached_channels.push(channel.insert_and_return(cache).await);
        }
        Ok(cached_channels)
    }
}

#[async_trait]
impl CachableEndpoint for CreatePrivateChannel {
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<Cached<Self::Response>, Box<ExecuteEndpointRequestError>> {
        Ok(client.execute(self).await?.insert_and_return(cache).await)
    }
}

#[async_trait]
impl BatchCachableEndpoint for ListCurrentUserMentions {
    type Response = Vec<Cached<Message>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as BatchCachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        let mut cached_messages = Vec::with_capacity(res.len());
        for message in res {
            cached_messages.push(message.insert_and_return(cache).await);
        }
        Ok(cached_messages)
    }
}

#[async_trait]
impl BatchCachableEndpoint for PreloadMessagesForChannels {
    type Response = HashMap<Id<ChannelMarker>, Cached<Message>>;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as BatchCachableEndpoint>::Response, Box<ExecuteEndpointRequestError>> {
        let res = client.execute(self).await?;
        let mut cached_messages = HashMap::with_capacity(res.len());
        for (id, message) in res {
            cached_messages.insert(id, message.insert_and_return(cache).await);
        }
        Ok(cached_messages)
    }
}
