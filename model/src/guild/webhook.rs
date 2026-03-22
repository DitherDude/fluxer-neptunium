use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::{
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker, WebhookMarker},
    },
    misc::{HexColor32, ImageHash},
    user::flags::PublicUserFlags,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebhookUser {
    pub id: Id<UserMarker>,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<ImageHash>,
    pub avatar_color: Option<HexColor32>,
    pub flags: PublicUserFlags,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Webhook {
    pub id: Id<WebhookMarker>,
    pub guild_id: Id<GuildMarker>,
    pub channel_id: Id<ChannelMarker>,
    /// The display name.
    pub name: String,
    pub token: Zeroizing<String>,
    pub user: WebhookUser,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageHash>,
}
