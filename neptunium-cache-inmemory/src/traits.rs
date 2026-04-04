use std::sync::Arc;

use crate::{Cache, Cached};
use async_trait::async_trait;
use neptunium_http::{
    client::HttpClient,
    endpoints::{Endpoint, ExecuteEndpointRequestError},
};
use neptunium_model::channel::{Channel, message::Message};
use tokio::sync::RwLock;

pub mod cachable_endpoints;
/*
pub trait CacheKey: Copy {
    type Value;
    fn get(&self, cache: &Arc<Cache>) -> Option<Cached<Self::Value>>;
    fn remove(&self, cache: &Arc<Cache>);
}
*/
trait CacheValue {
    fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self>;
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
/*
impl CacheKey for Id<UserMarker> {
    type Value = PartialUser;

    fn get(&self, cache: &Arc<Cache>) -> Option<Cached<Self::Value>> {
        cache.users.get(self)
    }

    fn remove(&self, cache: &Arc<Cache>) {
        cache.users.invalidate(self);
    }
}
*/
/*
impl CacheValue for PartialUser {
    fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let id = self.id;
        let value = Arc::new(RwLock::new(self));
        cache.users.insert(id, Arc::clone(&value));
        value
    }
}
*/
/*
impl CacheKey for UserProfileCacheKey {
    type Value = UserProfileFullResponse;
    fn get(&self, cache: &Arc<Cache>) -> Option<Cached<Self::Value>> {
        cache.user_profiles.get(self)
    }
    fn remove(&self, cache: &Arc<Cache>) {
        cache.user_profiles.invalidate(self);
    }
}
*/

impl CacheValue for Channel {
    fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let channel_id = self.id;
        let value = Arc::new(RwLock::new(self));
        cache.channels.insert(channel_id, Arc::clone(&value));
        value
    }
}

impl CacheValue for Message {
    fn insert_and_return(self, cache: &Arc<Cache>) -> Cached<Self> {
        let message_id = self.id;
        let value = Arc::new(RwLock::new(self));
        cache.messages.insert(message_id, Arc::clone(&value));
        value
    }
}