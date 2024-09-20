//! ECC encryption, keys and signing using ECDH and ECDSA
//! TODO Error handling, better structure, documentation

use p256::ecdsa::VerifyingKey;
use p256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey};
use p256::{
    elliptic_curve::sec1::ToEncodedPoint,
    elliptic_curve::{PublicKey, SecretKey},
    NistP256,
};
use rand::rngs::OsRng;

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
}
