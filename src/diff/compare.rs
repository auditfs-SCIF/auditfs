// Partie 4 — Comparaison de deux DirectorySnapshot
// TODO: détecter fichiers ajoutés, supprimés, modifiés
// Signaler les fichiers world-writable (permissions dangereuses)

use crate::hashing::DirectorySnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ChangeType {
    Added,
    Removed,
    Modified { changed_attributes: Vec<String> },
    DangerousPermission,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffResult {
    pub changes: Vec<(String, ChangeType)>,
}

pub fn compare(_before: &DirectorySnapshot, _after: &DirectorySnapshot) -> DiffResult {
    todo!("Implémenter la comparaison de snapshots")
}
