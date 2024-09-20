//! AES-GCM encryption
//!
//! TODO Error handling, better structure, documentation,

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};

/// AES encrypted payload with random generated nonce
pub struct AesPayload {
    encrypted_payload: Vec<u8>,
}

impl AesPayload {
    /// Init AES payload with already encrypted payload
    pub fn new(encrypted_payload: Vec<u8>) -> Self {
        Self { encrypted_payload }
    }

    /// Create new AES payload with data that will be encrypted using a key and provided 12 bytes nonce
    pub fn encrypt(data: &[u8], key: &[u8], nonce: &[u8]) -> Option<Self> {
        let cipher = Aes256Gcm::new_from_slice(key).unwrap();
        let payload = cipher.encrypt(nonce.into(), data.as_ref()).unwrap();
        Some(Self {
            encrypted_payload: payload,
        })
    }

    /// Decrypt AES payload using a given key
    pub fn decrypt(&self, key: &[u8], nonce: &[u8]) -> Option<Vec<u8>> {
        let cipher = Aes256Gcm::new_from_slice(key).ok()?;
        let nonce = Nonce::from_slice(nonce);
        cipher.decrypt(nonce, &*self.encrypted_payload).ok()
    }
}

#[cfg(test)]
mod tests {
    use aes_gcm::AeadCore;
    use rand::rngs::OsRng;

    use super::*;

    #[test]
    fn encrypt_decrypt() {
        let key = Aes256Gcm::generate_key(&mut OsRng);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let data = vec![10, 20];
        let encrypted = AesPayload::encrypt(&data, &key, &nonce).unwrap();
        let decrypted = encrypted.decrypt(&key, &nonce).unwrap();
        assert_eq!(decrypted, data);
    }
}
