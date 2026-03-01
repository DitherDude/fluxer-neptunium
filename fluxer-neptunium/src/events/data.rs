use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use debug_ignore::DebugIgnore;
use fluxer_gateway::model::{
    event::dispatch::{channel::MessageCreateDispatchData, session::ReadyDispatchData},
    snowflake::Snowflake,
};

use crate::Client;

#[derive(Clone, Debug)]
pub struct ReadyEventData {
    pub dispatch_data: ReadyDispatchData,
}

impl Deref for ReadyEventData {
    type Target = ReadyDispatchData;
    fn deref(&self) -> &Self::Target {
        &self.dispatch_data
    }
}

impl DerefMut for ReadyEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dispatch_data
    }
}

#[derive(Clone, Debug)]
pub struct MessageCreateEventData<'a> {
    pub dispatch_data: MessageCreateDispatchData,
    pub(crate) client: DebugIgnore<Arc<tokio::sync::Mutex<Client<'a>>>>,
}

impl Deref for MessageCreateEventData<'_> {
    type Target = MessageCreateDispatchData;
    fn deref(&self) -> &Self::Target {
        &self.dispatch_data
    }
}

impl DerefMut for MessageCreateEventData<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dispatch_data
    }
}

impl MessageCreateEventData<'_> {
    // WIP
    pub async fn reply(&self, content: String) {
        self.client
            .lock()
            .await
            .api_client
            .send_message(
                self.dispatch_data.message_response.channel_id.clone(),
                content,
            )
            .await;
    }
}

#[derive(Clone, Debug)]
pub struct GuildDeleteEventData {
    pub id: Snowflake,
    pub unavailable: bool,
}

#[derive(Clone, Debug)]
pub struct GuildCreateEventData {}
