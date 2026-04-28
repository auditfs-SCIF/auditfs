// Partie 2 — Sérialisation / désérialisation de l'IntegrityDB
// TODO: implémenter avec bincode + format binaire signé

use crate::hashing::IntegrityDB;

pub fn save(_db: &IntegrityDB, _path: &str, _password: &str) -> anyhow::Result<()> {
    todo!("Implémenter la sérialisation binaire signée")
}

pub fn load(_path: &str, _password: &str) -> anyhow::Result<IntegrityDB> {
    todo!("Implémenter le chargement et la vérification HMAC")
}
