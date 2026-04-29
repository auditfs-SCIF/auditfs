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
pub struct FileChange {
    pub path: String,
    pub change: ChangeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffResult {
    pub changes: Vec<FileChange>,
}

pub fn compare(before: &DirectorySnapshot, after: &DirectorySnapshot) -> DiffResult {
    let mut changes = Vec::new();

    // Fichiers supprimés
    for path in before.files.keys() {
        if !after.files.contains_key(path) {
            changes.push(FileChange { path: path.clone(), change: ChangeType::Removed });
        }
    }

    // Fichiers ajoutés
    for path in after.files.keys() {
        if !before.files.contains_key(path) {
            changes.push(FileChange { path: path.clone(), change: ChangeType::Added });
        }
    }

    // Fichiers modifiés
    for (path, after_snap) in &after.files {
        if let Some(before_snap) = before.files.get(path) {
            let mut attrs = Vec::new();
            if before_snap.sha256 != after_snap.sha256 { attrs.push("sha256".to_string()); }
            if before_snap.size != after_snap.size { attrs.push("size".to_string()); }
            if before_snap.permissions != after_snap.permissions { attrs.push("permissions".to_string()); }
            if before_snap.modified_at != after_snap.modified_at { attrs.push("modified_at".to_string()); }
            if !attrs.is_empty() {
                changes.push(FileChange { path: path.clone(), change: ChangeType::Modified { changed_attributes: attrs } });
            }
            // World-writable : permissions avec bit 002
            if after_snap.permissions & 0o002 != 0 {
                changes.push(FileChange { path: path.clone(), change: ChangeType::DangerousPermission });
            }
        }
    }

    DiffResult { changes }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hashing::{DirectorySnapshot, FileSnapshot};
    use std::collections::HashMap;

    fn make_snap(files: Vec<(&str, &str)>) -> DirectorySnapshot {
        let mut map = HashMap::new();
        for (path, hash) in files {
            map.insert(path.to_string(), FileSnapshot {
                path: path.to_string(),
                size: 10,
                sha256: hash.to_string(),
                blake3: hash.to_string(),
                permissions: 0o644,
                owner: 1000,
                modified_at: 0,
            });
        }
        DirectorySnapshot { root: "/tmp".to_string(), files: map, created_at: 0 }
    }

    #[test]
    fn test_detect_added() {
        let before = make_snap(vec![]);
        let after = make_snap(vec![("/tmp/new.txt", "abc123")]);
        let diff = compare(&before, &after);
        assert!(diff.changes.iter().any(|c| matches!(c.change, ChangeType::Added)));
    }

    #[test]
    fn test_detect_removed() {
        let before = make_snap(vec![("/tmp/old.txt", "abc123")]);
        let after = make_snap(vec![]);
        let diff = compare(&before, &after);
        assert!(diff.changes.iter().any(|c| matches!(c.change, ChangeType::Removed)));
    }

    #[test]
    fn test_detect_modified() {
        let before = make_snap(vec![("/tmp/f.txt", "hash1")]);
        let after = make_snap(vec![("/tmp/f.txt", "hash2")]);
        let diff = compare(&before, &after);
        assert!(diff.changes.iter().any(|c| matches!(c.change, ChangeType::Modified { .. })));
    }
}
