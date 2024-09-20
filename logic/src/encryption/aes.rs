//! AES-GCM encryption
//!
//! TODO: Error handling and separate encrypt/decrypt from the AES payload itself

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, Aes256Gcm, Nonce,
};

/// AES encrypted payload with random generated nonce
pub struct AesPayload {
    encrypted_payload: Vec<u8>,
    nonce: Vec<u8>,
}

impl AesPayload {
    /// Init AES payload with already encrypted payload and nonce
    pub fn new(encrypted_payload: Vec<u8>, nonce: Vec<u8>) -> Self {
        Self {
            encrypted_payload,
            nonce,
        }
    }

    /// Create new AES payload with data that will be encrypted using a key
    pub fn encrypt(data: &[u8], key: &[u8]) -> Option<Self> {
        let cipher = Aes256Gcm::new_from_slice(key).unwrap();
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let payload = cipher.encrypt(&nonce, data.as_ref()).unwrap();
        Some(Self {
            encrypted_payload: payload,
            nonce: (*nonce).to_vec(),
        })
    }

    /// Decrypt AES payload using a given key
    pub fn decrypt(&self, key: &[u8]) -> Option<Vec<u8>> {
        let cipher = Aes256Gcm::new_from_slice(key).ok()?;
        let nonce = Nonce::from_slice(&self.nonce);
        cipher.decrypt(nonce, &*self.encrypted_payload).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt() {
        let key = Aes256Gcm::generate_key(&mut OsRng);
        let data = vec![10, 20];
        let encrypted = AesPayload::encrypt(&data, &key).unwrap();
        let decrypted = encrypted.decrypt(&key).unwrap();
        assert_eq!(decrypted, data);
    }
}
