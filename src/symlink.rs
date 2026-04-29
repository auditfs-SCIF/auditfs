use std::collections::HashSet;
use std::fs;
use std::os::unix::fs::MetadataExt;

pub fn is_symlink_loop(path: &str, visited: &HashSet<u64>) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        visited.contains(&metadata.ino())
    } else {
        false
    }
}

pub fn register_inode(path: &str, visited: &mut HashSet<u64>) {
    if let Ok(metadata) = fs::metadata(path) {
        visited.insert(metadata.ino());
    }
}
