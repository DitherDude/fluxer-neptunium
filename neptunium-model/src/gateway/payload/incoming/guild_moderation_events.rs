mod guild_ban_add;
mod guild_ban_remove;

pub use guild_ban_add::*;
pub use guild_ban_remove::*;
use serde::{Deserialize, Serialize};

use crate::id::{Id, marker::UserMarker};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct GuildBanEventUser {
    pub id: Id<UserMarker>,
}
