// Partie 4 — Planificateur (désactivé temporairement)
use crate::daemon::whitelist::Whitelist;
use anyhow::Result;

pub fn run_once(
    _scan_dir: &str,
    _baseline_path: &str,
    _password: &str,
    _log_path: &str,
    _whitelist: &Whitelist,
) -> Result<()> {
    // Stub – à implémenter quand la partie diff sera prête
    Ok(())
}
