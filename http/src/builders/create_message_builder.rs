use std::sync::Arc;

use fluxer_model::{
    channel::message::MessageReferenceType,
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker},
    },
};
use futures::future::BoxFuture;
use reqwest::{Error, Response};

use crate::{
    channel::messages::{message_create::CreateMessageBody, message_reference::MessageReference},
    client::HttpClient,
};

pub struct CreateMessageBuilder {
    pub(crate) http_client: Arc<HttpClient>,
    pub(crate) channel_id: Id<ChannelMarker>,
    message: CreateMessageBody,
}

impl CreateMessageBuilder {
    #[must_use]
    pub fn new(http_client: Arc<HttpClient>, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http_client,
            channel_id,
            message: CreateMessageBody::default(),
        }
    }

    #[must_use]
    pub fn reply_to(mut self, message: Id<MessageMarker>) -> Self {
        self.message.message_reference = Some(MessageReference {
            message_id: message,
            channel_id: Some(self.channel_id),
            guild_id: None,
            r#type: MessageReferenceType::Reply,
        });

        self
    }

    #[must_use]
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.message.content = Some(content.into());
        self
    }

    /// Overwrites all previously set values relating the the message body (reply to, content, ...)
    #[must_use]
    pub fn body(mut self, body: impl Into<CreateMessageBody>) -> Self {
        self.message = body.into();
        self
    }

    // TODO other methods
}

impl IntoFuture for CreateMessageBuilder {
    type Output = Result<Response, Error>;
    type IntoFuture = BoxFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.http_client
                .messages(self.channel_id)
                .create(&self.message)
                .await
        })
    }
}
