use std::{sync::Arc, time::Duration};

use fluxer_gateway::{
    client::{
        GatewayClient, GatewayConnectionWriteHalf, client_config::GatewayClientConfiguration,
    },
    model::event::{
        GatewayEvent, IncomingGatewayEventData, IncomingGatewayOpCode, OutgoingGatewayEventData,
        dispatch::DispatchEvent, heartbeat::OutgoingHeartbeatEventData,
    },
};
use tokio::sync::{
    Mutex,
    mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
};

pub use async_trait::async_trait;
pub use error::Error;
pub use fluxer_gateway::client::client_config::GatewayIntents;
use tracing::Level;

mod api;
mod error;
pub mod events;

use crate::{
    api::ApiClient,
    error::NeptuniumErrorKind,
    events::{
        Event, EventBus, EventListener, MessageCreateEventData,
        data::{GuildDeleteEventData, ReadyEventData},
    },
};

const DEFAULT_API_URL: &str = "https://api.fluxer.app/v1";
const USER_AGENT: &str = "Fluxer-Neptunium";
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EventType {
    Ready,
}

enum ClientMessage {
    #[expect(unused)]
    SendMessage(OutgoingGatewayEventData),
    SendHeartbeat,
    Received(GatewayEvent<IncomingGatewayEventData, IncomingGatewayOpCode>),
}

pub struct Client<'a> {
    #[expect(clippy::struct_field_names)]
    gateway_client: GatewayClient<'a>,
    #[expect(clippy::struct_field_names)]
    pub(crate) api_client: ApiClient<'a>,
    last_sequence_number: Option<u64>,
    event_bus: Option<EventBus>,
    tx: UnboundedSender<ClientMessage>,
    rx: Option<UnboundedReceiver<ClientMessage>>,
}

impl<'a> From<GatewayClient<'a>> for Client<'a> {
    fn from(value: GatewayClient<'a>) -> Self {
        let token = value.config.token.to_string();
        let (tx, rx) = unbounded_channel();
        Self {
            gateway_client: value,
            last_sequence_number: None,
            event_bus: None,
            tx,
            rx: Some(rx),
            api_client: Self::create_api_client(token),
        }
    }
}

impl<'a> Client<'a> {
    /// Construct a new client given a `token` and the `GatewayIntents`
    #[must_use]
    pub fn new(token: &'a str, intents: GatewayIntents) -> Self {
        let token_clone = token.to_string();
        let (tx, rx) = unbounded_channel();
        Self {
            gateway_client: GatewayClient::new(GatewayClientConfiguration::new(token, intents)),
            last_sequence_number: None,
            event_bus: None,
            tx,
            rx: Some(rx),
            api_client: Self::create_api_client(token_clone),
        }
    }

    /// Set the API base path.
    /// The default is `https://api.fluxer.app/v1`.
    pub fn set_api_base_path(&mut self, base: &'a str) {
        self.api_client.base_path = base;
    }

    /// Set the gateway URL.
    /// The default is `wss://gateway.fluxer.app`.
    pub fn set_gateway_url(&mut self, url: &'a str) {
        self.gateway_client.config.gateway_url = url;
    }

    /// Start the client. Waits for events from the gateway and calls the registered event handlers.
    /// This function blocks forever unless a fatal error occurs.
    /// # Errors
    /// Returns an error if a fatal error occurs, such as the connection closing or the gateway sending
    /// an unexpected event.
    pub async fn start(mut self) -> Result<(), crate::error::Error> {
        tracing::event!(
            Level::DEBUG,
            "Starting fluxer neptunium client (version: {})",
            VERSION
        );
        let tx = self.tx.clone();
        let Some(rx) = self.rx.take() else {
            return Err(Error::new(NeptuniumErrorKind::InvalidInternalState));
        };
        let (mut write, mut read) = self
            .gateway_client
            .establish_connection()
            .await
            .map_err(Into::<Error>::into)?;
        tracing::event!(Level::TRACE, "Successfully established connection");
        let next_event = match GatewayClient::next_event(&mut read).await {
            Ok(event) => event,
            Err(e) => return Err(e.into()),
        };
        let IncomingGatewayEventData::Hello(hello_data) = next_event.data else {
            return Err(Error::new(NeptuniumErrorKind::UnexpectedEvent(next_event)));
        };

        let heartbeat_interval = hello_data.heartbeat_interval;

        let event_bus = self.event_bus.take().unwrap_or(EventBus::new());

        let client = Arc::new(tokio::sync::Mutex::new(self));

        let cloned_tx = tx.clone();
        tokio::spawn(async move {
            let wait_time = rand::random_range(0..heartbeat_interval);
            tokio::time::sleep(Duration::from_millis(wait_time)).await;
            let _ = cloned_tx.send(ClientMessage::SendHeartbeat);
        });
        let cloned_tx = tx.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(heartbeat_interval)).await;
                if cloned_tx.send(ClientMessage::SendHeartbeat).is_err() {
                    // If tx.send() returns an error it means that the channel has been closed
                    // which indicates that this task should end too
                    break;
                }
            }
        });
        let cloned_tx = tx.clone();
        tokio::spawn(async move {
            loop {
                let next_event = match GatewayClient::next_event(&mut read).await {
                    Ok(event) => event,
                    Err(e) => {
                        tracing::warn!("Gateway client error: {}", e);
                        break;
                    }
                };
                if cloned_tx.send(ClientMessage::Received(next_event)).is_err() {
                    tracing::debug!(
                        "Message receiver thread is stopping due to channel being closed."
                    );
                    break;
                }
            }
            tracing::warn!("Message receiver thread is stopping.");
        });

        if let Err(e) = client
            .lock()
            .await
            .gateway_client
            .identify(&mut write)
            .await
        {
            return Err(e.into());
        }

        Self::handle_messages(client, tx, rx, event_bus, write).await
    }

    async fn handle_messages(
        client: Arc<Mutex<Self>>,
        tx: UnboundedSender<ClientMessage>,
        mut rx: UnboundedReceiver<ClientMessage>,
        mut event_bus: EventBus,
        mut write: GatewayConnectionWriteHalf,
    ) -> Result<(), crate::error::Error> {
        while let Some(message) = rx.recv().await {
            match message {
                ClientMessage::SendMessage(message) => {
                    if let Err(e) = GatewayClient::send(&mut write, message).await {
                        return Err(e.into());
                    }
                }
                ClientMessage::SendHeartbeat => {
                    let message = OutgoingGatewayEventData::Heartbeat(OutgoingHeartbeatEventData {
                        last_sequence_number: client.lock().await.last_sequence_number,
                    });
                    if let Err(e) = GatewayClient::send(&mut write, message).await {
                        return Err(e.into());
                    }
                }
                ClientMessage::Received(event) => {
                    let event_sequence_number = event.payload.s;
                    match event.data {
                        IncomingGatewayEventData::HeartbeatAck => {}
                        IncomingGatewayEventData::Hello(_) => {
                            return Err(Error::new(NeptuniumErrorKind::UnexpectedEvent(event)));
                        }
                        IncomingGatewayEventData::Heartbeat => {
                            let _ = tx.send(ClientMessage::SendHeartbeat);
                        }
                        IncomingGatewayEventData::InvalidSession(data) => {
                            if !data.resumable {
                                return Err(Error::new(NeptuniumErrorKind::SessionInvalidated));
                            }

                            todo!("resuming/reconnecting");
                        }
                        IncomingGatewayEventData::Reconnect => todo!("reconnecting"),
                        IncomingGatewayEventData::Dispatch(event) => {
                            tracing::trace!("Event sequence number: {:?}", event_sequence_number);
                            if let Some(last_sequence_number) = event_sequence_number {
                                client.lock().await.last_sequence_number =
                                    Some(last_sequence_number);
                            }
                            match *event {
                                DispatchEvent::Ready(data) => {
                                    event_bus
                                        .emit(
                                            Event::Ready(Box::new(ReadyEventData {
                                                dispatch_data: *data,
                                            })),
                                            Arc::clone(&client),
                                        )
                                        .await;
                                }
                                DispatchEvent::GuildDelete(data) => {
                                    event_bus
                                        .emit(
                                            Event::GuildDelete(GuildDeleteEventData {
                                                id: data.id,
                                                unavailable: data.unavailable.unwrap_or(false),
                                            }),
                                            Arc::clone(&client),
                                        )
                                        .await;
                                }
                                DispatchEvent::GuildCreate(_data) => { /* TODO */ }
                                DispatchEvent::MessageCreate(data) => {
                                    event_bus
                                        .emit(
                                            Event::MessageCreate(Box::new(
                                                MessageCreateEventData {
                                                    dispatch_data: *data,
                                                    client: Arc::clone(&client).into(),
                                                },
                                            )),
                                            Arc::clone(&client),
                                        )
                                        .await;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn register_event_listener(&mut self, listener: impl EventListener + Send + 'static) {
        self.event_bus
            .get_or_insert(EventBus::new())
            .register(Box::new(listener) as Box<dyn EventListener + Send>);
    }

    // pub(crate) fn send_client_message(
    //     &self,
    //     message: ClientMessage,
    // ) -> Result<(), tokio::sync::mpsc::error::SendError<ClientMessage>> {
    //     self.tx.send(message)
    // }

    fn create_api_client(token: String) -> ApiClient<'a> {
        ApiClient {
            base_path: DEFAULT_API_URL,
            user_agent: format!("{USER_AGENT}/{VERSION}"),
            token: token.into(),
            reqwest_client: reqwest::Client::new(),
        }
    }
}
