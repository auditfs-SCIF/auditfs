mod daemon;
mod database;
mod diff;
mod hashing;
mod scanner;

use std::env;

fn usage() {
    println!("AuditFS — Système de contrôle d'intégrité de fichiers");
    println!();
    println!("USAGE:");
    println!("  auditfs scan   <dossier> <baseline.db> <motdepasse>");
    println!("  auditfs diff   <baseline.db> <motdepasse> [text|json|html]");
    println!("  auditfs daemon <dossier> <baseline.db> <motdepasse> <interval_sec> <log.txt>");
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        return Ok(());
    }

    match args[1].as_str() {
        "scan" => {
            if args.len() < 5 {
                println!("Usage: auditfs scan <dossier> <baseline.db> <motdepasse>");
                return Ok(());
            }
            let snap = scanner::walker::scan(&args[2])?;
            let db = hashing::IntegrityDB {
                baseline: snap,
                version: "1.0".to_string(),
            };
            database::serialize::save(&db, &args[3], &args[4])?;
            println!("Baseline sauvegardée dans {}", args[3]);
        }
        "diff" => {
            if args.len() < 4 {
                println!("Usage: auditfs diff <baseline.db> <motdepasse> [text|json|html]");
                return Ok(());
            }
            let db = database::serialize::load(&args[2], &args[3])?;
            let current = scanner::walker::scan(&db.baseline.root)?;
            let result = diff::compare::compare(&db.baseline, &current);
            let format = args.get(4).map(|s| s.as_str()).unwrap_or("text");
            match format {
                "json" => println!("{}", diff::report::to_json(&result)?),
                "html" => println!("{}", diff::report::to_html(&result)),
                _ => println!("{}", diff::report::to_text(&result)),
            }
        }
        "daemon" => {
            if args.len() < 7 {
                println!(
                    "Usage: auditfs daemon <dossier> <baseline.db> <motdepasse> <interval> <log>"
                );
                return Ok(());
            }
            let interval: u64 = args[5].parse().unwrap_or(3600);
            let wl = daemon::whitelist::Whitelist::new(vec![]);
            println!(
                "Daemon démarré (interval={}s) — Ctrl+C pour arrêter",
                interval
            );
            daemon::scheduler::run(&args[2], &args[3], &args[4], &args[6], interval, &wl)?;
        }
        _ => usage(),
    }
    Ok(())
}
