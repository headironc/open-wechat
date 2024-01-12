use aes::cipher::generic_array::GenericArray;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use aes::Aes128;
use base64::{engine::general_purpose::STANDARD, Engine};
use cbc::Decryptor;
use serde_json::from_slice;
use tracing::{event, instrument, Level};

use super::{Session, UserInfo};
use crate::error::Error;

type Aes128CbcDec = Decryptor<Aes128>;

pub trait Decrypt {
    fn decrypt(&self, encrypted_data: &str, iv: &str) -> Result<UserInfo, Error>;
}

impl Decrypt for Session {
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
