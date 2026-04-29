// Ces tests vérifient le pipeline complet :
//   snapshot A  →  compare_snapshots  →  DiffResult  →  to_text / to_json / to_html

use std::collections::HashMap;

use auditfs::diff::{compare_snapshots, to_html, to_json, to_text};

// Helper

/// Construit un snapshot à partir de paires (chemin, hash)
fn make_snapshot(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

// Tests d'intégration

/// Scénario complet : système sain → aucun rapport d'alerte
#[test]
fn integration_systeme_intact() {
    let snapshot = make_snapshot(&[
        ("/etc/passwd", "hash_passwd"),
        ("/etc/hosts", "hash_hosts"),
        ("/usr/bin/ls", "hash_ls"),
    ]);

    let result = compare_snapshots(&snapshot, &snapshot);

    assert!(result.is_clean(), "Aucune alerte pour un système intact");
    assert_eq!(result.total_old, 3);
    assert_eq!(result.total_new, 3);

    // to_text doit signaler qu'aucun changement n'a été détecté
    let text = to_text(&result);
    assert!(text.contains("Aucun changement"));

    // to_json doit indiquer un tableau vide
    let json = to_json(&result).unwrap();
    assert!(json.contains("\"changes\": []"));

    // to_html doit afficher le message "aucun changement"
    let html = to_html(&result);
    assert!(html.contains("Aucun changement"));
}

/// Scénario d'intrusion : fichier système modifié
#[test]
fn integration_fichier_modifie() {
    let old = make_snapshot(&[("/etc/passwd", "hash_original")]);
    let new = make_snapshot(&[("/etc/passwd", "hash_compromis")]);

    let result = compare_snapshots(&old, &new);

    assert_eq!(result.count(), 1);
    assert_eq!(result.modified().len(), 1);

    // to_text doit mentionner [MODIFIÉ]
    let text = to_text(&result);
    assert!(text.contains("[MODIFIÉ]"));
    assert!(text.contains("/etc/passwd"));

    // to_json doit contenir "Modified"
    let json = to_json(&result).unwrap();
    assert!(json.contains("Modified"));
    assert!(json.contains("/etc/passwd"));

    // to_html doit afficher le badge MODIFIÉ
    let html = to_html(&result);
    assert!(html.contains("MODIFIÉ"));
}

/// Scénario : backdoor installée (nouveau fichier suspect)
#[test]
fn integration_fichier_ajoute() {
    let old = make_snapshot(&[("/etc/passwd", "h1")]);
    let new = make_snapshot(&[
        ("/etc/passwd", "h1"),
        ("/tmp/.backdoor", "evil_hash"),
    ]);

    let result = compare_snapshots(&old, &new);

    assert_eq!(result.added().len(), 1);
    assert_eq!(result.added()[0].path, "/tmp/.backdoor");

    // to_text
    let text = to_text(&result);
    assert!(text.contains("[AJOUTÉ]"));
    assert!(text.contains(".backdoor"));

    // to_html
    let html = to_html(&result);
    assert!(html.contains("AJOUTÉ"));
    assert!(html.contains(".backdoor"));
}

/// Scénario : fichier de config supprimé (sabotage)
#[test]
fn integration_fichier_supprime() {
    let old = make_snapshot(&[
        ("/etc/nginx/nginx.conf", "config_hash"),
        ("/etc/nginx/sites-enabled/default", "default_hash"),
    ]);
    let new = make_snapshot(&[
        ("/etc/nginx/nginx.conf", "config_hash"),
        // sites-enabled/default a disparu
    ]);

    let result = compare_snapshots(&old, &new);

    assert_eq!(result.removed().len(), 1);
    assert_eq!(result.removed()[0].path, "/etc/nginx/sites-enabled/default");

    // to_text
    let text = to_text(&result);
    assert!(text.contains("[SUPPRIMÉ]"));

    // to_html
    let html = to_html(&result);
    assert!(html.contains("SUPPRIMÉ"));
}

/// Scénario complexe : plusieurs types de changements simultanés
#[test]
fn integration_changements_multiples() {
    let old = make_snapshot(&[
        ("/bin/bash", "bash_hash"),    // inchangé
        ("/etc/crontab", "cron_orig"), // sera modifié
        ("/etc/motd", "motd_hash"),    // sera supprimé
    ]);
    let new = make_snapshot(&[
        ("/bin/bash", "bash_hash"),        // inchangé
        ("/etc/crontab", "cron_modifie"),  // modifié
        ("/tmp/exploit.sh", "xploit"),     // ajouté
    ]);

    let result = compare_snapshots(&old, &new);

    assert_eq!(result.added().len(), 1);
    assert_eq!(result.removed().len(), 1);
    assert_eq!(result.modified().len(), 1);
    assert_eq!(result.count(), 3);

    // to_text : les 3 types présents
    let text = to_text(&result);
    assert!(text.contains("[MODIFIÉ]"));
    assert!(text.contains("[SUPPRIMÉ]"));
    assert!(text.contains("[AJOUTÉ]"));

    // to_json : les 3 types présents
    let json = to_json(&result).unwrap();
    assert!(json.contains("Modified"));
    assert!(json.contains("Added"));
    assert!(json.contains("Removed"));

    // to_html : les 3 sections présentes
    let html = to_html(&result);
    assert!(html.contains("Fichiers modifiés"));
    assert!(html.contains("Fichiers ajoutés"));
    assert!(html.contains("Fichiers supprimés"));
}

/// Vérifie que le JSON exporté est valide et re-parsable
#[test]
fn integration_json_roundtrip() {
    let old = make_snapshot(&[("/a", "1"), ("/b", "2")]);
    let new = make_snapshot(&[("/a", "1"), ("/c", "3")]);

    let result = compare_snapshots(&old, &new);
    let json = to_json(&result).expect("Sérialisation JSON");

    // Re-parser le JSON et vérifier la structure
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("JSON valide");
    assert!(parsed["changes"].is_array());
    assert_eq!(parsed["total_old"], 2);
    assert_eq!(parsed["total_new"], 2);
}

/// Vérifie que to_text contient bien les stats globales
#[test]
fn integration_text_stats() {
    let old = make_snapshot(&[("/a", "1"), ("/b", "2")]);
    let new = make_snapshot(&[("/a", "1"), ("/c", "3")]);

    let result = compare_snapshots(&old, &new);
    let text = to_text(&result);

    // Le rapport texte doit mentionner les compteurs
    assert!(text.contains("Fichiers avant"));
    assert!(text.contains("Changements"));
}
