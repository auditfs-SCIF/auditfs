use crate::hashing::DirectorySnapshot;
use crate::scanner::{metadata, symlink};
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn scan(root: &str) -> anyhow::Result<DirectorySnapshot> {
    let mut files = HashMap::new();
    let mut visited = HashSet::new();
    scan_dir(root, &mut files, &mut visited)?;
    Ok(DirectorySnapshot {
        root: root.to_string(),
        files,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64,
    })
}

fn scan_dir(
    dir: &str,
    files: &mut HashMap<String, crate::hashing::FileSnapshot>,
    visited: &mut HashSet<u64>,
) -> anyhow::Result<()> {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(()),
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let path_str = path.to_string_lossy().to_string();
        if path.is_symlink() {
            if symlink::is_symlink_loop(&path_str, visited) {
                continue;
            }
            symlink::register_inode(&path_str, visited);
        }
        if path.is_file() {
            match metadata::collect(&path_str) {
                Ok(snap) => {
                    files.insert(path_str, snap);
                }
                Err(_) => {}
            }
        } else if path.is_dir() {
            scan_dir(&path_str, files, visited)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_directory() {
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::write(dir.path().join("a.txt"), b"hello").unwrap();
        std::fs::write(dir.path().join("b.txt"), b"world").unwrap();
        let snap = scan(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(snap.files.len(), 2);
    }
}
