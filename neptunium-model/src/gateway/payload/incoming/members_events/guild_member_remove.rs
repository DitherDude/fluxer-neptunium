use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::GuildMarker},
    user::MicroUser,
};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct GuildMemberRemove {
    pub guild_id: Id<GuildMarker>,
    pub user: MicroUser,
}
