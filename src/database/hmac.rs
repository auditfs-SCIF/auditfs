// Partie 2 — HMAC-SHA256 et dérivation de clé
// TODO: dériver une clé depuis un mot de passe (PBKDF2)
// et signer/vérifier les données avec HMAC-SHA256

pub fn derive_key(_password: &str) -> Vec<u8> {
    todo!("Implémenter PBKDF2 pour dériver la clé")
}

pub fn sign(_data: &[u8], _key: &[u8]) -> Vec<u8> {
    todo!("Implémenter la signature HMAC-SHA256")
}

pub fn verify(_data: &[u8], _key: &[u8], _signature: &[u8]) -> bool {
    todo!("Implémenter la vérification HMAC")
}
