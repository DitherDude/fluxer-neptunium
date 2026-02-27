use fluxer_gateway::model::{
    event::dispatch::{
        channel::MessageCreateDispatchData,
        session::{GuildReadyResponse, ReadyDispatchData},
    },
    object::user::UserPrivateResponse,
    snowflake::Snowflake,
};

#[derive(Clone, Debug)]
pub struct ReadyEventData {
    pub dispatch_data: ReadyDispatchData,
}

impl ReadyEventData {
    #[must_use]
    pub fn user(&self) -> &UserPrivateResponse {
        &self.dispatch_data.user
    }

    #[must_use]
    pub fn guilds(&self) -> &Vec<GuildReadyResponse> {
        &self.dispatch_data.guilds
    }
}

#[derive(Clone, Debug)]
pub struct MessageCreateEventData {
    pub dispatch_data: MessageCreateDispatchData,
}

#[derive(Clone, Debug)]
pub struct GuildDeleteEventData {
    pub id: Snowflake,
    pub unavailable: bool,
}

#[derive(Clone, Debug)]
pub struct GuildCreateEventData {}
