pub mod messages;

pub struct ApiClient<'a> {
    pub base_path: &'a str,
    pub user_agent: String,
    pub token: zeroize::Zeroizing<String>,
    pub reqwest_client: reqwest::Client,
}
