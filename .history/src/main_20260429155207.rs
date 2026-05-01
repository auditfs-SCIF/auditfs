use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "auditfs")]
#[command(about = "Système de contrôle d'intégrité de fichiers")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scanner un répertoire et créer une baseline
    Scan {
        /// Répertoire à scanner
        path: String,
        /// Fichier de baseline à créer
        #[arg(long, default_value = "baseline.db")]
        baseline: String,
    },
    /// Comparer avec la baseline existante
    Diff {
        /// Fichier de baseline
        baseline: String,
        /// Format de sortie (text, json, html)
        #[arg(long, default_value = "text")]
        output: String,
    },
    /// Lancer le daemon de surveillance
    Daemon {
        /// Intervalle entre les scans (secondes)
        #[arg(long, default_value = "3600")]
        interval: u64,
        /// Fichier de log
        #[arg(long, default_value = "audit.log")]
        log: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan { path, baseline } => {
            println!("Scan de {} → baseline: {}", path, baseline);
            // TODO: appel Partie 3 (scanner) + Partie 2 (database)
        }
        Commands::Diff { baseline, output } => {
            println!("Diff baseline: {} format: {}", baseline, output);
            // TODO: appel Partie 4 (diff)
        }
        Commands::Daemon { interval, log } => {
            println!("Daemon démarré (interval={}s, log={})", interval, log);
            // TODO: appel Partie 5 (daemon)
        }
    }
}
