use std::collections::HashMap;

use aes::cipher::generic_array::GenericArray;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use aes::Aes128;
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine};
use cbc::Decryptor;
use serde_json::from_slice;
use tracing::{event, instrument, Level};

use super::{AccessToken, Credentials, PhoneInfo, UserInfo};
use crate::client::Response;
use crate::error::Error::{self, Internal};

type Aes128CbcDec = Decryptor<Aes128>;

static GET_PHONE_NUMBER_URI: &str = "https://api.weixin.qq.com/wxa/business/getuserphonenumber";

pub trait Decrypt {
    fn decrypt(&self, encrypted_data: &str, iv: &str) -> Result<UserInfo, Error>;
}

#[async_trait]
pub trait GetPhoneNumber {
    async fn get_phone_number(&self, code: &str, open_id: Option<&str>)
        -> Result<PhoneInfo, Error>;
}

impl Decrypt for Credentials {
    /// 解密用户数据，使用的是 AES-128-CBC 算法，数据采用PKCS#7填充。
    /// https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/signature.html
    #[instrument(skip(self, encrypted_data, iv))]
    fn decrypt(&self, encrypted_data: &str, iv: &str) -> Result<UserInfo, Error> {
        event!(Level::DEBUG, "encrypted data: {}", encrypted_data);

        let key = STANDARD.decode(self.session_key.as_bytes())?;
        let iv = STANDARD.decode(iv.as_bytes())?;

        let decryptor = Aes128CbcDec::new(
            &GenericArray::clone_from_slice(&key),
            &GenericArray::clone_from_slice(&iv),
        );

        let encrypted_data = STANDARD.decode(encrypted_data.as_bytes())?;

        let buffer = decryptor
            .decrypt_padded_vec_mut::<Pkcs7>(&encrypted_data)
            .map_err(|e| {
                event!(Level::ERROR, "error: {}", e);

                Error::Unpad(e.to_string())
            })?;

        let user_info = from_slice::<UserInfo>(&buffer)?;

        event!(Level::DEBUG, "user info: {:#?}", user_info);

        Ok(user_info)
    }
}

#[async_trait]
impl GetPhoneNumber for AccessToken {
    #[instrument(skip(self, code))]
    async fn get_phone_number(
        &self,
        code: &str,
        open_id: Option<&str>,
    ) -> Result<PhoneInfo, Error> {
        use reqwest::Client;

        event!(Level::DEBUG, "code: {}", code);

        let mut query = HashMap::new();

        let mut body = HashMap::new();

        query.insert("access_token", self.inner.as_str());
        body.insert("code", code);

        if let Some(open_id) = open_id {
            body.insert("openid", open_id);
        }

        let res = Client::new()
            .post(GET_PHONE_NUMBER_URI)
            .query(&query)
            .json(&body)
            .send()
            .await?;

        event!(Level::DEBUG, "response: {:#?}", res);

        if res.status().is_success() {
            event!(Level::DEBUG, "get phone info");

            let res = res.json::<Response<PhoneInfo>>().await?;

            let phone_info = res.get()?;

            event!(Level::DEBUG, "phone info: {:#?}", phone_info);

            Ok(phone_info)
        } else {
            Err(Internal(res.text().await?))
        }
    }
}
