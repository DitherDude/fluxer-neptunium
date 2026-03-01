use fluxer_gateway::model::snowflake::Snowflake;
use reqwest::Method;
use serde_json::json;

use crate::api::ApiClient;

impl ApiClient<'_> {
    pub async fn send_message(&self, channel_id: Snowflake, content: String) {
        let json = json!({
            "content": content,
        });
        let request = self
            .reqwest_client
            .request(
                Method::POST,
                format!(
                    "{}/channels/{}/messages",
                    self.base_path,
                    String::from(channel_id)
                ),
            )
            .header("Authorization", format!("Bot {}", self.token.as_str()))
            .header("User-Agent", &self.user_agent)
            .body(serde_json::to_string(&json).unwrap())
            .build()
            .unwrap();
        self.reqwest_client.execute(request).await.unwrap();
    }
}
