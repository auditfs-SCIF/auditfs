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

impl FileSnapshot {
    pub fn new(
        path: String,
        size: u64,
        sha256: String,
        blake3: String,
        permissions: u32,
        owner: u32,
        modified_at: i64,
    ) -> Self {
        Self { path, size, sha256, blake3, permissions, owner, modified_at }
    }
}

impl DirectorySnapshot {
    pub fn new(root: String) -> Self {
        Self {
            root,
            files: HashMap::new(),
            created_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn add_file(&mut self, snapshot: FileSnapshot) {
        self.files.insert(snapshot.path.clone(), snapshot);
    }
}

impl IntegrityDB {
    pub fn new(baseline: DirectorySnapshot) -> Self {
        Self {
            baseline,
            version: "1.0.0".to_string(),
        }
    }
}