//! Low-level AES-GCM encryption building blocks

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
};

/// AES key size in bytes
pub const AES_KEY_SIZE: usize = 32;

/// AES nonces size in bytes
pub const AES_NONCE_SIZE: usize = 12;

/// Encrypt data using AES with provided key and nonce.
/// Panics when key or nonce are of invalid size or when encryption fails: those cases are sign of
/// development error and are not expected to happen in runtime
pub fn aes_encrypt(data: &[u8], key: &[u8; AES_KEY_SIZE], nonce: &[u8; AES_NONCE_SIZE]) -> Vec<u8> {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid AES key");
    cipher
        .encrypt(nonce.into(), data.as_ref())
        .expect("Encryption error")
}

/// Decrypt AES payload using a given key and nonce, returns None if data cannot be decrypted.
/// Panics when key or nonce are of invalid size
pub fn aes_decrypt(
    data: &[u8],
    key: &[u8; AES_KEY_SIZE],
    nonce: &[u8; AES_NONCE_SIZE],
) -> Option<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid AES key");
    cipher.decrypt(nonce.into(), data).ok()
}

#[cfg(test)]
mod tests {
    use aes_gcm::AeadCore;
    use rand::rngs::OsRng;

    use super::*;

    #[test]
    fn encrypt_decrypt() {
        let key = Aes256Gcm::generate_key(&mut OsRng);
        assert_eq!(key.len(), AES_KEY_SIZE);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        assert_eq!(nonce.len(), AES_NONCE_SIZE);

        // Valid data
        let data = vec![10, 20];
        let encrypted = aes_encrypt(&data, &key.into(), &nonce.into());
        let decrypted = aes_decrypt(&encrypted, &key.into(), &nonce.into()).unwrap();
        assert_eq!(decrypted, data);

        // Invalid data
        assert!(aes_decrypt(&data, &key.into(), &nonce.into()).is_none())
    }
}
