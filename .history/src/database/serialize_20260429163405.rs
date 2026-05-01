pub mod compress;
pub mod hmac;

// ----- Code de serialize.rs intégré -----
use crate::hashing::IntegrityDB;
use std::fs;
use std::path::Path;

pub fn save(db: &IntegrityDB, path: &str, password: &str) -> anyhow::Result<()> {
    println!("DEBUG: save() called with path = {}", path);

    // Création du répertoire parent
    if let Some(parent) = std::path::Path::new(path).parent() {
        println!("DEBUG: parent directory = {:?}", parent);
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("ERREUR: impossible de créer le répertoire parent : {}", e);
            return Err(e.into());
        }
    }

    let serialized = bincode::serialize(db)?;
    let compressed = compress::compress(&serialized)?;
    let key = hmac::derive_key(password);
    let signature = hmac::sign(&compressed, &key);

    let mut output = Vec::new();
    let sig_len = signature.len() as u32;
    output.extend_from_slice(&sig_len.to_le_bytes());
    output.extend_from_slice(&signature);
    output.extend_from_slice(&compressed);

    match std::fs::write(path, &output) {
        Ok(_) => println!("DEBUG: écriture réussie dans {}", path),
        Err(e) => eprintln!("ERREUR écriture dans {} : {}", path, e),
    }
    std::fs::write(path, output)?;
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
