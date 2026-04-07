use std::{collections::HashMap, sync::Arc};

use neptunium_model::{
    channel::{
        Channel, ChannelType, PermissionOverwrite, VoiceRegion,
        message::{
            Message, MessageBase, MessageCall, MessageFlags, MessageReaction, MessageReference,
            MessageSnapshot, MessageSticker, MessageType, attachment::MessageAttachment,
            embed::MessageEmbed, nonce::Nonce,
        },
    },
    id::{
        Id,
        marker::{
            ChannelMarker, GuildMarker, MessageMarker, RoleMarker, UserMarker, WebhookMarker,
        },
    },
    time::timestamp::{Timestamp, representations::Iso8601},
    user::PartialUser,
};

use crate::{Cache, CacheValue, Cached, gateway::cached_payload::cache_option_vec};

#[derive(Clone, Debug)]
pub struct CachedChannel {
    /// The bitrate of the voice channel in bits per second
    pub bitrate: Option<i32>,
    /// `None` if this is a DM channel.
    pub guild_id: Option<Id<GuildMarker>>,
    /// The icon hash of the channel (for group DMs)
    pub icon: Option<String>,
    pub id: Id<ChannelMarker>,
    pub last_message_id: Option<Id<MessageMarker>>,
    pub last_pin_timestamp: Option<Timestamp<Iso8601>>,
    pub name: Option<String>,
    /// Custom nicknames for users in this channel (for group DMs)
    pub nicks: Option<HashMap<Id<UserMarker>, String>>,
    pub nsfw: Option<bool>,
    /// The ID of the owner of the channel (for group DMs)
    pub owner_id: Option<Id<UserMarker>>,
    pub parent_id: Option<Id<ChannelMarker>>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub position: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub recipients: Option<Vec<Cached<PartialUser>>>,
    pub rtc_region: Option<VoiceRegion>,
    pub topic: Option<String>,
    pub r#type: ChannelType,
    pub url: Option<String>,
    pub user_limit: Option<i32>,
}

impl CachedChannel {
    /// Converts this cached channel into a normal `Channel`. This is async because it needs to
    /// access all cached recipients to clone them, which are behind an `RwLock`.
    #[must_use]
    pub fn into_channel(self) -> Channel {
        let recipients = if let Some(cached_recipients) = self.recipients {
            let mut recipients = Vec::with_capacity(cached_recipients.len());
            for recipient in cached_recipients {
                recipients.push((*(*recipient.load())).clone());
            }
            Some(recipients)
        } else {
            None
        };

        Channel {
            bitrate: self.bitrate,
            guild_id: self.guild_id,
            icon: self.icon,
            id: self.id,
            last_message_id: self.last_message_id,
            last_pin_timestamp: self.last_pin_timestamp,
            name: self.name,
            nicks: self.nicks,
            nsfw: self.nsfw,
            owner_id: self.owner_id,
            parent_id: self.parent_id,
            permission_overwrites: self.permission_overwrites,
            position: self.position,
            rate_limit_per_user: self.rate_limit_per_user,
            recipients,
            rtc_region: self.rtc_region,
            topic: self.topic,
            r#type: self.r#type,
            url: self.url,
            user_limit: self.user_limit,
        }
    }

    #[must_use]
    pub fn from_channel(value: Channel, cache: &Arc<Cache>) -> Self {
        let recipients = cache_option_vec!(value.recipients, cache);
        Self {
            bitrate: value.bitrate,
            guild_id: value.guild_id,
            icon: value.icon,
            id: value.id,
            last_message_id: value.last_message_id,
            last_pin_timestamp: value.last_pin_timestamp,
            name: value.name,
            nicks: value.nicks,
            nsfw: value.nsfw,
            owner_id: value.owner_id,
            parent_id: value.parent_id,
            permission_overwrites: value.permission_overwrites,
            position: value.position,
            rate_limit_per_user: value.rate_limit_per_user,
            recipients,
            rtc_region: value.rtc_region,
            topic: value.topic,
            r#type: value.r#type,
            url: value.url,
            user_limit: value.user_limit,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CachedMessage {
    pub attachments: Option<Vec<MessageAttachment>>,
    pub author: Cached<PartialUser>,
    pub call: Option<MessageCall>,
    pub channel_id: Id<ChannelMarker>,
    pub content: String,
    pub edited_timestamp: Option<Timestamp<Iso8601>>,
    pub embeds: Option<Vec<MessageEmbed>>,
    pub flags: MessageFlags,
    pub id: Id<MessageMarker>,
    pub mention_everyone: bool,
    pub mention_roles: Option<Vec<Id<RoleMarker>>>,
    pub mentions: Option<Vec<Cached<PartialUser>>>,
    pub message_reference: Option<MessageReference>,
    /// Snapshots of forwarded messages.
    pub message_snapshots: Option<Vec<MessageSnapshot>>,
    /// A client-provided value for message deduplication.
    pub nonce: Option<Nonce>,
    pub pinned: bool,
    pub reactions: Option<Vec<MessageReaction>>,
    pub stickers: Option<Vec<MessageSticker>>,
    pub timestamp: Timestamp<Iso8601>,
    pub tts: bool,
    pub r#type: MessageType,
    pub webhook_id: Option<Id<WebhookMarker>>,
    /// The message that this message is replying to or forwarding.
    pub referenced_message: Option<CachedMessageBase>,
}

impl CachedMessage {
    #[must_use]
    pub fn from_message(value: Message, cache: &Arc<Cache>) -> Self {
        // To make rust happy, we need to destructure
        let Message {
            base:
                MessageBase {
                    attachments,
                    author,
                    call,
                    channel_id,
                    content,
                    edited_timestamp,
                    embeds,
                    flags,
                    id,
                    mention_everyone,
                    mention_roles,
                    mentions,
                    message_reference,
                    message_snapshots,
                    nonce,
                    pinned,
                    reactions,
                    stickers,
                    timestamp,
                    tts,
                    r#type,
                    webhook_id,
                },
            referenced_message,
        } = value;
        let author = author.insert_and_return(cache);
        let mentions = cache_option_vec!(mentions, cache);
        let referenced_message = referenced_message.map(|referenced_message| {
            CachedMessageBase::from_message_base(referenced_message, cache)
        });
        Self {
            attachments,
            author,
            call,
            channel_id,
            content,
            edited_timestamp,
            embeds,
            flags,
            id,
            mention_everyone,
            mention_roles,
            mentions,
            message_reference,
            message_snapshots,
            nonce,
            pinned,
            reactions,
            stickers,
            timestamp,
            tts,
            r#type,
            webhook_id,
            referenced_message,
        }
    }

    #[must_use]
    pub fn into_message(self) -> Message {
        let author = self.author.clone_inner();
        let mentions = if let Some(cached_mentions) = self.mentions {
            let mut mentions = Vec::with_capacity(cached_mentions.len());
            for mention in cached_mentions {
                mentions.push(mention.clone_inner());
            }
            Some(mentions)
        } else {
            None
        };
        let referenced_message = self
            .referenced_message
            .map(CachedMessageBase::into_message_base);
        Message {
            base: MessageBase {
                attachments: self.attachments,
                author,
                call: self.call,
                channel_id: self.channel_id,
                content: self.content,
                edited_timestamp: self.edited_timestamp,
                embeds: self.embeds,
                flags: self.flags,
                id: self.id,
                mention_everyone: self.mention_everyone,
                mention_roles: self.mention_roles,
                mentions,
                message_reference: self.message_reference,
                message_snapshots: self.message_snapshots,
                nonce: self.nonce,
                pinned: self.pinned,
                reactions: self.reactions,
                stickers: self.stickers,
                timestamp: self.timestamp,
                tts: self.tts,
                r#type: self.r#type,
                webhook_id: self.webhook_id,
            },
            referenced_message,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CachedMessageBase {
    pub attachments: Option<Vec<MessageAttachment>>,
    pub author: Cached<PartialUser>,
    pub call: Option<MessageCall>,
    pub channel_id: Id<ChannelMarker>,
    pub content: String,
    pub edited_timestamp: Option<Timestamp<Iso8601>>,
    pub embeds: Option<Vec<MessageEmbed>>,
    pub flags: MessageFlags,
    pub id: Id<MessageMarker>,
    pub mention_everyone: bool,
    pub mention_roles: Option<Vec<Id<RoleMarker>>>,
    pub mentions: Option<Vec<Cached<PartialUser>>>,
    pub message_reference: Option<MessageReference>,
    /// Snapshots of forwarded messages.
    pub message_snapshots: Option<Vec<MessageSnapshot>>,
    /// A client-provided value for message deduplication.
    pub nonce: Option<Nonce>,
    pub pinned: bool,
    pub reactions: Option<Vec<MessageReaction>>,
    pub stickers: Option<Vec<MessageSticker>>,
    pub timestamp: Timestamp<Iso8601>,
    pub tts: bool,
    pub r#type: MessageType,
    pub webhook_id: Option<Id<WebhookMarker>>,
}

impl CachedMessageBase {
    #[must_use]
    pub fn from_message_base(value: MessageBase, cache: &Arc<Cache>) -> Self {
        let author = value.author.insert_and_return(cache);
        let mentions = cache_option_vec!(value.mentions, cache);
        Self {
            attachments: value.attachments,
            author,
            call: value.call,
            channel_id: value.channel_id,
            content: value.content,
            edited_timestamp: value.edited_timestamp,
            embeds: value.embeds,
            flags: value.flags,
            id: value.id,
            mention_everyone: value.mention_everyone,
            mention_roles: value.mention_roles,
            mentions,
            message_reference: value.message_reference,
            message_snapshots: value.message_snapshots,
            nonce: value.nonce,
            pinned: value.pinned,
            reactions: value.reactions,
            stickers: value.stickers,
            timestamp: value.timestamp,
            tts: value.tts,
            r#type: value.r#type,
            webhook_id: value.webhook_id,
        }
    }

    #[must_use]
    pub fn into_message_base(self) -> MessageBase {
        let author = self.author.clone_inner();
        let mentions = if let Some(cached_mentions) = self.mentions {
            let mut mentions = Vec::with_capacity(cached_mentions.len());
            for mention in cached_mentions {
                mentions.push(mention.clone_inner());
            }
            Some(mentions)
        } else {
            None
        };
        MessageBase {
            attachments: self.attachments,
            author,
            call: self.call,
            channel_id: self.channel_id,
            content: self.content,
            edited_timestamp: self.edited_timestamp,
            embeds: self.embeds,
            flags: self.flags,
            id: self.id,
            mention_everyone: self.mention_everyone,
            mention_roles: self.mention_roles,
            mentions,
            message_reference: self.message_reference,
            message_snapshots: self.message_snapshots,
            nonce: self.nonce,
            pinned: self.pinned,
            reactions: self.reactions,
            stickers: self.stickers,
            timestamp: self.timestamp,
            tts: self.tts,
            r#type: self.r#type,
            webhook_id: self.webhook_id,
        }
    }
}
