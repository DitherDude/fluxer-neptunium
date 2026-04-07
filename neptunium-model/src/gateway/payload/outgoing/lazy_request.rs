use std::collections::HashMap;

use bon::Builder;
use serde::Serialize;

use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker, UserMarker},
};

#[derive(Serialize, Clone, Debug, Builder)]
#[expect(clippy::type_complexity)]
pub struct GuildSubscriptionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    // TODO: What do those two numbers do?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_list_channels: Option<HashMap<Id<ChannelMarker>, Vec<(i64, i64)>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Id<UserMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
}

#[derive(Serialize, Clone, Debug, Builder)]
pub struct LazyRequest {
    pub subscriptions: HashMap<Id<GuildMarker>, GuildSubscriptionRequest>,
}
