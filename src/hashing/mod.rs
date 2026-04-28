// Partie 1 — Moteur de hashing
// Binôme : Étudiant 1 + Étudiant 2

pub mod sha256;
pub mod blake3;
pub mod types;

pub use types::{FileSnapshot, DirectorySnapshot, IntegrityDB};

use std::fs;
use std::path::Path;

pub fn snapshot_file(path: &str) -> anyhow::Result<FileSnapshot> {
    let metadata = fs::metadata(path)?;

    let sha256 = sha256::hash_file(path)?;
    let blake3  = blake3::hash_file(path)?;

    // Permissions et owner (Unix-style stockés en u32)
    #[cfg(unix)]
    let (permissions, owner) = {
        use std::os::unix::fs::MetadataExt;
        (metadata.mode(), metadata.uid())
    };

    // Sur Windows : pas de permissions Unix réelles, on met 0
    #[cfg(not(unix))]
    let (permissions, owner) = (0u32, 0u32);

    let modified_at = metadata
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;

    Ok(FileSnapshot::new(
        path.to_string(),
        metadata.len(),
        sha256,
        blake3,
        permissions,
        owner,
        modified_at,
    ))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_file() {
        // Crée un fichier temporaire
        let path = "test_temp.txt";
        std::fs::write(path, "contenu de test AuditFS").unwrap();

        let snapshot = snapshot_file(path).unwrap();

        assert_eq!(snapshot.size, 23);
        assert!(!snapshot.sha256.is_empty());
        assert!(!snapshot.blake3.is_empty());
        assert_eq!(snapshot.sha256.len(), 64);  // SHA-256 = 64 hex chars
        assert_eq!(snapshot.blake3.len(), 64);  // BLAKE3 = 64 hex chars

        println!("✅ path:     {}", snapshot.path);
        println!("✅ size:     {}", snapshot.size);
        println!("✅ sha256:   {}", snapshot.sha256);
        println!("✅ blake3:   {}", snapshot.blake3);
        println!("✅ modified: {}", snapshot.modified_at);

        std::fs::remove_file(path).unwrap();
    }
}