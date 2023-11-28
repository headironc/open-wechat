pub mod authenticate;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Client {
    app_id: String,
    secret: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Session {
    #[serde(rename = "openid")]
    open_id: String,
    session_key: String,
}

impl Session {
    pub fn open_id(&self) -> &str {
        &self.open_id
    }

    pub fn session_key(&self) -> &str {
        &self.session_key
    }
}

impl Client {
    pub fn new(app_id: &str, secret: &str) -> Self {
        let client = reqwest::Client::new();

        Self {
            app_id: app_id.into(),
            secret: secret.into(),
            client,
        }
    }
}
