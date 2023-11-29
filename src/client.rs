#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) app_id: String,
    pub(crate) secret: String,
    pub(crate) client: reqwest::Client,
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
