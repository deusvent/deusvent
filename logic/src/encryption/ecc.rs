//! Low-level ECC encryption building blocks, keys generation and signing using ECDSA

use hkdf::Hkdf;
use p256::ecdsa::VerifyingKey;
use p256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey};
use p256::elliptic_curve::sec1::FromEncodedPoint;
use p256::elliptic_curve::PublicKey;
use p256::EncodedPoint;
use p256::{elliptic_curve::sec1::ToEncodedPoint, elliptic_curve::SecretKey, NistP256};
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::Sha256;

use super::aes::{aes_decrypt, aes_encrypt, AES_KEY_SIZE, AES_NONCE_SIZE};

/// Size of a ECDSA signature in bytes
pub const ECC_SIGNATURE_SIZE: usize = 64;

/// Size of ECC compressed public key in bytes
pub const ECC_PUBLIC_KEY_SIZE: usize = 33;

/// Size of ECC private key
pub const ECC_PRIVATE_KEY_SIZE: usize = 32;

/// Size of random salt which is added to every encrypted message, same size as AES nonce for convenience
pub const ECC_SALT_SIZE: usize = AES_NONCE_SIZE;
pub struct EccPrivateKey(SecretKey<NistP256>);
pub struct EccPublicKey(PublicKey<NistP256>);

impl EccPublicKey {
    pub fn serialize(&self) -> Vec<u8> {
        self.0.to_encoded_point(true).as_bytes().to_vec()
    }

    pub fn deserialize(data: &[u8]) -> Option<Self> {
        let point = EncodedPoint::from_bytes(data).ok()?;
        if let Some(key) = PublicKey::from_encoded_point(&point).into_option() {
            return Some(Self(key));
        }
        None
    }
}

impl EccPrivateKey {
    pub fn serialize(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    pub fn deserialize(data: &[u8]) -> Option<Self> {
        let key = SecretKey::from_bytes(data.into()).ok()?;
        Some(Self(key))
    }
}

pub fn generate_ecc_keys() -> (EccPrivateKey, EccPublicKey) {
    let private_key = SecretKey::random(&mut OsRng);
    let public_key = private_key.public_key();
    (EccPrivateKey(private_key), EccPublicKey(public_key))
}

pub fn ecdsa_sign(data: &[u8], private_key: &EccPrivateKey) -> Vec<u8> {
    let signing_key = SigningKey::from_bytes(&private_key.0.to_bytes())
        .expect("Signing key should be creatable from a private key bytes");
    let signature: Signature = signing_key.sign(data);
    signature.to_vec()
}

pub fn ecdsa_verify(data: &[u8], public_key: &EccPublicKey, signature: &[u8]) -> bool {
    let verifying_key = VerifyingKey::from_encoded_point(&public_key.0.to_encoded_point(false));
    let signature = Signature::from_bytes(signature.into());
    match (verifying_key, signature) {
        (Ok(key), Ok(signature)) => key.verify(data, &signature).is_ok(),
        _ => false,
    }
}

pub struct EncryptedData {
    data: Vec<u8>,
    salt: [u8; ECC_SALT_SIZE],
}

pub fn encrypt(data: &[u8], private_key: &EccPrivateKey) -> EncryptedData {
    let mut salt = [0; ECC_SALT_SIZE];
    OsRng.fill_bytes(&mut salt);
    let key = derive_aes_key(private_key, &salt);
    let data = aes_encrypt(data, &key, &salt);
    EncryptedData { data, salt }
}

pub fn decrypt(data: &EncryptedData, private_key: &EccPrivateKey) -> Option<Vec<u8>> {
    let aes_key = derive_aes_key(private_key, &data.salt);
    aes_decrypt(&data.data, &aes_key, &data.salt)
}

fn derive_aes_key(private_key: &EccPrivateKey, salt: &[u8; ECC_SALT_SIZE]) -> [u8; AES_KEY_SIZE] {
    let hkdf = Hkdf::<Sha256>::new(Some(salt), &private_key.0.to_bytes());
    let mut aes_key = [0u8; 32];
    hkdf.expand(b"ephemeral-key", &mut aes_key).unwrap();
    aes_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        // Check that keys can be generated
        let (private_key, public_key) = generate_ecc_keys();
        let private_key_bytes = private_key.serialize();
        let public_key_bytes = public_key.serialize();

        // Check sizes
        assert_eq!(private_key_bytes.len(), ECC_PRIVATE_KEY_SIZE);
        assert_eq!(public_key_bytes.len(), ECC_PUBLIC_KEY_SIZE);

        // Test deserialization: we can't check for equality with existing keys, but we can check for
        // serialized data from deserialized keys and that way ensure that deserialized works
        assert_eq!(
            private_key_bytes,
            EccPrivateKey::deserialize(&private_key_bytes)
                .unwrap()
                .serialize()
        );
        assert_eq!(
            public_key_bytes,
            EccPublicKey::deserialize(&public_key_bytes)
                .unwrap()
                .serialize()
        );
    }

    #[test]
    fn sign_verify() {
        let payload = vec![1u8; 10];
        let (private_key, public_key) = generate_ecc_keys();
        let signature = ecdsa_sign(&payload, &private_key);
        assert_eq!(signature.len(), ECC_SIGNATURE_SIZE);
        assert!(ecdsa_verify(&payload, &public_key, &signature));
    }

    #[test]
    fn encrypt_decrypt() {
        let data = vec![1u8; 10];
        let (private_key, _) = generate_ecc_keys();
        let encrypted = encrypt(&data, &private_key);
        assert_eq!(encrypted.salt.len(), ECC_SALT_SIZE);
        let decrypted = decrypt(&encrypted, &private_key).unwrap();
        assert_eq!(decrypted, data);
    }
}
