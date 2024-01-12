use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{event, instrument, Level};

use super::{Client, Response};
use crate::credentials::Credentials;
use crate::error::Error::{self, Internal};

static AUTH_URL: &str = "https://api.weixin.qq.com/sns/jscode2session";

#[async_trait]
pub trait Authenticate {
    async fn login(&self, code: &str) -> Result<Credentials, Error>;
}

#[async_trait]
impl Authenticate for Client {
    /// 登录凭证校验
    /// https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html
    #[instrument(skip(self, code))]
    async fn login(&self, code: &str) -> Result<Credentials, Error> {
        event!(Level::DEBUG, "code: {}", code);

        let mut hash_map: HashMap<&str, &str> = HashMap::new();

        hash_map.insert("appid", &self.app_id);
        hash_map.insert("secret", &self.secret);
        hash_map.insert("js_code", code);
        hash_map.insert("grant_type", "authorization_code");

        let res = self.client.get(AUTH_URL).query(&hash_map).send().await?;

        event!(Level::DEBUG, "response: {:#?}", res);

        if res.status().is_success() {
            let res = res.json::<Response<Credentials>>().await?;

            let session = res.get()?;

            event!(Level::DEBUG, "session: {:#?}", session);

            Ok(session)
        } else {
            Err(Internal(res.text().await?))
        }
    }
}
