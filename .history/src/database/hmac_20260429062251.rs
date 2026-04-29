use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Dérivation de clé simple basée sur HMAC-SHA256 (100 000 itérations)
pub fn derive_key(password: &str) -> Vec<u8> {
    let salt = b"auditfs-salt-v1";
    let mut key = vec![0u8; 32];
    // PBKDF2 simplifié : itérations HMAC
    let mut mac = HmacSha256::new_from_slice(password.as_bytes()).expect("HMAC init");
    mac.update(salt);
    let mut prev = mac.finalize().into_bytes().to_vec();
    key[..prev.len().min(32)].copy_from_slice(&prev[..prev.len().min(32)]);
    for _ in 1..100_000 {
        let mut mac2 = HmacSha256::new_from_slice(password.as_bytes()).expect("HMAC init");
        mac2.update(&prev);
        prev = mac2.finalize().into_bytes().to_vec();
        for (k, p) in key.iter_mut().zip(prev.iter()) {
            *k ^= p;
        }
    }
    key
}

pub fn sign(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC key invalide");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

pub fn verify(data: &[u8], key: &[u8], signature: &[u8]) -> bool {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC key invalide");
    mac.update(data);
    mac.verify_slice(signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify() {
        let key = derive_key("motdepasse");
        let data = b"donnees importantes";
        let sig = sign(data, &key);
        assert!(verify(data, &key, &sig));
    }

    #[test]
    fn test_verify_fails_wrong_data() {
        let key = derive_key("motdepasse");
        let sig = sign(b"original", &key);
        assert!(!verify(b"modifie", &key, &sig));
    }

    #[test]
    fn test_derive_key_length() {
        let key = derive_key("test");
        assert_eq!(key.len(), 32);
    }
}
