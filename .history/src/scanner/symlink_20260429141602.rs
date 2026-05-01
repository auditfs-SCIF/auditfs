
use std::collections::HashSet;
use std::path::Path;

#[cfg(unix)]
fn get_inode(path: &Path) -> Option<u64> {
    use std::os::unix::fs::MetadataExt;
    fs::metadata(path).ok().map(|m| m.ino())
}

#[cfg(not(unix))]
fn get_inode(_path: &Path) -> Option<u64> {
    None
}

pub fn is_symlink_loop(path: &str, visited_inodes: &HashSet<u64>) -> bool {
    let p = Path::new(path);
    if !p.is_symlink() {
        return false;
    }
    if let Some(inode) = get_inode(p) {
        return visited_inodes.contains(&inode);
    }
    false
}

pub fn register_inode(path: &str, visited: &mut HashSet<u64>) {
    let p = Path::new(path);
    if let Some(inode) = get_inode(p) {
        visited.insert(inode);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_loop_normal_file() {
        let visited = HashSet::new();
        // Un fichier normal ne crée pas de boucle
        assert!(!is_symlink_loop("/tmp", &visited));
    }
}
