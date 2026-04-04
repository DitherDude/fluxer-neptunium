use async_trait::async_trait;
use neptunium_cache_inmemory::{CachableEndpoint, Cached, CachedChannel};
use neptunium_http::endpoints::{
    channel::{
        AddUserToGroupDm, BulkDeleteMessages, CallEligibilityStatus, ChannelSettingsUpdates,
        CreateMessage, CreateMessageBody, DeleteChannel, DeletePermissionOverwrite,
        GetCallEligibilityStatus, GetChannel, IndicateTyping, ListChannelMessages,
        ListChannelMessagesParams, ListRtcRegions, ListRtcRegionsResponseEntry,
        PermissionOverwriteUpdate, PinDirectMessageChannel, RemoveUserFromGroupDm,
        RingCallRecipients, SetPermissionOverwrite, StopRingingCallRecipients,
        UnpinDirectMessageChannel, UpdateCallRegion, UpdateChannelSettings,
    },
    invites::{CreateChannelInvite, CreateChannelInviteOptions, ListChannelInvites},
    webhooks::{CreateWebhook, ListChannelWebhooks},
};
use neptunium_model::{
    channel::{VoiceRegion, message::Message},
    guild::webhook::Webhook,
    id::{
        Id,
        marker::{GenericMarker, MessageMarker, UserMarker},
    },
    invites::InviteWithMetadata,
};

use crate::{
    client::error::Error, events::context::Context, internal::traits::channel::ChannelTrait,
};

#[async_trait]
pub trait ChannelExt {
    async fn delete(&self, ctx: &Context) -> Result<(), Error>;
    async fn delete_silent(&self, ctx: &Context) -> Result<(), Error>;
    // TODO: Maybe make a builder or something around the ChannelSettingsUpdates
    // because it's annoying to create ig
    async fn update_settings(
        &self,
        ctx: &Context,
        settings: ChannelSettingsUpdates,
    ) -> Result<Cached<CachedChannel>, Error>;
    async fn get(&self, ctx: &Context) -> Result<Cached<CachedChannel>, Error>;
    async fn get_call_eligibility_status(
        &self,
        ctx: &Context,
    ) -> Result<CallEligibilityStatus, Error>;
    /// Update the voice region for an ongoing call.
    async fn update_call_region(&self, ctx: &Context, region: VoiceRegion) -> Result<(), Error>;
    /// Sends ringing notifications to specfied users in a call. If the recipients
    /// are set to `None`, rings all channel members.
    async fn ring_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), Error>;
    /// Stops ringing notifications for specified users in a call. This allows callers
    /// to stop notifying users who have declined or not responded. Pass `None` for the
    /// recipients to stop ringing everyone.
    async fn stop_ringing_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), Error>;
    async fn list_messages(
        &self,
        ctx: &Context,
        params: ListChannelMessagesParams,
    ) -> Result<Vec<Cached<Message>>, Error>;
    async fn bulk_delete_messages(
        &self,
        ctx: &Context,
        messages: Vec<Id<MessageMarker>>,
    ) -> Result<(), Error>;
    /// Same as `create_message`.
    async fn send_message(
        &self,
        ctx: &Context,
        message: CreateMessageBody,
    ) -> Result<Cached<Message>, Error>;
    async fn create_message(
        &self,
        ctx: &Context,
        message: CreateMessageBody,
    ) -> Result<Cached<Message>, Error>;
    async fn set_permission_overwrite(
        &self,
        ctx: &Context,
        update: PermissionOverwriteUpdate,
    ) -> Result<(), Error>;
    async fn delete_permission_overwrite(
        &self,
        ctx: &Context,
        overwrite_id: Id<GenericMarker>,
    ) -> Result<(), Error>;
    #[cfg(feature = "user_api")]
    async fn acknowledge_new_pin_notifications(&self, ctx: &Context) -> Result<(), Error>;
    async fn add_user_to_group_dm(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
    ) -> Result<(), Error>;
    /// Remove a user from a group DM or leave a group DM by specifying
    /// your own user ID. Set `silent` to `true` to suppress the system
    /// message when leaving.
    async fn remove_user_from_group_dm(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
        silent: bool,
    ) -> Result<(), Error>;
    async fn list_rtc_regions(
        &self,
        ctx: &Context,
    ) -> Result<Vec<ListRtcRegionsResponseEntry>, Error>;
    async fn indicate_typing(&self, ctx: &Context) -> Result<(), Error>;
    async fn create_invite(
        &self,
        ctx: &Context,
        options: CreateChannelInviteOptions,
    ) -> Result<InviteWithMetadata, Error>;
    async fn list_invites(&self, ctx: &Context) -> Result<Vec<InviteWithMetadata>, Error>;
    async fn list_webhooks(&self, ctx: &Context) -> Result<Vec<Webhook>, Error>;
    /// Create a webhook in this channel, with the given name and optionally the avatar image as a base64-encoded data URI.
    async fn create_webhook(
        &self,
        ctx: &Context,
        name: String,
        avatar: Option<String>,
    ) -> Result<Webhook, Error>;
    async fn pin_channel(&self, ctx: &Context) -> Result<(), Error>;
    async fn unpin_channel(&self, ctx: &Context) -> Result<(), Error>;
}

#[async_trait]
impl<T: ChannelTrait> ChannelExt for T {
    async fn delete(&self, ctx: &Context) -> Result<(), Error> {
        Ok(DeleteChannel {
            channel_id: self.get_channel_id(),
            silent: None,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn delete_silent(&self, ctx: &Context) -> Result<(), Error> {
        Ok(DeleteChannel {
            channel_id: self.get_channel_id(),
            silent: Some(true),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn update_settings(
        &self,
        ctx: &Context,
        settings: ChannelSettingsUpdates,
    ) -> Result<Cached<CachedChannel>, Error> {
        Ok(UpdateChannelSettings {
            channel_id: self.get_channel_id(),
            updates: settings,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn get(&self, ctx: &Context) -> Result<Cached<CachedChannel>, Error> {
        Ok(GetChannel {
            channel_id: self.get_channel_id(),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn get_call_eligibility_status(
        &self,
        ctx: &Context,
    ) -> Result<CallEligibilityStatus, Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                GetCallEligibilityStatus::builder()
                    .channel_id(self.get_channel_id())
                    .build(),
            )
            .await?)
    }

    async fn update_call_region(&self, ctx: &Context, region: VoiceRegion) -> Result<(), Error> {
        Ok(UpdateCallRegion {
            channel_id: self.get_channel_id(),
            region,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn ring_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                RingCallRecipients::builder()
                    .channel_id(self.get_channel_id())
                    .maybe_recipients(recipients)
                    .build(),
            )
            .await?)
    }

    async fn stop_ringing_call_recipients(
        &self,
        ctx: &Context,
        recipients: Option<Vec<Id<UserMarker>>>,
    ) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(
                StopRingingCallRecipients::builder()
                    .channel_id(self.get_channel_id())
                    .maybe_recipients(recipients)
                    .build(),
            )
            .await?)
    }

    async fn list_messages(
        &self,
        ctx: &Context,
        params: ListChannelMessagesParams,
    ) -> Result<Vec<Cached<Message>>, Error> {
        Ok(ListChannelMessages {
            channel_id: self.get_channel_id(),
            params,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn bulk_delete_messages(
        &self,
        ctx: &Context,
        messages: Vec<Id<MessageMarker>>,
    ) -> Result<(), Error> {
        Ok(BulkDeleteMessages {
            channel_id: self.get_channel_id(),
            messages,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn send_message(
        &self,
        ctx: &Context,
        message: CreateMessageBody,
    ) -> Result<Cached<Message>, Error> {
        self.create_message(ctx, message).await
    }

    async fn create_message(
        &self,
        ctx: &Context,
        message: CreateMessageBody,
    ) -> Result<Cached<Message>, Error> {
        Ok(CreateMessage {
            channel_id: self.get_channel_id(),
            message,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn set_permission_overwrite(
        &self,
        ctx: &Context,
        update: PermissionOverwriteUpdate,
    ) -> Result<(), Error> {
        Ok(SetPermissionOverwrite {
            channel_id: self.get_channel_id(),
            overwrite: update,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn delete_permission_overwrite(
        &self,
        ctx: &Context,
        overwrite_id: Id<GenericMarker>,
    ) -> Result<(), Error> {
        Ok(DeletePermissionOverwrite {
            channel_id: self.get_channel_id(),
            overwrite_id,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    #[cfg(feature = "user_api")]
    async fn acknowledge_new_pin_notifications(&self, ctx: &Context) -> Result<(), Error> {
        use neptunium_http::endpoints::channel::AcknowledgeNewPinNotifications;

        Ok(ctx
            .get_http_client()
            .execute(AcknowledgeNewPinNotifications {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn add_user_to_group_dm(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
    ) -> Result<(), Error> {
        Ok(AddUserToGroupDm {
            channel_id: self.get_channel_id(),
            user_id,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn remove_user_from_group_dm(
        &self,
        ctx: &Context,
        user_id: Id<UserMarker>,
        silent: bool,
    ) -> Result<(), Error> {
        Ok(RemoveUserFromGroupDm {
            channel_id: self.get_channel_id(),
            silent,
            user_id,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }
    // TODO: Caching for all below functions:
    async fn list_rtc_regions(
        &self,
        ctx: &Context,
    ) -> Result<Vec<ListRtcRegionsResponseEntry>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListRtcRegions {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn indicate_typing(&self, ctx: &Context) -> Result<(), Error> {
        Ok(ctx
            .get_http_client()
            .execute(IndicateTyping {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn create_invite(
        &self,
        ctx: &Context,
        options: CreateChannelInviteOptions,
    ) -> Result<InviteWithMetadata, Error> {
        Ok(ctx
            .get_http_client()
            .execute(CreateChannelInvite {
                channel_id: self.get_channel_id(),
                options,
            })
            .await?)
    }

    async fn list_invites(&self, ctx: &Context) -> Result<Vec<InviteWithMetadata>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListChannelInvites {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn list_webhooks(&self, ctx: &Context) -> Result<Vec<Webhook>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListChannelWebhooks {
                channel_id: self.get_channel_id(),
            })
            .await?)
    }

    async fn create_webhook(
        &self,
        ctx: &Context,
        name: String,
        avatar: Option<String>,
    ) -> Result<Webhook, Error> {
        Ok(ctx
            .get_http_client()
            .execute(CreateWebhook {
                channel_id: self.get_channel_id(),
                name,
                avatar,
            })
            .await?)
    }

    async fn pin_channel(&self, ctx: &Context) -> Result<(), Error> {
        ctx.get_http_client()
            .execute(PinDirectMessageChannel {
                channel_id: self.get_channel_id(),
            })
            .await?;
        Ok(())
    }

    async fn unpin_channel(&self, ctx: &Context) -> Result<(), Error> {
        ctx.get_http_client()
            .execute(UnpinDirectMessageChannel {
                channel_id: self.get_channel_id(),
            })
            .await?;
        Ok(())
    }
}
