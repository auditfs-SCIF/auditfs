use auditfs::database::hmac;
use auditfs::database::serialize;
use auditfs::hashing::{IntegrityDB, DirectorySnapshot};
use std::collections::HashMap;
use tempfile::tempdir;

fn fake_db() -> IntegrityDB {
    IntegrityDB {
        version: "1.0".to_string(),
        baseline: DirectorySnapshot {
            root: "/test".to_string(),
            files: HashMap::new(),
            created_at: 0,
        },
    }
}

#[test]
#[ignore]
fn test_save_and_load() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("auditfs.db");
    let db = fake_db();
    serialize::save(&db, path.to_str().unwrap(), "secret")?;
    let loaded = serialize::load(path.to_str().unwrap(), "secret")?;
    assert_eq!(db.version, loaded.version);
    assert_eq!(db.baseline.root, loaded.baseline.root);
    Ok(())
}

#[test]
#[ignore]
fn test_wrong_password_fails() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("auditfs.db");
    let db = fake_db();
    serialize::save(&db, path.to_str().unwrap(), "secret").unwrap();
    let result = serialize::load(path.to_str().unwrap(), "mauvais_mdp");
    assert!(result.is_err(), "Un mauvais mot de passe doit échouer");
}

#[test]
#[ignore]
fn test_hmac_signature() {
    let key = hmac::derive_key("secret");
    let data = b"test data";
    let sig = hmac::sign(data, &key);
    assert!(hmac::verify(data, &key, &sig));
    assert!(!hmac::verify(b"wrong", &key, &sig));
}

#[test]
#[ignore]
fn test_hmac_key_derivation_is_deterministic() {
    let key1 = hmac::derive_key("monmotdepasse");
    let key2 = hmac::derive_key("monmotdepasse");
    assert_eq!(key1, key2, "La même clé doit être dérivée pour le même mot de passe");
}

#[test]
fn test_hmac_different_passwords_give_different_keys() {
    let key1 = hmac::derive_key("motdepasse1");
    let key2 = hmac::derive_key("motdepasse2");
    assert_ne!(key1, key2);
}

#[test]
fn test_save_creates_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.db");
    assert!(!path.exists());
    serialize::save(&fake_db(), path.to_str().unwrap(), "secret").unwrap();
    assert!(path.exists(), "Le fichier de base de données doit être créé");
}
