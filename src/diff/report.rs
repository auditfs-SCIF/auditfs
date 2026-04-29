// Implémente les 3 formats d'export d'un DiffResult :
//   - to_text  : rapport lisible dans un terminal
//   - to_json  : sérialisation JSON (pour scripts / SIEM)
//   - to_html  : rapport visuel coloré pour navigateur


use super::compare::{ChangeKind, DiffResult};
use anyhow::Context; // pour les messages d'erreur explicites

// Export TEXTE

pub fn to_text(diff: &DiffResult) -> String {
    let mut out = String::new();

    // En-tête 
    out.push_str("=== AuditFS — Rapport de diff ===\n");
    out.push_str(&format!(
        "Fichiers avant : {}  |  après : {}\n",
        diff.total_old, diff.total_new
    ));
    out.push_str(&format!(
        "Changements : {}  (ajoutés: {} | supprimés: {} | modifiés: {})\n",
        diff.count(),
        diff.added().len(),
        diff.removed().len(),
        diff.modified().len(),
    ));

    // Cas aucun changement
    if diff.is_clean() {
        out.push_str("\n✅ Aucun changement détecté — le système est intègre.\n");
        return out;
    }

    out.push('\n');

    // Liste des changements (triés, déjà par compare.rs)
    for change in &diff.changes {
        match change.kind {
            ChangeKind::Modified => {
                out.push_str(&format!("[MODIFIÉ]   {}\n", change.path));
                out.push_str(&format!(
                    "  ancien : {}\n  nouveau: {}\n\n",
                    change.old_hash.as_deref().unwrap_or("—"),
                    change.new_hash.as_deref().unwrap_or("—"),
                ));
            }
            ChangeKind::Removed => {
                out.push_str(&format!("[SUPPRIMÉ]  {}\n", change.path));
                out.push_str(&format!(
                    "  ancien : {}\n\n",
                    change.old_hash.as_deref().unwrap_or("—"),
                ));
            }
            ChangeKind::Added => {
                out.push_str(&format!("[AJOUTÉ]    {}\n", change.path));
                out.push_str(&format!(
                    "  nouveau: {}\n\n",
                    change.new_hash.as_deref().unwrap_or("—"),
                ));
            }
        }
    }

    out
}

// Export JSON

pub fn to_json(diff: &DiffResult) -> anyhow::Result<String> {
    // serde_json::to_string_pretty utilise le trait Serialize dérivé sur DiffResult
    serde_json::to_string_pretty(diff)
        .context("Échec de la sérialisation JSON du DiffResult")
}

// Export HTML

pub fn to_html(diff: &DiffResult) -> String {
    let mut html = String::new();

    let n_added    = diff.added().len();
    let n_removed  = diff.removed().len();
    let n_modified = diff.modified().len();

    // En-tête HTML + CSS inline
    html.push_str(r#"<!DOCTYPE html>
<html lang="fr">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>AuditFS — Rapport de diff</title>
  <style>
    body { font-family: 'Segoe UI', Arial, sans-serif; background:#f5f7fa; color:#333; margin:0; padding:20px; }
    h1   { color:#2c3e50; border-bottom:3px solid #2c3e50; padding-bottom:10px; }
    h2   { color:#555; margin-top:30px; }
    .stats { display:flex; gap:20px; margin:20px 0; flex-wrap:wrap; }
    .stat-card { background:white; border-radius:8px; padding:15px 25px; box-shadow:0 2px 6px rgba(0,0,0,.1); text-align:center; min-width:120px; }
    .stat-card .number { font-size:2em; font-weight:bold; }
    .stat-card .label  { color:#777; font-size:.85em; }
    .added    { color:#27ae60; }
    .removed  { color:#e74c3c; }
    .modified { color:#e67e22; }
    table { width:100%; border-collapse:collapse; background:white; box-shadow:0 2px 6px rgba(0,0,0,.1); border-radius:8px; overflow:hidden; margin-bottom:30px; }
    th { background:#2c3e50; color:white; padding:12px 16px; text-align:left; }
    td { padding:10px 16px; border-bottom:1px solid #eee; font-size:.9em; word-break:break-all; }
    tr:last-child td { border-bottom:none; }
    tr:hover td { background:#f0f4f8; }
    .badge { display:inline-block; padding:3px 10px; border-radius:12px; font-size:.8em; font-weight:bold; color:white; }
    .badge-added    { background:#27ae60; }
    .badge-removed  { background:#e74c3c; }
    .badge-modified { background:#e67e22; }
    .clean { background:#d5f5e3; border:1px solid #27ae60; border-radius:8px; padding:20px; color:#1e8449; font-size:1.1em; text-align:center; margin:30px 0; }
    .hash  { font-family:'Courier New',monospace; font-size:.85em; color:#666; }
    footer { margin-top:40px; color:#aaa; font-size:.8em; text-align:center; }
  </style>
</head>
<body>
  <h1>🛡️ AuditFS — Rapport de comparaison</h1>
"#);

    // Statistiques
    html.push_str(&format!(
        r#"  <div class="stats">
    <div class="stat-card"><div class="number">{old}</div><div class="label">Fichiers (avant)</div></div>
    <div class="stat-card"><div class="number">{new}</div><div class="label">Fichiers (après)</div></div>
    <div class="stat-card"><div class="number added">{na}</div><div class="label">Ajoutés</div></div>
    <div class="stat-card"><div class="number removed">{nr}</div><div class="label">Supprimés</div></div>
    <div class="stat-card"><div class="number modified">{nm}</div><div class="label">Modifiés</div></div>
  </div>
"#,
        old = diff.total_old, new = diff.total_new,
        na = n_added, nr = n_removed, nm = n_modified,
    ));

    // Cas aucun changement
    if diff.is_clean() {
        html.push_str(r#"  <div class="clean">✅ Aucun changement détecté — le système est intègre.</div>
</body></html>"#);
        return html;
    }

    // Section Modifiés
    if n_modified > 0 {
        html.push_str("  <h2>⚠️ Fichiers modifiés</h2>\n");
        html.push_str("  <table><thead><tr><th>Statut</th><th>Chemin</th><th>Ancien hash</th><th>Nouveau hash</th></tr></thead><tbody>\n");
        for c in diff.modified() {
            html.push_str(&format!(
                "    <tr><td><span class=\"badge badge-modified\">MODIFIÉ</span></td><td>{}</td><td class=\"hash\">{}</td><td class=\"hash\">{}</td></tr>\n",
                escape_html(&c.path),
                truncate_hash(&c.old_hash),
                truncate_hash(&c.new_hash),
            ));
        }
        html.push_str("  </tbody></table>\n");
    }

    // Section Supprimés
    if n_removed > 0 {
        html.push_str("  <h2>🗑️ Fichiers supprimés</h2>\n");
        html.push_str("  <table><thead><tr><th>Statut</th><th>Chemin</th><th>Hash (référence)</th></tr></thead><tbody>\n");
        for c in diff.removed() {
            html.push_str(&format!(
                "    <tr><td><span class=\"badge badge-removed\">SUPPRIMÉ</span></td><td>{}</td><td class=\"hash\">{}</td></tr>\n",
                escape_html(&c.path),
                truncate_hash(&c.old_hash),
            ));
        }
        html.push_str("  </tbody></table>\n");
    }

    // Section Ajoutés
    if n_added > 0 {
        html.push_str("  <h2>➕ Fichiers ajoutés</h2>\n");
        html.push_str("  <table><thead><tr><th>Statut</th><th>Chemin</th><th>Hash (nouveau)</th></tr></thead><tbody>\n");
        for c in diff.added() {
            html.push_str(&format!(
                "    <tr><td><span class=\"badge badge-added\">AJOUTÉ</span></td><td>{}</td><td class=\"hash\">{}</td></tr>\n",
                escape_html(&c.path),
                truncate_hash(&c.new_hash),
            ));
        }
        html.push_str("  </tbody></table>\n");
    }

    html.push_str("  <footer>AuditFS — Système de contrôle d'intégrité de fichiers</footer>\n</body>\n</html>\n");
    html
}

// Fonctions utilitaires internes

/// Tronque un hash à 16 caractères pour l'affichage (évite tableaux trop larges)
fn truncate_hash(hash: &Option<String>) -> String {
    match hash {
        Some(h) if h.len() > 16 => format!("{}…", &h[..16]),
        Some(h) => h.clone(),
        None => "—".to_string(),
    }
}

/// Échappe les caractères spéciaux HTML pour éviter les injections XSS
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}

// Tests unitaires

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff::compare::{ChangeKind, FileChange};

    fn make_result(changes: Vec<FileChange>, total_old: usize, total_new: usize) -> DiffResult {
        DiffResult { changes, total_old, total_new }
    }

    #[test]
    fn test_to_text_propre() {
        let r = make_result(vec![], 3, 3);
        let txt = to_text(&r);
        assert!(txt.contains("Aucun changement"));
    }

    #[test]
    fn test_to_text_avec_changements() {
        let changes = vec![FileChange {
            path: "/etc/passwd".to_string(),
            kind: ChangeKind::Modified,
            old_hash: Some("aaa".to_string()),
            new_hash: Some("bbb".to_string()),
        }];
        let txt = to_text(&make_result(changes, 1, 1));
        assert!(txt.contains("[MODIFIÉ]"));
        assert!(txt.contains("/etc/passwd"));
        assert!(txt.contains("aaa"));
    }

    #[test]
    fn test_to_json_valide() {
        let r = make_result(vec![], 2, 2);
        let json = to_json(&r).expect("JSON valide attendu");
        assert!(json.contains("\"changes\": []"));
    }

    #[test]
    fn test_to_html_propre() {
        let r = make_result(vec![], 0, 0);
        let html = to_html(&r);
        assert!(html.contains("Aucun changement"));
        assert!(!html.contains("<table>"));
    }

    #[test]
    fn test_to_html_modifie() {
        let changes = vec![FileChange {
            path: "/etc/shadow".to_string(),
            kind: ChangeKind::Modified,
            old_hash: Some("old".to_string()),
            new_hash: Some("new".to_string()),
        }];
        let html = to_html(&make_result(changes, 1, 1));
        assert!(html.contains("MODIFIÉ"));
        assert!(html.contains("/etc/shadow"));
    }

    #[test]
    fn test_escape_html_xss() {
        let changes = vec![FileChange {
            path: "/tmp/<script>".to_string(),
            kind: ChangeKind::Added,
            old_hash: None,
            new_hash: Some("x".to_string()),
        }];
        let html = to_html(&make_result(changes, 0, 1));
        assert!(html.contains("&lt;script&gt;"));
        assert!(!html.contains("<script>"));
    }
}
