?use auditfs::scanner::walker;
use auditfs::database::serialize;
use auditfs::diff::compare;
use auditfs::hashing::IntegrityDB;
use auditfs::daemon::whitelist::Whitelist;
use auditfs::daemon::scheduler::run_once;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_full_scan_diff_cycle() {
    let dir = TempDir::new().unwrap();
    let baseline_path = dir.path().join("baseline.db");

    // 1. Créer un fichier initial
    fs::write(dir.path().join("fichier.txt"), b"contenu initial").unwrap();

    // 2. Scanner et sauvegarder la baseline
    let snap = walker::scan(dir.path().to_str().unwrap()).unwrap();
    let db = IntegrityDB { baseline: snap, version: "1.0".to_string() };
    serialize::save(&db, baseline_path.to_str().unwrap(), "secret").unwrap();

    // 3. Modifier le fichier
    fs::write(dir.path().join("fichier.txt"), b"contenu modifie par attaquant").unwrap();

    // 4. Scanner à nouveau et comparer
    let db2 = serialize::load(baseline_path.to_str().unwrap(), "secret").unwrap();
    let current = walker::scan(dir.path().to_str().unwrap()).unwrap();
    let diff = compare::compare(&db2.baseline, &current);

    // 5. Vérifier que la modification est détectée
    assert!(!diff.changes.is_empty(), "La modification doit être détectée");
}

#[test]
fn test_full_scan_no_changes() {
    let dir = TempDir::new().unwrap();
    // Mettre la baseline HORS du dossier scanné pour éviter les faux positifs
    let baseline_dir = TempDir::new().unwrap();
    let baseline_path = baseline_dir.path().join("baseline.db");

    // 1. Créer un fichier stable
    fs::write(dir.path().join("stable.txt"), b"contenu stable").unwrap();

    // 2. Scanner et sauvegarder la baseline (baseline.db hors du dossier scanné)
    let snap = walker::scan(dir.path().to_str().unwrap()).unwrap();
    let db = IntegrityDB { baseline: snap, version: "1.0".to_string() };
    serialize::save(&db, baseline_path.to_str().unwrap(), "secret").unwrap();

    // 3. Comparer sans rien modifier
    let db2 = serialize::load(baseline_path.to_str().unwrap(), "secret").unwrap();
    let current = walker::scan(dir.path().to_str().unwrap()).unwrap();
    let diff = compare::compare(&db2.baseline, &current);

    // 4. Aucun vrai changement ne doit être détecté
    let real_changes: Vec<_> = diff.changes.iter()
        .filter(|c| !matches!(c.change, auditfs::diff::compare::ChangeType::DangerousPermission))
        .collect();
    assert!(real_changes.is_empty(), "Aucun changement si rien n'a été modifié");
}

#[test]
fn test_detect_new_file_added() {
    let dir = TempDir::new().unwrap();
    let baseline_path = dir.path().join("baseline.db");

    // 1. Baseline avec un fichier
    fs::write(dir.path().join("existant.txt"), b"fichier existant").unwrap();
    let snap = walker::scan(dir.path().to_str().unwrap()).unwrap();
    let db = IntegrityDB { baseline: snap, version: "1.0".to_string() };
    serialize::save(&db, baseline_path.to_str().unwrap(), "secret").unwrap();

    // 2. Ajouter un nouveau fichier
    fs::write(dir.path().join("nouveau.txt"), b"intrus").unwrap();

    // 3. Comparer
    let db2 = serialize::load(baseline_path.to_str().unwrap(), "secret").unwrap();
    let current = walker::scan(dir.path().to_str().unwrap()).unwrap();
    let diff = compare::compare(&db2.baseline, &current);

    assert!(
        diff.changes.iter().any(|c| matches!(c.change, auditfs::diff::compare::ChangeType::Added)),
        "Un fichier ajouté doit être détecté"
    );
}

#[test]
fn test_daemon_detects_change() {
    let dir = TempDir::new().unwrap();
    let baseline_path = dir.path().join("baseline.db");
    let log_path = dir.path().join("audit.log");

    // 1. Créer baseline
    fs::write(dir.path().join("config.txt"), b"config originale").unwrap();
    let snap = walker::scan(dir.path().to_str().unwrap()).unwrap();
    let db = IntegrityDB { baseline: snap, version: "1.0".to_string() };
    serialize::save(&db, baseline_path.to_str().unwrap(), "secret").unwrap();

    // 2. Simuler une modification par un attaquant
    fs::write(dir.path().join("config.txt"), b"modifie par attaquant").unwrap();

    // 3. Lancer un scan unique du daemon
    let wl = Whitelist::new(vec![]);
    run_once(
        dir.path().to_str().unwrap(),
        baseline_path.to_str().unwrap(),
        "secret",
        log_path.to_str().unwrap(),
        &wl,
    ).unwrap();

    // 4. Vérifier que le log contient une alerte
    let log = fs::read_to_string(&log_path).unwrap_or_default();
    assert!(!log.is_empty(), "Le fichier de log doit contenir une alerte");
    assert!(log.contains("MODIFIÉ") || log.contains("config.txt"),
        "Le log doit mentionner le fichier modifié");
}

#[test]
fn test_daemon_whitelist_filters_changes() {
    let dir = TempDir::new().unwrap();
    let baseline_dir = TempDir::new().unwrap();
    let baseline_path = baseline_dir.path().join("baseline.db");
    let log_path = baseline_dir.path().join("audit.log");

    // 1. Baseline avec un fichier cache
    fs::write(dir.path().join("cache.tmp"), b"cache initial").unwrap();
    let snap = walker::scan(dir.path().to_str().unwrap()).unwrap();
    let db = IntegrityDB { baseline: snap, version: "1.0".to_string() };
    serialize::save(&db, baseline_path.to_str().unwrap(), "secret").unwrap();

    // 2. Modifier le fichier de cache
    fs::write(dir.path().join("cache.tmp"), b"cache mis a jour").unwrap();

    // 3. Daemon avec whitelist qui filtre "cache.tmp"
    let wl = Whitelist::new(vec!["cache.tmp".to_string()]);
    run_once(
        dir.path().to_str().unwrap(),
        baseline_path.to_str().unwrap(),
        "secret",
        log_path.to_str().unwrap(),
        &wl,
    ).unwrap();

    // 4. Le log doit être vide car le fichier est dans la whitelist
    let log = fs::read_to_string(&log_path).unwrap_or_default();
    assert!(log.is_empty(), "Le log doit être vide si le fichier est dans la whitelist");
}*/
