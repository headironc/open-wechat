use std::{collections::HashMap, sync::Arc};

use tracing::{event, instrument, Level};

use crate::{
    credential::{AccessTokenBuilder, Credential},
    error::Error::InternalServer,
    response::Response,
    Result,
};

/// 存储微信小程序的 appid 和 secret
#[derive(Debug, Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
}

impl Client {
    pub fn new(app_id: &str, secret: &str) -> Self {
        let client = reqwest::Client::new();

        Self {
            inner: Arc::new(ClientInner {
                app_id: app_id.into(),
                secret: secret.into(),
                client,
            }),
        }
    }

    const AUTHENTICATION: &'static str = "https://api.weixin.qq.com/sns/jscode2session";

    /// 登录凭证校验
    /// https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html
    #[instrument(skip(self))]
    pub async fn login(&self, code: &str) -> Result<Credential> {
        event!(Level::DEBUG, "code: {}", code);

        let mut map: HashMap<&str, &str> = HashMap::new();

        map.insert("appid", &self.inner.app_id);
        map.insert("secret", &self.inner.secret);
        map.insert("js_code", code);
        map.insert("grant_type", "authorization_code");

        let response = self
            .inner
            .client
            .get(Self::AUTHENTICATION)
            .query(&map)
            .send()
            .await?;

        event!(Level::DEBUG, "authentication response: {:#?}", response);

        if response.status().is_success() {
            event!(Level::DEBUG, "get credentials");

            let response = response.json::<Response<Credential>>().await?;

            let credential = response.extract()?;

            event!(Level::DEBUG, "credentials: {:#?}", credential);

            Ok(credential)
        } else {
            Err(InternalServer(response.text().await?))
        }
    }

    const ACCESS_TOKEN: &'static str = "https://api.weixin.qq.com/cgi-bin/token";

    /// 获取小程序全局唯一后台接口调用凭据（access_token）
    /// https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/access-token/auth.getAccessToken.html
    #[instrument(skip(self))]
    pub(crate) async fn get_access_token(&self) -> Result<AccessTokenBuilder> {
        let mut map: HashMap<&str, &str> = HashMap::new();

        map.insert("grant_type", "client_credential");
        map.insert("appid", &self.inner.app_id);
        map.insert("secret", &self.inner.secret);

        let response = self
            .inner
            .client
            .get(Self::ACCESS_TOKEN)
            .query(&map)
            .send()
            .await?;

        event!(Level::DEBUG, "response: {:#?}", response);

        if response.status().is_success() {
            event!(Level::DEBUG, "get access_token");

            let res = response.json::<Response<AccessTokenBuilder>>().await?;

            let builder = res.extract()?;

            event!(Level::DEBUG, "access token builder: {:#?}", builder);

            Ok(builder)
        } else {
            Err(InternalServer(response.text().await?))
        }
    }

    const STABLE_ACCESS_TOKEN: &str = "https://api.weixin.qq.com/cgi-bin/stable_token";

    /// 获取小程序全局唯一后台接口调用凭据（access_token）
    /// https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-access-token/getStableAccessToken.html
    #[instrument(skip(self))]
    pub(crate) async fn get_stable_access_token(
        &self,
        force_refresh: Option<bool>,
    ) -> Result<AccessTokenBuilder> {
        let mut map: HashMap<&str, String> = HashMap::new();

        map.insert("grant_type", "client_credential".into());
        map.insert("appid", self.inner.app_id.clone());
        map.insert("secret", self.inner.secret.clone());

        if let Some(force_refresh) = force_refresh {
            map.insert("force_refresh", force_refresh.to_string());
        }

        let response = self
            .inner
            .client
            .post(Self::STABLE_ACCESS_TOKEN)
            .json(&map)
            .send()
            .await?;

        event!(Level::DEBUG, "response: {:#?}", response);

        if response.status().is_success() {
            event!(Level::DEBUG, "get stable access_token");

            let response = response.json::<Response<AccessTokenBuilder>>().await?;

            let builder = response.extract()?;

            event!(Level::DEBUG, "stable access token builder: {:#?}", builder);

            Ok(builder)
        } else {
            Err(InternalServer(response.text().await?))
        }
    }
}

#[derive(Debug)]
struct ClientInner {
    app_id: String,
    secret: String,
    client: reqwest::Client,
}
