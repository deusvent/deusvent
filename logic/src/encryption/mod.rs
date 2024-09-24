//! Everything related to encryption, signing and new keys creation.
//!
//! Under the hood we are using AES, ECC and ECDSA, but all the details are abstracted away and convenient
//! wrappers provided instead.
//!
//! TODO Better documentation and describe pretty much everything from https://github.com/deusvent/deusvent/issues/41
//! in here as it's much closer to code and it would be useful in a long run

use std::sync::Arc;

use binary_encoding::encode_base94;
use ecc::{
    ecdsa_sign, ecdsa_verify, generate_ecc_keys, EccPrivateKey, EccPublicKey, EncryptedData,
    ECC_PRIVATE_KEY_SIZE, ECC_PUBLIC_KEY_SIZE, ECC_SIGNATURE_SIZE,
};
use thiserror::Error;

use crate::messages::serializers::SerializationError;

mod aes;
mod ecc;

pub const SIGNATURE_SIZE: usize = ECC_SIGNATURE_SIZE;
pub const PUBLIC_KEY_SIZE: usize = ECC_PUBLIC_KEY_SIZE;
pub const PRIVATE_KEY_SIZE: usize = ECC_PRIVATE_KEY_SIZE;

#[derive(Error, Debug, uniffi::Error)]
pub enum EncryptionError {
    #[error("Invalid data")]
    InvalidData,
}

/// Private key - used for signing and decryption
#[derive(uniffi::Object, Clone)]
pub struct PrivateKey(EccPrivateKey);

#[uniffi::export]
impl PrivateKey {
    pub fn serialize(&self) -> Vec<u8> {
        self.0.serialize()
    }

    #[uniffi::constructor]
    pub fn deserialize(data: Vec<u8>) -> Result<Arc<Self>, SerializationError> {
        EccPrivateKey::deserialize(&data)
            .map(|key| Arc::new(Self(key)))
            .ok_or_else(|| SerializationError::BadData {
                msg: "Invalid private key data".to_string(),
            })
    }
}

/// Public key - used as a public user identifier, for signature verification and encrypting data
/// Although we can always derive public key from a private key it's a good practice to have two types
/// for keys so we can be explicit when one or another is needed, rather than passing private key everywhere
#[derive(uniffi::Object, Clone)]
pub struct PublicKey(EccPublicKey);

#[uniffi::export]
impl PublicKey {
    pub fn serialize(&self) -> Vec<u8> {
        self.0.serialize()
    }

    #[uniffi::constructor]
    pub fn deserialize(data: Vec<u8>) -> Result<Arc<Self>, SerializationError> {
        EccPublicKey::deserialize(&data)
            .map(|key| Arc::new(PublicKey(key)))
            .ok_or_else(|| SerializationError::BadData {
                msg: "Invalid public key data".to_string(),
            })
    }

    pub fn as_string(&self) -> String {
        let data = self.0.serialize();
        encode_base94(&data)
    }
}

#[derive(uniffi::Record)]
pub struct Keys {
    pub public_key: Arc<PublicKey>,
    pub private_key: Arc<PrivateKey>,
}

#[uniffi::export]
pub fn generate_new_keys() -> Keys {
    let keys = generate_ecc_keys();
    Keys {
        private_key: Arc::new(PrivateKey(keys.0)),
        public_key: Arc::new(PublicKey(keys.1)),
    }
}

pub fn sign(data: &[u8], private_key: &PrivateKey) -> Vec<u8> {
    ecdsa_sign(data, &private_key.0)
}

pub fn verify(payload: &[u8], public_key: &PublicKey, signature: &[u8]) -> bool {
    ecdsa_verify(payload, &public_key.0, signature)
}

/// Encrypted string with data bytes and salt
#[derive(PartialEq, Debug, Clone, uniffi::Object, bincode::Encode, bincode::Decode)]
pub struct EncryptedString {
    data: Vec<u8>,
    salt: Vec<u8>,
}

#[uniffi::export]
impl EncryptedString {
    #[uniffi::constructor]
    pub fn new(plaintext: String, private_key: &PrivateKey) -> Arc<Self> {
        let encrypted = ecc::encrypt(plaintext.as_bytes(), &private_key.0);
        Arc::new(Self {
            data: encrypted.data,
            salt: encrypted.salt.to_vec(),
        })
    }

    pub fn decrypt(&self, private_key: &PrivateKey) -> Result<String, EncryptionError> {
        let encrypted = EncryptedData {
            data: self.data.clone(),
            salt: self
                .salt
                .clone()
                .try_into()
                .map_err(|_| EncryptionError::InvalidData)?,
        };
        let decrypted =
            ecc::decrypt(&encrypted, &private_key.0).ok_or(EncryptionError::InvalidData)?;
        let text = String::from_utf8(decrypted).map_err(|_| EncryptionError::InvalidData)?;
        Ok(text)
    }
}

/// Safe strings which users may decide to encrypt if that contains sensitive data
#[derive(PartialEq, Debug, Clone, uniffi::Enum, bincode::Encode, bincode::Decode)]
pub enum SafeString {
    /// Encrypted string
    Encrypted { data: Arc<EncryptedString> },
    /// Raw non encrypted string
    Plaintext { value: String },
}

#[cfg(test)]
mod tests {
    use ecc::ECC_SALT_SIZE;

    use super::*;

    #[test]
    fn test_encrypted_string() {
        let plaintext = "foo".to_string();
        let keys = generate_new_keys();
        let encrypted = EncryptedString::new(plaintext.clone(), &keys.private_key);
        assert_eq!(encrypted.salt.len(), ECC_SALT_SIZE); // Check that salt was created, so as encrypted payload
        let decrypted = encrypted.decrypt(&keys.private_key).unwrap();
        assert_eq!(plaintext, decrypted);
    }
}
