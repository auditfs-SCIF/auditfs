#[cfg(test)]
mod tests {
    use auditfs::scanner::walker;
    use auditfs::database::serialize;
    use auditfs::diff::compare;
    use auditfs::hashing::IntegrityDB;
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
        fs::write(dir.path().join("fichier.txt"), b"contenu modifie").unwrap();

        // 4. Scanner à nouveau et comparer
        let db2 = serialize::load(baseline_path.to_str().unwrap(), "secret").unwrap();
        let current = walker::scan(dir.path().to_str().unwrap()).unwrap();
        let diff = compare::compare(&db2.baseline, &current);

        // 5. Vérifier que la modification est détectée
        assert!(!diff.changes.is_empty(), "La modification doit être détectée");
    }

    #[test]
    fn test_daemon_detects_change() {
        let dir = TempDir::new().unwrap();
        let baseline_path = dir.path().join("baseline.db");
        let log_path = dir.path().join("audit.log");

        // 1. Créer baseline
        fs::write(dir.path().join("config.txt"), b"original").unwrap();
        let snap = walker::scan(dir.path().to_str().unwrap()).unwrap();
        let db = IntegrityDB { baseline: snap, version: "1.0".to_string() };
        serialize::save(&db, baseline_path.to_str().unwrap(), "secret").unwrap();

        // 2. Simuler une modification
        fs::write(dir.path().join("config.txt"), b"modifie par attaquant").unwrap();

        // 3. Lancer un scan unique du daemon
        let wl = auditfs::daemon::whitelist::Whitelist::new(vec![]);
        auditfs::daemon::scheduler::run_once(
            dir.path().to_str().unwrap(),
            baseline_path.to_str().unwrap(),
            "secret",
            log_path.to_str().unwrap(),
            &wl,
        ).unwrap();

        // 4. Vérifier que le log contient une alerte
        let log = fs::read_to_string(&log_path).unwrap_or_default();
        assert!(!log.is_empty(), "Le log doit contenir une alerte");
    }
}