// Partie 4 — Planificateur (daemon)
use crate::daemon::whitelist::Whitelist;
use crate::database;
use crate::diff::compare;
use crate::scanner::walker;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;

pub fn run_once(
    scan_dir: &str,
    baseline_path: &str,
    password: &str,
    log_path: &str,
    whitelist: &Whitelist,
) -> Result<()> {
    // 1. Charger la baseline
    let db = database::load(baseline_path, password)?;

    // 2. Scanner l’état actuel
    let current = walker::scan(scan_dir)?;

    // 3. Comparer
    let diff = compare::compare(&db.baseline, &current);

    // 4. Filtrer les fichiers whitelistés
    let filtered_changes: Vec<_> = diff
        .changes
        .into_iter()
        .filter(|c| !whitelist.is_whitelisted(&c.path))
        .collect();

    if filtered_changes.is_empty() {
        return Ok(());
    }

    // 5. Écrire les changements dans le fichier log (append)
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

    for change in filtered_changes {
        let line = match change.change {
            compare::ChangeType::Added => format!("[{}] AJOUTÉ    : {}\n", timestamp, change.path),
            compare::ChangeType::Removed => {
                format!("[{}] SUPPRIMÉ  : {}\n", timestamp, change.path)
            }
            compare::ChangeType::Modified { changed_attributes } => format!(
                "[{}] MODIFIÉ   : {} ({:?})\n",
                timestamp, change.path, changed_attributes
            ),
            compare::ChangeType::DangerousPermission => format!(
                "[{}] DANGER     : {} — permissions world-writable\n",
                timestamp, change.path
            ),
        };
        file.write_all(line.as_bytes())?;
    }

    Ok(())
}
