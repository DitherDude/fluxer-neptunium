use serde::Deserialize;

use crate::{
    channel::Channel,
    gateway::{presence::Presence, voice_state::VoiceState},
    guild::{
        member::GuildMember,
        properties::{
            DefaultMessageNotifications, GuildEmoji, GuildExplicitContentFilter, GuildFeatureFlag,
            GuildMfaLevel, GuildOperations, GuildSticker, GuildVerificationLevel, NsfwLevel,
            SplashCardAlignment, SystemChannelFlags,
        },
    },
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
    misc::ImageHash,
    time::timestamp::{Timestamp, representations::Iso8601},
};

#[derive(Deserialize, Clone, Debug)]
pub struct GuildProperties {
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    pub afk_timeout: u32,
    pub banner: Option<String>,
    pub banner_height: Option<u32>,
    pub banner_width: Option<u32>,
    pub default_message_notifications: DefaultMessageNotifications,
    pub disabled_operations: GuildOperations,
    /// Base64-encoded image data for the embedded invite splash.
    pub embed_splash: Option<String>,
    pub embed_splash_height: Option<i32>,
    pub embed_splash_width: Option<i32>,
    pub explicit_content_filter: GuildExplicitContentFilter,
    pub features: Vec<GuildFeatureFlag>,
    /// Hash of the guild icon
    pub icon: Option<ImageHash>,
    pub id: Id<GuildMarker>,
    pub message_history_cutoff: Option<Timestamp<Iso8601>>,
    pub mfa_level: GuildMfaLevel,
    pub name: String,
    pub nsfw_level: NsfwLevel,
    pub owner_id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    /// Base64-encoded image data for the guild splash screen.
    pub splash: Option<String>,
    pub splash_card_alignment: SplashCardAlignment,
    pub splash_height: Option<i32>,
    pub splash_width: Option<i32>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub vanity_url_code: Option<String>,
    pub verification_level: GuildVerificationLevel,
}

// Figured out by looking at guild response... :(
// TODO: Check official gateway code once fluxer-v2 is released
#[derive(Deserialize, Clone, Debug)]
pub struct GuildCreate {
    pub properties: GuildProperties,
    pub channels: Vec<Channel>,
    pub id: Id<GuildMarker>,
    pub member_count: u64,
    pub online_count: u64,
    pub stickers: Vec<GuildSticker>,
    pub emojis: Vec<GuildEmoji>,
    pub members: Vec<GuildMember>,
    pub presences: Vec<Presence>,
    pub voice_states: Vec<VoiceState>,
    pub joined_at: Timestamp<Iso8601>,
}
