use bon::Builder;
use neptunium_model::id::{
    Id,
    marker::{GuildMarker, RoleMarker, UserMarker},
};
use reqwest::Method;

use crate::{endpoints::Endpoint, request::Request};

#[derive(Builder, Copy, Clone, Debug)]
pub struct AddRoleToGuildMember {
    pub guild_id: Id<GuildMarker>,
    pub user_id: Id<UserMarker>,
    pub role_id: Id<RoleMarker>,
}

impl Endpoint for AddRoleToGuildMember {
    type Response = ();

    fn into_request(self) -> crate::request::Request {
        Request::builder()
            .method(Method::PUT)
            .path(format!(
                "/guilds/{}/members/{}/roles/{}",
                self.guild_id, self.user_id, self.role_id
            ))
            .build()
    }
}
