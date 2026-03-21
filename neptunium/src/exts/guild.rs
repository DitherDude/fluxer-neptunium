use async_trait::async_trait;
use neptunium_http::endpoints::invites::list_guild_invites::ListGuildInvites;
use neptunium_model::{guild::Guild, invites::InviteWithMetadata};

use crate::{client::error::Error, events::context::Context};

#[async_trait]
pub trait GuildExt {
    async fn list_invites(&self, ctx: &Context) -> Result<Vec<InviteWithMetadata>, Error>;
}

#[async_trait]
impl GuildExt for Guild {
    async fn list_invites(&self, ctx: &Context) -> Result<Vec<InviteWithMetadata>, Error> {
        Ok(ctx
            .get_http_client()
            .execute(ListGuildInvites { guild_id: self.id })
            .await?)
    }
}
