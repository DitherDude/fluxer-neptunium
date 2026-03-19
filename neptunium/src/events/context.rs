use std::sync::Arc;

use fluxer_model::gateway::payload::outgoing::presence_update::PresenceUpdateOutgoing;
use neptunium_http::client::HttpClient;
use tokio::sync::mpsc::UnboundedSender;

use crate::client::ClientMessage;

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) http_client: Arc<HttpClient>,
    pub(crate) tx: UnboundedSender<ClientMessage>,
}

impl Context {
    #[must_use]
    pub fn get_http_client(&self) -> &Arc<HttpClient> {
        &self.http_client
    }

    pub fn update_presence(&self, data: PresenceUpdateOutgoing) {
        // ignoring potential error caused by the channel being closed
        // TODO: Maybe not ignore it
        let _ = self.tx.send(ClientMessage::UpdatePresence(data));
    }
}
