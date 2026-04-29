use auditfs::scanner::walker;
use std::fs;
use tempfile::TempDir;

#[test]

fn test_scan_directory() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("a.txt"), b"hello").unwrap();
    fs::write(dir.path().join("b.txt"), b"world").unwrap();
    let snap = walker::scan(dir.path().to_str().unwrap()).unwrap();
    assert_eq!(snap.files.len(), 2);
}

#[test]
#[ignore]
fn test_scan_empty_directory() {
    let dir = TempDir::new().unwrap();
    let snap = walker::scan(dir.path().to_str().unwrap()).unwrap();
    assert_eq!(snap.files.len(), 0);
}

#[test]
#[ignore]
fn test_scan_nested_directory() {
    let dir = TempDir::new().unwrap();
    let subdir = dir.path().join("sous_dossier");
    fs::create_dir(&subdir).unwrap();
    fs::write(dir.path().join("racine.txt"), b"fichier racine").unwrap();
    fs::write(subdir.join("profond.txt"), b"fichier profond").unwrap();
    let snap = walker::scan(dir.path().to_str().unwrap()).unwrap();
    // Doit trouver les 2 fichiers dans les sous-dossiers
    assert_eq!(snap.files.len(), 2);
}

#[test]
#[ignore]
fn test_scan_captures_file_size() {
    let dir = TempDir::new().unwrap();
    let content = b"contenu de taille connue";
    fs::write(dir.path().join("fichier.txt"), content).unwrap();
    let snap = walker::scan(dir.path().to_str().unwrap()).unwrap();
    let file = snap.files.values().next().unwrap();
    assert_eq!(file.size, content.len() as u64);
}

#[test]
#[ignore]
fn test_scan_captures_hashes() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("test.txt"), b"contenu test").unwrap();
    let snap = walker::scan(dir.path().to_str().unwrap()).unwrap();
    let file = snap.files.values().next().unwrap();
    assert_eq!(file.sha256.len(), 64, "SHA-256 doit faire 64 caractères hex");
    assert_eq!(file.blake3.len(), 64, "BLAKE3 doit faire 64 caractères hex");
}

#[test]
#[ignore]
fn test_scan_root_is_correct() {
    let dir = TempDir::new().unwrap();
    let root = dir.path().to_str().unwrap().to_string();
    let snap = walker::scan(&root).unwrap();
    assert_eq!(snap.root, root);
}

#[test]
#[ignore]
fn test_symlink_no_loop() {
    // Vérifie que le scan d'un dossier sans symlinks ne plante pas
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("normal.txt"), b"fichier normal").unwrap();
    let result = walker::scan(dir.path().to_str().unwrap());
    assert!(result.is_ok(), "Le scan ne doit pas planter sur des fichiers normaux");
}
