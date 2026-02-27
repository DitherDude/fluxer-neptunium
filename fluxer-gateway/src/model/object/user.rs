// TODO make all fields public

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::OffsetDateTime;

use crate::{__fluxer_gateway_bitflags_as_number, model::snowflake::Snowflake};

mod locale;
pub use locale::*;
mod settings;
pub use settings::*;

__fluxer_gateway_bitflags_as_number! {
    GuildMemberProfileFlagsDef =>
    #[derive(Copy, Clone, Debug)]
    pub struct GuildMemberProfileFlags: u32 {
        const AVATAR_UNSET = 1 << 0;
        const BANNER_UNSET = 1 << 1;
    }
}

__fluxer_gateway_bitflags_as_number! {
    PublicUserFlagsDef =>
    #[derive(Copy, Clone, Debug)]
    pub struct PublicUserFlags: u32 {
        const STAFF = 1 << 0;
        const CTP_MEMBER = 1 << 1;
        const PARTNER = 1 << 2;
        const BUG_HUNTER = 1 << 3;
        const FRIENDLY_BOT = 1 << 4;
        const FRIENDLY_BOT_MANUAL_APPROVAL = 1 << 5;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPartialResponse {
    /// The unique identifier (snowflake) for this user
    pub id: Snowflake,
    /// The username of the user, not unique across the platform
    pub username: String,
    /// The four-digit discriminator tag of the user
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub avatar_color: Option<i32>,
    /// Whether the user is a bot account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    /// Whether the user is an official system user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    /// The public flags on the user account
    pub flags: PublicUserFlagsDef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildMemberResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banner: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "time::serde::iso8601::option"
    )]
    communication_disabled_until: Option<OffsetDateTime>,
    deaf: bool,
    #[serde(with = "time::serde::iso8601")]
    joined_at: OffsetDateTime,
    mute: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile_flags: Option<GuildMemberProfileFlagsDef>,
    roles: Vec<Snowflake>,
    user: UserPartialResponse,
}

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UserAuthenticatorTypes {
    TimeBasedOneTimePassword = 0,
    SMSBased,
    WebAuthn,
}

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UserPremiumTypes {
    /// No premium subscription.
    None = 0,
    Active,
    Lifetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivateResponsePendingBulkMessageDeletion {
    channel_count: i32,
    message_count: i32,
    #[serde(with = "time::serde::iso8601")]
    scheduled_at: OffsetDateTime,
}

#[expect(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivateResponse {
    pub accent_color: Option<i32>,
    /// Access control list entries for the user
    pub acls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_types: Option<Vec<UserAuthenticatorTypes>>,
    pub avater: Option<String>,
    pub avatar_color: Option<i32>,
    pub banner: Option<String>,
    pub banner_color: Option<i32>,
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    pub discriminator: String,
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_bounced: Option<bool>,
    pub flags: PublicUserFlagsDef,
    pub global_name: Option<String>,
    pub has_dismissed_premium_onboarding: bool,
    pub has_ever_purchased: bool,
    pub has_unread_gift_inventory: bool,
    pub id: Snowflake,
    pub is_staff: bool,
    pub mfa_enabled: bool,
    pub nsfw_allowed: bool,
    #[serde(with = "time::serde::iso8601::option")]
    pub password_last_changed_at: Option<OffsetDateTime>,
    pub pending_bulk_message_deletion: Option<UserPrivateResponsePendingBulkMessageDeletion>,
    pub phone: Option<String>,
    pub premium_badge_hidden: bool,
    pub premium_badge_masked: bool,
    pub premium_badge_sequence_hidden: bool,
    pub premium_badge_timestamp_hidden: bool,
    pub premium_billing_cylcle: Option<String>,
    pub premium_enabled_override: bool,
    pub premium_lifetime_sequence: Option<i32>,
    pub premium_purchase_disabled: bool,
    #[serde(with = "time::serde::iso8601::option")]
    pub premium_since: Option<OffsetDateTime>,
    pub premium_type: Option<UserPremiumTypes>,
    pub premium_will_cancel: bool,
    pub pronouns: Option<String>,
    pub required_actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    pub traits: Vec<String>,
    pub unread_gift_inventory_count: i32,
    pub used_mobile_client: bool,
    pub username: String,
    /// Whether the email address has been verified
    pub verified: bool,
}
