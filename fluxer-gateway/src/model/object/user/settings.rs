use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::serde_as;
use time::OffsetDateTime;

use crate::{
    __fluxer_gateway_bitflags_as_number,
    model::{
        object::{gateway::CustomStatusResponse, user::Locale},
        snowflake::Snowflake,
    },
};

/// Spoiler rendering preference
#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum RenderSpoilers {
    AlwaysReveal = 0,
    OnClick,
    RevealIfModerator,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum TimeFormatTypes {
    AutoDetect = 0,
    TwelveHour,
    TwentyFourHour,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum UserNotificationSettings {
    AllMessages = 0,
    MentionsOnly,
    None,
    InheritFromParent,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum StickerAnimationOptions {
    Always = 0,
    OnHoverOrInteraction,
    Never,
}

__fluxer_gateway_bitflags_as_number! {
    FriendSourceFlagsDef =>
    #[derive(Copy, Clone, Debug)]
    pub struct FriendSourceFlags: u32 {
        const MUTUAL_FRIENDS = 1 << 0;
        const MUTUAL_GUILDS = 1 << 1;
        const NO_RELATION = 1 << 2;
    }
}

__fluxer_gateway_bitflags_as_number! {
    GroupDmAddPermissionFlagsDef =>
    #[derive(Copy, Clone, Debug)]
    pub struct GroupDmAddPermissionFlags: u32 {
        const FRIENDS_OF_FRIENDS = 1 << 0;
        const GUILD_MEMBERS = 1 << 1;
        const EVERYONE = 1 << 2;
        const FRIENDS_ONLY = 1 << 3;
        const NOBODY = 1 << 4;
    }
}

__fluxer_gateway_bitflags_as_number! {
    IncomingCallFlagsDef =>
    #[derive(Copy, Clone, Debug)]
    pub struct IncomingCallFlags: u32 {
        const FRIENDS_OF_FRIENDS = 1 << 0;
        const GUILD_MEMBERS = 1 << 1;
        const EVERYONE = 1 << 2;
        const FRIENDS_ONLY = 1 << 3;
        const NOBODY = 1 << 4;
        const SILENT_EVERYONE = 1 << 5;
    }
}

__fluxer_gateway_bitflags_as_number! {
    GuildFolderFlagsDef =>
    #[derive(Copy, Clone, Debug)]
    pub struct GuildFolderFlags: u32 {
        const SHOW_ICON_WHEN_COLLAPSED = 1 << 0;
    }
}

// In the Fluxer source, this is called `GuildFolderIconSchema`, but in the
// documentation it has this longer name.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum UserSettingsResponseGuildFoldersItemIcon {
    #[serde(rename = "FOLDER")]
    #[default]
    Folder,
    #[serde(rename = "STAR")]
    Star,
    #[serde(rename = "HEART")]
    Heart,
    #[serde(rename = "BOOKMARK")]
    Bookmark,
    #[serde(rename = "GAME_CONTROLLER")]
    GameController,
    #[serde(rename = "SHIELD")]
    Shield,
    #[serde(rename = "MUSIC_NOTE")]
    MusicNote,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGuildSettingsResponseMuteConfig {
    #[serde(with = "time::serde::iso8601::option")]
    end_time: Option<OffsetDateTime>,
    #[serde_as(as = "serde_with::DurationSeconds<u64>")]
    selected_time_window: std::time::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGuildSettingsResponseChannelOverridesValue {
    pub collapsed: bool,
    pub message_notifications: UserNotificationSettings,
    pub muted: bool,
    pub mute_config: Option<UserGuildSettingsResponseMuteConfig>,
}

#[expect(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGuildSettingsResponse {
    pub guild_id: Option<String>,
    /// The default notification level for the guild
    pub message_notifications: UserNotificationSettings,
    /// Whether the guild is muted
    pub muted: bool,
    pub mute_config: Option<UserGuildSettingsResponseMuteConfig>,
    /// Whether mobile push notifications are enabled
    pub mobile_push: bool,
    /// Whether @everyone mentions are suppressed
    pub suppress_everyone: bool,
    /// Whether role mentions are suppressed
    pub suppress_roles: bool,
    /// Whether muted channels are hidden in the sidebar
    pub hide_muted_channels: bool,
    pub channel_overrides: Option<HashMap<String, UserGuildSettingsResponseChannelOverridesValue>>,
    /// The version number of these settings for sync
    pub version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsResponseGuildFoldersItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<GuildFolderFlagsDef>,
    guild_ids: Vec<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<UserSettingsResponseGuildFoldersItemIcon>,
    /// -1 means uncategorized
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[expect(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsResponse {
    afk_timeout: i32,
    animate_emoji: bool,
    animate_stickers: StickerAnimationOptions,
    bot_default_guilds_restricted: bool,
    bot_restricted_guilds: Vec<Snowflake>,
    custom_status: Option<CustomStatusResponse>,
    default_guilds_restricted: bool,
    default_hide_muted_channels: bool,
    developer_mode: bool,
    friend_source_flags: FriendSourceFlagsDef,
    gif_auto_play: bool,
    group_dm_add_permission_flags: GroupDmAddPermissionFlagsDef,
    guild_folders: Vec<UserSettingsResponseGuildFoldersItem>,
    incoming_call_flags: IncomingCallFlagsDef,
    inline_attachment_media: bool,
    locale: Locale,
    message_display_compact: bool,
    render_embeds: bool,
    render_reactions: bool,
    render_spoilers: RenderSpoilers,
    restricted_guilds: Vec<Snowflake>,
    status: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "time::serde::iso8601::option"
    )]
    status_resets_at: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_resets_to: Option<String>,
    theme: String,
    time_format: TimeFormatTypes,
    trusted_domains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteMemeResponse {
    /// Unique identifier for the favorite meme
    pub id: String,
    /// ID of the user who owns this favorite meme
    pub user_id: String,
    /// Display name of the meme
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    /// Tags for categorizing and searching the meme
    pub tags: Vec<String>,
    /// ID of the attachment storing the meme
    pub attachment_id: String,
    /// Original filename of the meme
    pub filename: String,
    /// MIME type of the meme file
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    /// File size in bytes
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// CDN URL to access the meme
    pub url: String,
    /// Whether the meme is a video converted from GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_gifv: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klipy_slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenor_slug_id: Option<String>,
}
