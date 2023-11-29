use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{debug, instrument};

use crate::client::Client;
use crate::error::{Error, ErrorMessage};
use crate::session::Session;

static AUTH_URL: &str = "https://api.weixin.qq.com/sns/jscode2session";

#[async_trait]
pub trait Authenticate {
    async fn login(&self, code: &str) -> Result<Session, Error>;
}

#[async_trait]
impl Authenticate for Client {
    #[instrument(skip(self, code))]
    async fn login(&self, code: &str) -> Result<Session, Error> {
        debug!("login code: {}", code);

        let mut hash_map: HashMap<&str, &str> = HashMap::new();

        hash_map.insert("appid", &self.app_id);
        hash_map.insert("secret", &self.secret);
        hash_map.insert("js_code", code);
        hash_map.insert("grant_type", "authorization_code");

        let res = self.client.get(AUTH_URL).query(&hash_map).send().await?;

        tracing::debug!("response: {:#?}", res);

        if !res.status().is_success() {
            let error = res.json::<ErrorMessage>().await?;

            tracing::error!("error: {:#?}", error);

            return Err(error.into());
        }

        let session = res.json::<Session>().await?;

        tracing::debug!("session: {:#?}", session);

        Ok(session)
    }
}
