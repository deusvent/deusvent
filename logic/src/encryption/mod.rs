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
    ecdsa_sign, ecdsa_verify, generate_ecc_keys, EccPrivateKey, EccPublicKey, ECC_PRIVATE_KEY_SIZE,
    ECC_PUBLIC_KEY_SIZE, ECC_SIGNATURE_SIZE,
};

use crate::messages::serializers::SerializationError;

mod aes;
mod ecc;

pub const SIGNATURE_SIZE: usize = ECC_SIGNATURE_SIZE;
pub const PUBLIC_KEY_SIZE: usize = ECC_PUBLIC_KEY_SIZE;
pub const PRIVATE_KEY_SIZE: usize = ECC_PRIVATE_KEY_SIZE;

/// Private key - used for signing and decryption
#[derive(uniffi::Object)]
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
#[derive(uniffi::Object)]
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

pub fn generate_new_keys() -> (PrivateKey, PublicKey) {
    let keys = generate_ecc_keys();
    (PrivateKey(keys.0), PublicKey(keys.1))
}

pub fn sign(data: &[u8], private_key: &PrivateKey) -> Vec<u8> {
    ecdsa_sign(data, &private_key.0)
}

pub fn verify(payload: &[u8], public_key: &PublicKey, signature: &[u8]) -> bool {
    ecdsa_verify(payload, &public_key.0, signature)
}

/*

/// Encrypted string with data bytes and salt
pub struct EncryptedString {
    data: Vec<u8>,
    salt: Vec<u8>,
}

/// Safe strings which users may decide to encrypt if that contains sensitive data
pub enum SafeString {
    /// Encrypted string
    Encrypted(EncryptedString),
    /// Raw non encrypted string
    Plaintext(String),
}

/// Signed message which contains of payload, author PublicKey and signature
pub struct SignedMessage {
    payload: Vec<u8>,
    signature: Vec<u8>,
    public_key: PublicKey,
}



impl SignedMessage {
    /// Create signed message by signing payload and user public key
    pub fn create(payload: Vec<u8>, private_key: &PrivateKey, public_key: PublicKey) -> Self {
        // TODO That's inefficient to allocate a third vector just to sign it. We would anyway would call
        //      bincode::encode for both payload and public_key to create a final message data, so
        //      right way would be to create it, then infer a signature from it. Signature is always of a
        //      fixed size, so we can simply add it as a prefix to the final data
        let mut total_data = Vec::with_capacity(payload.len() + public_key.0.as_bytes().len());
        total_data.extend_from_slice(&payload);
        total_data.extend_from_slice(&public_key.0.as_bytes());
        let signature = ecdsa_sign(&private_key.0, &total_data);
        Self {
            payload,
            signature,
            public_key,
        }
    }

    /// Serialize message to bytes
    pub fn serialize(&self) -> Vec<u8> {
        let mut msg = Vec::with_capacity(
            self.payload.len() + self.public_key.0.as_bytes().len() + self.signature.len(),
        );
        // Signature and so as public keys are of a constant length, so we can safely add those first and rest will be a payload
        msg.extend_from_slice(&self.signature);
        msg.extend_from_slice(&self.public_key.0.as_bytes());
        msg.extend_from_slice(&self.payload);
        msg
    }

    /// Deserialize message from bytes and verify the load
    pub fn deserialize(data: &[u8]) -> Self {
        // TODO Check for valid length
        let signature = &data[..SIGNATURE_SIZE];
        let public_key_data = &data[SIGNATURE_SIZE..SIGNATURE_SIZE + PUBLIC_KEY_SIZE];
        let ecc_public_key = EccPublicKey::from_bytes(public_key_data);
        let signed_data = &data[SIGNATURE_SIZE..];
        if !ecdsa_verify(&ecc_public_key, signed_data, signature) {
            panic!("Bad data")
        }
        // TODO So much allocation and data movement just to verify the load?
        Self {
            payload: data[SIGNATURE_SIZE + PUBLIC_KEY_SIZE..].to_vec(),
            signature: signature.to_vec(),
            public_key: PublicKey(ecc_public_key),
        }
    }
}
 */
