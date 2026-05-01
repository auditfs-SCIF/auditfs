use crate::database::{compress, hmac};
use crate::hashing::IntegrityDB;
use std::fs;

pub fn save(db: &IntegrityDB, path: &str, password: &str) -> anyhow::Result<()> {
    let serialized = bincode::serialize(db)?;
    let compressed = compress::compress(&serialized)?;
    let key = hmac::derive_key(password);
    let signature = hmac::sign(&compressed, &key);
    // Format: [4 bytes sig_len][signature][compressed_data]
    let mut output = Vec::new();
    let sig_len = signature.len() as u32;
    output.extend_from_slice(&sig_len.to_le_bytes());
    output.extend_from_slice(&signature);
    output.extend_from_slice(&compressed);
    fs::write(path, output)?;
    Ok(())
}

pub fn load(path: &str, password: &str) -> anyhow::Result<IntegrityDB> {
    let data = fs::read(path)?;
    if data.len() < 4 {
        anyhow::bail!("Fichier corrompu");
    }
    let sig_len = u32::from_le_bytes(data[..4].try_into()?) as usize;
    if data.len() < 4 + sig_len {
        anyhow::bail!("Fichier corrompu");
    }
    let signature = &data[4..4 + sig_len];
    let compressed = &data[4 + sig_len..];
    let key = hmac::derive_key(password);
    if !hmac::verify(compressed, &key, signature) {
        anyhow::bail!("Signature invalide — fichier falsifié ou mot de passe incorrect");
    }
    let decompressed = compress::decompress(compressed)?;
    let db: IntegrityDB = bincode::deserialize(&decompressed)?;
    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hashing::{DirectorySnapshot, IntegrityDB};
    use std::collections::HashMap;
    use tempfile::NamedTempFile;

    fn make_db() -> IntegrityDB {
        IntegrityDB {
            version: "1.0".to_string(),
            baseline: DirectorySnapshot {
                root: "/tmp".to_string(),
                files: HashMap::new(),
                created_at: 0,
            },
        }
    }

    #[test]
    fn test_save_load_roundtrip() {
        let f = NamedTempFile::new().unwrap();
        let db = make_db();
        save(&db, f.path().to_str().unwrap(), "secret").unwrap();
        let loaded = load(f.path().to_str().unwrap(), "secret").unwrap();
        assert_eq!(loaded.version, "1.0");
    }

    #[test]
    fn test_wrong_password_fails() {
        let f = NamedTempFile::new().unwrap();
        let db = make_db();
        save(&db, f.path().to_str().unwrap(), "secret").unwrap();
        let result = load(f.path().to_str().unwrap(), "mauvais");
        assert!(result.is_err());
    }
}