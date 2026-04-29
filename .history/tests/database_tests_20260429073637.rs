use auditfs::database::hmac;
use auditfs::database::serialize;
use auditfs::hashing::IntegrityDB;
use std::collections::HashMap;
use tempfile::tempdir;

fn fake_db() -> IntegrityDB {
    IntegrityDB {
        version: "1.0".to_string(),
        baseline: auditfs::hashing::DirectorySnapshot {
            root: "/test".to_string(),
            files: HashMap::new(),
            created_at: 0,
        },
    }
}

#[test]
fn test_save_and_load() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("auditfs.db");
    let db = fake_db();
    serialize::save(&db, path.to_str().unwrap(), "secret")?;
    let loaded = serialize::load(path.to_str().unwrap(), "secret")?;
    assert_eq!(db.version, loaded.version);
    Ok(())
}

#[test]
fn test_hmac_signature() {
    let key = hmac::derive_key("secret");
    let data = b"test data";
    let sig = hmac::sign(data, &key);
    assert!(hmac::verify(data, &key, &sig));
    assert!(!hmac::verify(b"wrong", &key, &sig));
}
