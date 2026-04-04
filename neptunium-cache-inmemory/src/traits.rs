use std::sync::Arc;

use crate::{Cache, Cached};
use async_trait::async_trait;
use neptunium_http::{
    client::HttpClient,
    endpoints::{Endpoint, ExecuteEndpointRequestError},
};
use neptunium_model::{
    channel::{Channel, message::Message},
    gateway::payload::incoming::UserPrivateResponse,
};
use tokio::sync::RwLock;

pub mod cachable_endpoints;

trait CacheValue {
    async fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self>;
}

#[async_trait]
pub trait CachableEndpoint: Endpoint {
    /// Either get the result from the cache or execute the request.
    /// # Errors
    /// Returns an error if the HTTP request fails or parsing the response fails.
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<Cached<Self::Response>, Box<ExecuteEndpointRequestError>>;
}

#[async_trait]
pub trait BatchCachableEndpoint: Endpoint {
    type Response;
    async fn execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<<Self as BatchCachableEndpoint>::Response, Box<ExecuteEndpointRequestError>>;
}

// TODO: A better name
#[async_trait]
pub trait NoReturnCachableEndpoint: Endpoint {
    async fn noreturn_execute_cached(
        self,
        client: &Arc<HttpClient>,
        cache: &Arc<Cache>,
    ) -> Result<Self::Response, Box<ExecuteEndpointRequestError>>;
}

impl CacheValue for Channel {
    async fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let channel_id = self.id;
        if let Some(existing_channel) = cache.channels.get(&channel_id) {
            {
                let mut guard = existing_channel.write().await;
                *guard = self;
            }
            return existing_channel;
        }
        let value = Arc::new(RwLock::new(self));
        cache.channels.insert(channel_id, Arc::clone(&value));
        value
    }
}

impl CacheValue for Message {
    async fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let message_id = self.id;
        if let Some(existing_message) = cache.messages.get(&message_id) {
            {
                let mut guard = existing_message.write().await;
                *guard = self;
            }
            return existing_message;
        }
        let value = Arc::new(RwLock::new(self));
        cache.messages.insert(message_id, Arc::clone(&value));
        value
    }
}

impl CacheValue for UserPrivateResponse {
    async fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        if let Some(existing_user) = cache.current_user.get() {
            let existing_user = Arc::clone(existing_user);
            {
                let mut guard = existing_user.write().await;
                *guard = self;
            }
            existing_user
        } else {
            Arc::clone(
                cache
                    .current_user
                    .get_or_init(async || Arc::new(RwLock::new(self)))
                    .await,
            )
        }
    }
}
