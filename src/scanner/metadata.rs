use crate::hashing::FileSnapshot;
use crate::hashing::{blake3, sha256};
use std::fs;

pub fn collect(path: &str) -> anyhow::Result<FileSnapshot> {
    let meta = fs::metadata(path)?;
    #[cfg(unix)]
    let (permissions, owner) = {
        use std::os::unix::fs::MetadataExt;
        (meta.mode(), meta.uid())
    };
    #[cfg(not(unix))]
    let (permissions, owner) = (0u32, 0u32);

    let modified_at = meta
        .modified()
        .map(|t| {
            t.duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64
        })
        .unwrap_or(0);

    Ok(FileSnapshot {
        path: path.to_string(),
        size: meta.len(),
        sha256: sha256::hash_file(path)?,
        blake3: blake3::hash_file(path)?,
        permissions,
        owner,
        modified_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_collect_metadata() {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(b"contenu test").unwrap();
        let snap = collect(f.path().to_str().unwrap()).unwrap();
        assert_eq!(snap.size, 12);
        assert_eq!(snap.sha256.len(), 64);
        assert_eq!(snap.blake3.len(), 64);
    }
}
