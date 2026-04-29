// Partie 4 — Planificateur (désactivé temporairement)
use anyhow::Result;
use crate::daemon::whitelist::Whitelist;

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