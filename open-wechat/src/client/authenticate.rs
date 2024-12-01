use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{event, instrument, Level};

use super::{Client, Response};
use crate::credentials::{AccessToken, Credentials};
use crate::error::Error::{self, Internal};

static AUTH_URI: &str = "https://api.weixin.qq.com/sns/jscode2session";
static ACCESS_TOKEN_URI: &str = "https://api.weixin.qq.com/cgi-bin/token";
static STABLE_ACCESS_TOKEN_URI: &str = "https://api.weixin.qq.com/cgi-bin/stable_token";

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

        let res = self.client.get(AUTH_URI).query(&hash_map).send().await?;

        event!(Level::DEBUG, "response: {:#?}", res);

        if res.status().is_success() {
            event!(Level::DEBUG, "get credentials");

            let res = res.json::<Response<Credentials>>().await?;

            let credential = res.get()?;

            event!(Level::DEBUG, "credentials: {:#?}", credential);

            Ok(credential)
        } else {
            Err(Internal(res.text().await?))
        }
    }
}

#[async_trait]
pub trait GetAccessToken {
    async fn get_access_token(&self) -> Result<AccessToken, Error>;
}

#[async_trait]
impl GetAccessToken for Client {
    /// 获取小程序全局唯一后台接口调用凭据（access_token）
    /// https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/access-token/auth.getAccessToken.html
    #[instrument(skip(self))]
    async fn get_access_token(&self) -> Result<AccessToken, Error> {
        let mut hash_map: HashMap<&str, &str> = HashMap::new();

        hash_map.insert("grant_type", "client_credential");
        hash_map.insert("appid", &self.app_id);
        hash_map.insert("secret", &self.secret);

        let res = self
            .client
            .get(ACCESS_TOKEN_URI)
            .query(&hash_map)
            .send()
            .await?;

        event!(Level::DEBUG, "response: {:#?}", res);

        if res.status().is_success() {
            event!(Level::DEBUG, "get access_token");

            let res = res.json::<Response<AccessToken>>().await?;

            let access_token = res.get()?;

            event!(Level::DEBUG, "access_token: {:#?}", access_token);

            Ok(access_token)
        } else {
            Err(Internal(res.text().await?))
        }
    }
}

#[async_trait]
pub trait GetStableAccessToken {
    async fn get_stable_access_token(
        &self,
        force_refresh: Option<bool>,
    ) -> Result<AccessToken, Error>;
}

#[async_trait]
impl GetStableAccessToken for Client {
    /// 获取小程序全局唯一后台接口调用凭据（access_token）
    /// https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-access-token/getStableAccessToken.html
    #[instrument(skip(self))]
    async fn get_stable_access_token(
        &self,
        force_refresh: Option<bool>,
    ) -> Result<AccessToken, Error> {
        let mut hash_map: HashMap<&str, String> = HashMap::new();

        hash_map.insert("grant_type", "client_credential".into());
        hash_map.insert("appid", self.app_id.clone());
        hash_map.insert("secret", self.secret.clone());

        if let Some(force_refresh) = force_refresh {
            hash_map.insert("force_refresh", force_refresh.to_string());
        }

        let res = self
            .client
            .post(STABLE_ACCESS_TOKEN_URI)
            .json(&hash_map)
            .send()
            .await?;

        event!(Level::DEBUG, "response: {:#?}", res);

        if res.status().is_success() {
            event!(Level::DEBUG, "get stable access_token");

            let res = res.json::<Response<AccessToken>>().await?;

            let access_token = res.get()?;

            event!(Level::DEBUG, "stable access_token: {:#?}", access_token);

            Ok(access_token)
        } else {
            Err(Internal(res.text().await?))
        }
    }
}
