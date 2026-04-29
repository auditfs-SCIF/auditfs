use crate::scanner::walker;
use crate::database::serialize;
use crate::diff::{compare, report};
use crate::daemon::{alert, whitelist::Whitelist};
use std::thread;
use std::time::Duration;

pub fn run(
    target_dir: &str,
    baseline_path: &str,
    password: &str,
    log_path: &str,
    interval_secs: u64,
    whitelist: &Whitelist,
) -> anyhow::Result<()> {
    loop {
        match run_once(target_dir, baseline_path, password, log_path, whitelist) {
            Ok(_) => {}
            Err(e) => { let _ = alert::log_to_file(log_path, &format!("ERREUR scan: {}", e)); }
        }
        thread::sleep(Duration::from_secs(interval_secs));
    }
}

pub fn run_once(
    target_dir: &str,
    baseline_path: &str,
    password: &str,
    log_path: &str,
    whitelist: &Whitelist,
) -> anyhow::Result<()> {
    let baseline = serialize::load(baseline_path, password)?;
    let current = walker::scan(target_dir)?;
   // let diff = compare::compare(&baseline.baseline, &current);
let diff = crate::diff::compare::DiffResult {
    changes: vec![],
    total_new: 0,
    total_old: 0,
};
...
let filtered_diff = crate::diff::compare::DiffResult {
    changes: filtered,
    total_new: 0,
    total_old: 0,
};

    if !filtered.is_empty() {
        let filtered_diff = crate::diff::compare::DiffResult { changes: filtered };
        let report_text = report::to_text(&filtered_diff);
        alert::log_to_file(log_path, &report_text)?;
    }
    Ok(())
}
