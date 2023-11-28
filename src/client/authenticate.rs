use async_trait::async_trait;
use std::collections::HashMap;

use super::{Client, Session};
use crate::error::{Error, ErrorMessage};

static AUTH_URL: &str = "https://api.weixin.qq.com/sns/jscode2session";

#[async_trait]
pub trait Authenticate {
    async fn login(&self, code: &str) -> Result<Session, Error>;
}

#[async_trait]
impl Authenticate for Client {
    #[tracing::instrument]
    async fn login(&self, code: &str) -> Result<Session, Error> {
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

        tracing::debug!("response: {:#?}", session);

        Ok(session)
    }
}
