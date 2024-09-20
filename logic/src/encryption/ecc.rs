//! ECC encryption, keys and signing using ECDH and ECDSA
//! TODO Error handling, better structure, documentation

use hkdf::Hkdf;
use p256::ecdsa::VerifyingKey;
use p256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey};
use p256::{
    elliptic_curve::sec1::ToEncodedPoint,
    elliptic_curve::{PublicKey, SecretKey},
    NistP256,
};
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::Sha256;

use super::aes::{self, AesPayload};

/// ECC keys
pub struct EccKeys {
    private_key: SecretKey<NistP256>,
    public_key: PublicKey<NistP256>,
}

impl EccKeys {
    /// Generate new random pair of ECC keys
    pub fn generate() -> Self {
        let private_key = SecretKey::random(&mut OsRng); // Generate a new private key
        let public_key = private_key.public_key();
        Self {
            private_key,
            public_key,
        }
    }
}

/// Sign payload with given ECC keys
pub fn ecdsa_sign(keys: &EccKeys, payload: &[u8]) -> Vec<u8> {
    let signing_key = SigningKey::from_bytes(&keys.private_key.to_bytes()).unwrap();
    let signature: Signature = signing_key.sign(payload);
    signature.to_vec()
}

/// Verify signature for the supplied payload with given keys
pub fn ecdsa_verify(keys: &EccKeys, payload: &[u8], signature: &[u8]) -> bool {
    let verifying_key =
        VerifyingKey::from_encoded_point(&keys.public_key.to_encoded_point(false)).unwrap();
    let signature = Signature::from_bytes(signature.into()).unwrap();
    verifying_key.verify(payload, &signature).is_ok()
}

/// Encrypted data
pub struct EncryptedData {
    data: AesPayload,
    salt: Vec<u8>,
}

/// Encrypt given payload with supplied ECC keys. Will derive a random AES
/// key from the private key and with salt will use it for encryption
pub fn encrypt(keys: &EccKeys, data: &[u8]) -> EncryptedData {
    // Generate some random 12 bytes salt first
    let mut salt = vec![0u8; 12];
    OsRng.fill_bytes(&mut salt);

    // Now using salt and ECC private key we can derive a new AES key
    let aes_key = derive_aes_key(keys, &salt);

    // Now we can encrypt the payload using AES, reusing generated salt as nonce
    let payload = aes::AesPayload::encrypt(data, &aes_key, &salt).unwrap();

    // Return payload and used salt
    EncryptedData {
        data: payload,
        salt: salt.to_vec(),
    }
}

/// Decrypt the given payload with supplied ECC keys using the provided salt.
/// Will derive the same AES key from the private key and salt to decrypt the payload.
pub fn decrypt(keys: &EccKeys, encrypted_data: &EncryptedData) -> Vec<u8> {
    // Extract the salt used during encryption
    let salt = &encrypted_data.salt;
    let aes_key = derive_aes_key(keys, salt);
    aes::AesPayload::decrypt(&encrypted_data.data, &aes_key, salt).unwrap()
}

fn derive_aes_key(keys: &EccKeys, salt: &Vec<u8>) -> Vec<u8> {
    let hkdf = Hkdf::<Sha256>::new(Some(salt), &keys.private_key.to_bytes());
    let mut aes_key = [0u8; 32];
    hkdf.expand(b"ephemeral-key", &mut aes_key).unwrap();
    aes_key.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_new_keys() {
        // Check that keys can be generated
        let keys = EccKeys::generate();
        assert_eq!(keys.private_key.to_bytes().len(), 32);
        assert_eq!(keys.public_key.to_encoded_point(true).as_bytes().len(), 33);
    }

    #[test]
    fn sign_verify() {
        let payload = vec![1u8; 10];
        let keys = EccKeys::generate();
        let signature = ecdsa_sign(&keys, &payload);
        assert_eq!(signature.len(), 64);
        assert!(ecdsa_verify(&keys, &payload, &signature));
    }

    #[test]
    fn encrypt_decrypt() {
        let data = vec![1u8; 10];
        let keys = EccKeys::generate();
        let encrypted = encrypt(&keys, &data);
        assert_eq!(encrypted.salt.len(), 12);
        let decrypted = decrypt(&keys, &encrypted);
        assert_eq!(decrypted, data);
    }
}
