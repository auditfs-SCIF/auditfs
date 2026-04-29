use auditfs::hashing::sha256;
use auditfs::hashing::blake3;
use auditfs::hashing::{FileSnapshot, DirectorySnapshot};
use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
#[ignore]
fn test_sha256_hash() {
    let mut f = NamedTempFile::new().unwrap();
    f.write_all(b"hello world").unwrap();
    let hash = sha256::hash_file(f.path().to_str().unwrap()).unwrap();
    // SHA-256 de "hello world" — valeur connue
    assert_eq!(hash, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
}

#[test]
#[ignore]
fn test_sha256_empty_file() {
    let f = NamedTempFile::new().unwrap();
    let hash = sha256::hash_file(f.path().to_str().unwrap()).unwrap();
    // SHA-256 d'un fichier vide — valeur connue
    assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
}

#[test]
#[ignore]
fn test_sha256_different_contents_give_different_hashes() {
    let mut f1 = NamedTempFile::new().unwrap();
    let mut f2 = NamedTempFile::new().unwrap();
    f1.write_all(b"contenu A").unwrap();
    f2.write_all(b"contenu B").unwrap();
    let h1 = sha256::hash_file(f1.path().to_str().unwrap()).unwrap();
    let h2 = sha256::hash_file(f2.path().to_str().unwrap()).unwrap();
    assert_ne!(h1, h2);
}

#[test]
#[ignore]
fn test_blake3_hash() {
    let mut f = NamedTempFile::new().unwrap();
    f.write_all(b"hello world").unwrap();
    let hash = blake3::hash_file(f.path().to_str().unwrap()).unwrap();
    // BLAKE3 produit un hash hexadécimal de 64 caractères
    assert_eq!(hash.len(), 64);
    assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
#[ignore]
fn test_blake3_deterministic() {
    let mut f = NamedTempFile::new().unwrap();
    f.write_all(b"contenu stable").unwrap();
    let h1 = blake3::hash_file(f.path().to_str().unwrap()).unwrap();
    let h2 = blake3::hash_file(f.path().to_str().unwrap()).unwrap();
    assert_eq!(h1, h2, "Le hash doit être identique pour le même fichier");
}

#[test]
#[ignore]
fn test_blake3_different_contents_give_different_hashes() {
    let mut f1 = NamedTempFile::new().unwrap();
    let mut f2 = NamedTempFile::new().unwrap();
    f1.write_all(b"fichier original").unwrap();
    f2.write_all(b"fichier modifie").unwrap();
    let h1 = blake3::hash_file(f1.path().to_str().unwrap()).unwrap();
    let h2 = blake3::hash_file(f2.path().to_str().unwrap()).unwrap();
    assert_ne!(h1, h2);
}

#[test]
#[ignore]
fn test_file_snapshot_creation() {
    let snap = FileSnapshot {
        path: "/etc/passwd".to_string(),
        size: 1024,
        sha256: "abc123".to_string(),
        blake3: "def456".to_string(),
        permissions: 0o644,
        owner: 0,
        modified_at: 1700000000,
    };
    assert_eq!(snap.path, "/etc/passwd");
    assert_eq!(snap.size, 1024);
    assert_eq!(snap.permissions, 0o644);
}

#[test]
#[ignore]
fn test_directory_snapshot_creation() {
    let mut files = HashMap::new();
    files.insert("/etc/hosts".to_string(), FileSnapshot {
        path: "/etc/hosts".to_string(),
        size: 256,
        sha256: "hash1".to_string(),
        blake3: "hash2".to_string(),
        permissions: 0o644,
        owner: 0,
        modified_at: 0,
    });
    let snap = DirectorySnapshot {
        root: "/etc".to_string(),
        files,
        created_at: 0,
    };
    assert_eq!(snap.files.len(), 1);
    assert!(snap.files.contains_key("/etc/hosts"));
}
