use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use aes::{
    cipher::{block_padding::Pkcs7, generic_array::GenericArray, BlockDecryptMut, KeyIvInit},
    Aes128,
};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine};
use cbc::Decryptor;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::from_slice;
use tokio::sync::{Notify, RwLock};
use tracing::{event, instrument, Level};

use crate::{
    client::Client,
    error::Error::Unpad,
    user::{User, UserBuilder},
    Result,
};

type Aes128CbcDec = Decryptor<Aes128>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Credential {
    #[serde(rename = "openid")]
    open_id: String,
    session_key: String,
    #[serde(rename = "unionid", skip_serializing_if = "Option::is_none")]
    union_id: Option<String>,
}

impl Credential {
    pub fn open_id(&self) -> &str {
        &self.open_id
    }

    pub fn session_key(&self) -> &str {
        &self.session_key
    }

    pub fn union_id(&self) -> Option<&str> {
        self.union_id.as_deref()
    }

    /// 解密用户数据，使用的是 AES-128-CBC 算法，数据采用PKCS#7填充。
    /// https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/signature.html
    #[instrument(skip(self, encrypted_data, iv))]
    pub fn decrypt(&self, encrypted_data: &str, iv: &str) -> Result<User> {
        event!(Level::DEBUG, "encrypted_data: {}", encrypted_data);
        event!(Level::DEBUG, "iv: {}", iv);

        let key = STANDARD.decode(self.session_key.as_bytes())?;
        let iv = STANDARD.decode(iv.as_bytes())?;

        let decryptor = Aes128CbcDec::new(
            &GenericArray::clone_from_slice(&key),
            &GenericArray::clone_from_slice(&iv),
        );

        let encrypted_data = STANDARD.decode(encrypted_data.as_bytes())?;

        let buffer = decryptor
            .decrypt_padded_vec_mut::<Pkcs7>(&encrypted_data)
            .map_err(Unpad)?;

        let builder = from_slice::<UserBuilder>(&buffer)?;

        event!(Level::DEBUG, "user builder: {:#?}", builder);

        Ok(builder.build())
    }
}

impl std::fmt::Debug for Credential {
    // 为了安全，不打印 session_key
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("open_id", &self.open_id)
            .field("session_key", &"********")
            .field("union_id", &self.union_id)
            .finish()
    }
}

pub struct AccessToken {
    access_token: String,
    expired_at: DateTime<Utc>,
}

impl std::fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessToken")
            .field("access_token", &"********")
            .field("expired_at", &self.expired_at)
            .finish()
    }
}

pub struct StableAccessToken {
    access_token: String,
    expired_at: DateTime<Utc>,
    force_refresh: Option<bool>,
}

impl std::fmt::Debug for StableAccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StableAccessToken")
            .field("access_token", &"********")
            .field("expired_at", &self.expired_at)
            .field("force_refresh", &self.force_refresh)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct GenericAccessToken<T = AccessToken> {
    inner: Arc<RwLock<T>>,
    refreshing: Arc<AtomicBool>,
    notify: Arc<Notify>,
    client: Client,
}

#[async_trait]
pub trait GetAccessToken {
    async fn new(client: Client) -> Result<Self>
    where
        Self: Sized;

    async fn access_token(&self) -> Result<String>;
}

#[async_trait]
impl GetAccessToken for GenericAccessToken<AccessToken> {
    async fn new(client: Client) -> Result<Self> {
        let builder = client.get_access_token().await?;

        Ok(Self {
            inner: Arc::new(RwLock::new(AccessToken {
                access_token: builder.access_token,
                expired_at: builder.expired_at,
            })),
            refreshing: Arc::new(AtomicBool::new(false)),
            notify: Arc::new(Notify::new()),
            client,
        })
    }

    async fn access_token(&self) -> Result<String> {
        event!(Level::DEBUG, "read access token guard");

        let guard = self.inner.read().await;

        if guard.expired_at <= Utc::now() {
            event!(Level::DEBUG, "expired at: {}", guard.expired_at);

            if self.refreshing.load(Ordering::Acquire) {
                event!(Level::DEBUG, "refreshing");

                self.notify.notified().await;
            } else {
                event!(Level::DEBUG, "prepare to fresh");

                self.refreshing.store(true, Ordering::Release);

                drop(guard);

                event!(Level::DEBUG, "write access token guard");

                let mut guard = self.inner.write().await;

                let builder = self.client.get_access_token().await?;

                guard.access_token = builder.access_token;
                guard.expired_at = builder.expired_at;

                self.refreshing.store(false, Ordering::Release);

                self.notify.notify_waiters();

                event!(Level::DEBUG, "fresh access token: {:#?}", guard);

                return Ok(guard.access_token.clone());
            }
        }

        event!(Level::DEBUG, "access token not expired");

        Ok(guard.access_token.clone())
    }
}

#[async_trait]
pub trait GetStableAccessToken {
    async fn new(client: Client, force_refresh: Option<bool>) -> Result<Self>
    where
        Self: Sized;

    async fn access_token(&self) -> Result<String>;

    async fn set_force_refresh(&self, force_refresh: bool) -> Result<()>;
}

#[async_trait]
impl GetStableAccessToken for GenericAccessToken<StableAccessToken> {
    async fn new(client: Client, force_refresh: Option<bool>) -> Result<Self> {
        let builder = client.get_stable_access_token(force_refresh).await?;

        Ok(Self {
            inner: Arc::new(RwLock::new(StableAccessToken {
                access_token: builder.access_token,
                expired_at: builder.expired_at,
                force_refresh,
            })),
            refreshing: Arc::new(AtomicBool::new(false)),
            notify: Arc::new(Notify::new()),
            client,
        })
    }

    async fn access_token(&self) -> Result<String> {
        event!(Level::DEBUG, "read stable access token guard");

        let guard = self.inner.read().await;

        if guard.expired_at <= Utc::now() {
            event!(Level::DEBUG, "expired at: {}", guard.expired_at);

            if self.refreshing.load(Ordering::Acquire) {
                event!(Level::DEBUG, "refreshing");

                self.notify.notified().await;
            } else {
                event!(Level::DEBUG, "prepare to fresh");

                self.refreshing.store(true, Ordering::Release);

                drop(guard);

                event!(Level::DEBUG, "write stable access token guard");

                let mut guard = self.inner.write().await;

                let builder = self
                    .client
                    .get_stable_access_token(guard.force_refresh)
                    .await?;

                guard.access_token = builder.access_token;
                guard.expired_at = builder.expired_at;

                self.refreshing.store(false, Ordering::Release);

                self.notify.notify_waiters();

                event!(Level::DEBUG, "fresh stable access token: {:#?}", guard);

                return Ok(guard.access_token.clone());
            }
        }

        event!(Level::DEBUG, "stable access token not expired");

        Ok(guard.access_token.clone())
    }

    async fn set_force_refresh(&self, force_refresh: bool) -> Result<()> {
        let mut guard = self.inner.write().await;

        guard.force_refresh = Some(force_refresh);

        Ok(())
    }
}

#[derive(Deserialize)]
pub(crate) struct AccessTokenBuilder {
    access_token: String,
    #[serde(
        deserialize_with = "AccessTokenBuilder::deserialize_expired_at",
        rename = "expires_in"
    )]
    expired_at: DateTime<Utc>,
}

impl AccessTokenBuilder {
    fn deserialize_expired_at<'de, D>(
        deserializer: D,
    ) -> std::result::Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = Duration::seconds(i64::deserialize(deserializer)?);

        Ok(Utc::now() + seconds)
    }
}

impl std::fmt::Debug for AccessTokenBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessTokenBuilder")
            .field("access_token", &"********")
            .field("expired_at", &self.expired_at)
            .finish()
    }
}
