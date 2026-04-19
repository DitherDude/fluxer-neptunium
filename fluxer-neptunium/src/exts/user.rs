use std::{cmp::Ordering, collections::HashMap};

use crate::{
    client::error::Error, events::context::Context, exts::GuildExt,
    internal::traits::user::UserTrait,
};
use async_trait::async_trait;
use neptunium_cache_inmemory::{
    CachableEndpoint, Cached, CachedChannel, CachedGuildMember, CachedUserProfileFullResponse,
};
use neptunium_http::endpoints::users::{GetUserById, GetUserProfile, GetUserProfileParams};
#[cfg(feature = "user_api")]
use neptunium_model::user::relationship::Relationship;
use neptunium_model::{
    channel::PermissionOverwriteEntity, guild::permissions::Permissions, user::PartialUser,
};

#[async_trait]
pub trait UserExt {
    #[cfg(feature = "user_api")]
    async fn send_friend_request(&self, ctx: &Context) -> Result<Relationship, Error>;
    /// Creates or updates a private note on this user.
    /// Pass `None` for the `note` to clear the note.
    #[cfg(feature = "user_api")]
    async fn set_user_note(&self, ctx: &Context, note: Option<String>) -> Result<(), Error>;
    /// Retrieves a specific note the current user has written about this user.
    #[cfg(feature = "user_api")]
    async fn get_user_note(&self, ctx: &Context) -> Result<String, Error>;
    /// Removes a relationship with another user by ID. Removes friends, cancels
    /// friend requests (incoming or outgoing), or unblocks a blocked user
    /// depending on current relationship type.
    #[cfg(feature = "user_api")]
    async fn remove_relationship(&self, ctx: &Context) -> Result<(), Error>;
    #[cfg(feature = "user_api")]
    async fn update_friend_nickname(
        &self,
        ctx: &Context,
        nickname: Option<String>,
    ) -> Result<Relationship, Error>;
    /// May respect privacy settings.
    async fn get_profile(
        &self,
        ctx: &Context,
        params: GetUserProfileParams,
    ) -> Result<Cached<CachedUserProfileFullResponse>, Error>;
    async fn get_user(&self, ctx: &Context) -> Result<Cached<PartialUser>, Error>;
}

#[async_trait]
impl<T: UserTrait> UserExt for T {
    #[cfg(feature = "user_api")]
    async fn send_friend_request(&self, ctx: &Context) -> Result<Relationship, Error> {
        use neptunium_http::endpoints::users::SendFriendRequest;

        Ok(ctx
            .get_http_client()
            .execute(SendFriendRequest {
                user_id: self.get_user_id(),
            })
            .await?)
    }

    #[cfg(feature = "user_api")]
    async fn set_user_note(&self, ctx: &Context, note: Option<String>) -> Result<(), Error> {
        use neptunium_http::endpoints::users::SetUserNote;

        Ok(ctx
            .get_http_client()
            .execute(SetUserNote {
                user_id: self.get_user_id(),
                note,
            })
            .await?)
    }

    /// Retrieves a specific note the current user has written about another user.
    #[cfg(feature = "user_api")]
    async fn get_user_note(&self, ctx: &Context) -> Result<String, Error> {
        use neptunium_http::endpoints::users::GetUserNote;

        let response = ctx
            .get_http_client()
            .execute(GetUserNote {
                user_id: self.get_user_id(),
            })
            .await?;
        Ok(response.note)
    }

    #[cfg(feature = "user_api")]
    async fn remove_relationship(&self, ctx: &Context) -> Result<(), Error> {
        use neptunium_http::endpoints::users::RemoveRelationship;

        Ok(ctx
            .get_http_client()
            .execute(RemoveRelationship {
                user_id: self.get_user_id(),
            })
            .await?)
    }

    /// Updates the nickname associated with a relationship.
    /// Nicknames are personal labels that override the user’s display name in the current user’s view.
    #[cfg(feature = "user_api")]
    async fn update_friend_nickname(
        &self,
        ctx: &Context,
        nickname: Option<String>,
    ) -> Result<Relationship, Error> {
        use neptunium_http::endpoints::users::UpdateRelationshipNickname;

        Ok(ctx
            .get_http_client()
            .execute(UpdateRelationshipNickname {
                nickname,
                user_id: self.get_user_id(),
            })
            .await?)
    }

    async fn get_profile(
        &self,
        ctx: &Context,
        params: GetUserProfileParams,
    ) -> Result<Cached<CachedUserProfileFullResponse>, Error> {
        Ok(GetUserProfile {
            user_id: self.get_user_id(),
            params,
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }

    async fn get_user(&self, ctx: &Context) -> Result<Cached<PartialUser>, Error> {
        Ok(GetUserById {
            user_id: self.get_user_id(),
        }
        .execute_cached(ctx.get_http_client(), &ctx.cache)
        .await?)
    }
}

pub trait PartialUserExt {
    /// Returns either the display name (global name), or
    /// the username if a global name is not set.
    fn display_name(&self) -> String;
    /// Returns either the display name (global name) with the specified `global_name_prefix`,
    /// or the username with the specified `username_prefix` if a global name is not set.
    fn display_name_formatted(&self, global_name_prefix: &str, username_prefix: &str) -> String;
}

impl PartialUserExt for PartialUser {
    fn display_name(&self) -> String {
        if let Some(global_name) = &self.global_name {
            global_name.clone()
        } else {
            self.username.clone()
        }
    }

    fn display_name_formatted(&self, global_name_prefix: &str, username_prefix: &str) -> String {
        if let Some(global_name) = &self.global_name {
            format!("{global_name_prefix}{global_name}")
        } else {
            format!("{}{}", username_prefix, self.username)
        }
    }
}

#[async_trait]
pub trait GuildMemberExt {
    /// Calculate the member's permissions based on their roles.
    /// To calculate a member's permissions in a specific channel, use `calculate_permissions_in_channel`.
    async fn calculate_permissions(&self, ctx: &Context) -> Result<Permissions, Error>;
    /// Returns `Ok(None)` if the specified channel is not a guild channel.
    /// Note that this does not account for whether the member is the owner of the guild
    /// (and thus bypasses all permission checks).
    async fn calculate_permissions_in_channel(
        &self,
        ctx: &Context,
        channel: &Cached<CachedChannel>,
    ) -> Result<Option<Permissions>, Error>;
    /// Whether this member is the owner of the guild.
    async fn is_guild_owner(&self, ctx: &Context) -> Result<bool, Error>;
    /// Whether the member has the specified permissions. If the member is the owner of the guild,
    /// this function will always return `true`.
    async fn has_permissions(&self, ctx: &Context, permissions: Permissions)
    -> Result<bool, Error>;
    /// Returns `Ok(None)` if the specified channel is not a guild channel. Will always return
    /// `true` if the member is the owner of the guild.
    async fn has_permissions_in_channel(
        &self,
        ctx: &Context,
        channel: &Cached<CachedChannel>,
        permissions: Permissions,
    ) -> Result<Option<bool>, Error>;
}

#[async_trait]
impl GuildMemberExt for CachedGuildMember {
    async fn calculate_permissions(&self, ctx: &Context) -> Result<Permissions, Error> {
        let mut role_permissions = Vec::with_capacity(self.roles.len());
        let roles = self
            .guild_id
            .list_roles(ctx)
            .await?
            .into_iter()
            .map(|role| {
                let role = role.load();
                (role.id, role)
            })
            .collect::<HashMap<_, _>>();
        for role_id in &self.roles {
            let Some(role) = roles.get(role_id) else {
                tracing::warn!("User has role {role_id} but it was not found in the guild.");
                continue;
            };
            role_permissions.push(role.permissions);
        }

        Ok(role_permissions
            .into_iter()
            .fold(Permissions::empty(), |acc, role_perms| {
                acc.union(role_perms)
            }))
    }

    async fn calculate_permissions_in_channel(
        &self,
        ctx: &Context,
        channel: &Cached<CachedChannel>,
    ) -> Result<Option<Permissions>, Error> {
        let channel = channel.load();
        let my_id = self.user.load().id;
        let user_permissions = self.calculate_permissions(ctx).await?;
        let user_roles = &self.roles;
        let Some(mut permission_overwrites) = channel.permission_overwrites.clone() else {
            return Ok(None);
        };
        let role_positions = self
            .guild_id
            .list_roles(ctx)
            .await?
            .into_iter()
            .map(|role| {
                let role = role.load();
                (role.id, role.position)
            })
            .collect::<HashMap<_, _>>();

        // Note that there is no guarantee that the overwrites
        // are in the correct order
        // Source: https://fluxer.app/channels/1427764813854588940/1483532018185537313/1495486102027720271
        // Neither are the user roles:
        // https://fluxer.app/channels/1427764813854588940/1483532018185537313/1495496523828812182

        // Since the permission overwrites will usually (or always?) be in the correct order, this
        // operation will usually take linear time, given the current implementation
        // of sort_by.
        permission_overwrites.sort_by(|a, b| {
            if a.r#type == PermissionOverwriteEntity::Member {
                Ordering::Greater
            } else if b.r#type == PermissionOverwriteEntity::Member {
                Ordering::Less
            } else {
                // Check for @everyone role:
                if a.id.cast() == self.guild_id {
                    return Ordering::Less;
                } else if b.id.cast() == self.guild_id {
                    return Ordering::Greater;
                }
                let Some(a_role_position) = role_positions.get(&a.id.cast()) else {
                    return Ordering::Equal;
                };
                let Some(b_role_position) = role_positions.get(&b.id.cast()) else {
                    return Ordering::Equal;
                };
                let ordering = a_role_position.cmp(b_role_position);
                if ordering == Ordering::Equal {
                    // The position may be equal, in this case the ID is used for ordering.
                    a.id.into_inner().cmp(&b.id.into_inner())
                } else {
                    ordering
                }
            }
        });

        Ok(Some(permission_overwrites.iter().rev().fold(
            user_permissions,
            |mut acc, overwrite| {
                if (overwrite.r#type == PermissionOverwriteEntity::Member
                    && overwrite.id.cast() == my_id)
                    || user_roles.contains(&overwrite.id.cast())
                    || overwrite.id.cast() == self.guild_id
                {
                    acc = acc.difference(overwrite.deny);
                    acc = acc.union(overwrite.allow);
                }
                acc
            },
        )))
    }

    async fn is_guild_owner(&self, ctx: &Context) -> Result<bool, Error> {
        let guild = match ctx.cache.guilds.get(&self.guild_id) {
            Some(guild) => guild,
            None => self.guild_id.fetch(ctx).await?,
        };
        let guild = guild.load();
        Ok(guild.owner_id == self.user.load().id)
    }

    async fn has_permissions(
        &self,
        ctx: &Context,
        permissions: Permissions,
    ) -> Result<bool, Error> {
        if self.is_guild_owner(ctx).await? {
            return Ok(true);
        }
        let user_permissions = self.calculate_permissions(ctx).await?;
        Ok(user_permissions.contains(permissions))
    }

    async fn has_permissions_in_channel(
        &self,
        ctx: &Context,
        channel: &Cached<CachedChannel>,
        permissions: Permissions,
    ) -> Result<Option<bool>, Error> {
        if self.is_guild_owner(ctx).await? {
            return Ok(Some(true));
        }
        let user_channel_permissions = self.calculate_permissions_in_channel(ctx, channel).await?;
        Ok(user_channel_permissions.map(|p| p.contains(permissions)))
    }
}
