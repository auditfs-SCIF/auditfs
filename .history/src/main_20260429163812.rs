mod daemon;
mod database;
mod diff;
mod hashing;
mod scanner;

use database::{load, save};
use std::env;

fn usage() {
    println!("AuditFS — Système de contrôle d'intégrité de fichiers");
    println!();
    println!("USAGE:");
    println!("  auditfs scan   <dossier> <baseline.db> <motdepasse>");
    println!("  auditfs diff   <baseline.db> <motdepasse> [text|json|html]");
    println!("  auditfs daemon <dossier> <baseline.db> <motdepasse> <log.txt>");
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
    let path_to_scan = &args[2];
    // Vérifier que le dossier existe
    if !std::path::Path::new(path_to_scan).exists() {
        eprintln!("ERREUR : le dossier '{}' n'existe pas", path_to_scan);
        return Ok(());
    }
    println!("Scan du dossier : {}", path_to_scan);
    let snap = match scanner::walker::scan(path_to_scan) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Erreur lors du scan : {}", e);
            return Ok(());
        }
    };
    let db = hashing::IntegrityDB {
        baseline: snap,
        version: "1.0".to_string(),
    };
    println!("Appel de save...");
    if let Err(e) = save(&db, &args[3], &args[4]) {
        eprintln!("Erreur lors de la sauvegarde : {}", e);
        return Ok(());
    }
    println!("Baseline sauvegardée dans {}", args[3]);
}
        "diff" => {
            if args.len() < 4 {
                println!("Usage: auditfs diff <baseline.db> <motdepasse> [text|json|html]");
                return Ok(());
            }
            let db = load(&args[2], &args[3])?;
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
            if args.len() < 6 {
                println!("Usage: auditfs daemon <dossier> <baseline.db> <motdepasse> <log.txt>");
                return Ok(());
            }
            let wl = daemon::whitelist::Whitelist::new(vec![]);
            println!("Daemon démarré (Ctrl+C pour arrêter)");
            loop {
                daemon::scheduler::run_once(&args[2], &args[3], &args[4], &args[5], &wl)?;
                println!("Scan effectué, prochain dans 60 secondes...");
                std::thread::sleep(std::time::Duration::from_secs(60));
            }
        }
        _ => usage(),
    }
    Ok(())
}
