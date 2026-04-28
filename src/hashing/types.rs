// Partie 1 — Types de données principaux
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSnapshot {
    pub path: String,
    pub size: u64,
    pub sha256: String,
    pub blake3: String,
    pub permissions: u32,
    pub owner: u32,
    pub modified_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectorySnapshot {
    pub root: String,
    pub files: HashMap<String, FileSnapshot>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityDB {
    pub baseline: DirectorySnapshot,
    pub version: String,
}

// TODO: implémenter les méthodes nécessaires
