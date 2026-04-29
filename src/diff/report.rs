use super::compare::{ChangeType, DiffResult};

pub fn to_text(diff: &DiffResult) -> String {
    if diff.changes.is_empty() {
        return "Aucun changement détecté.\n".to_string();
    }
    let mut out = String::from("=== Rapport AuditFS ===\n\n");
    for change in &diff.changes {
        let line = match &change.change {
            ChangeType::Added => format!("[AJOUTÉ]    {}\n", change.path),
            ChangeType::Removed => format!("[SUPPRIMÉ]  {}\n", change.path),
            ChangeType::Modified { changed_attributes } => format!(
                "[MODIFIÉ]   {} ({})\n",
                change.path,
                changed_attributes.join(", ")
            ),
            ChangeType::DangerousPermission => {
                format!("[DANGER]    {} — permissions world-writable\n", change.path)
            }
        };
        out.push_str(&line);
    }
    out
}

pub fn to_json(diff: &DiffResult) -> anyhow::Result<String> {
    Ok(serde_json::to_string_pretty(diff)?)
}

pub fn to_html(diff: &DiffResult) -> String {
    let mut rows = String::new();
    for change in &diff.changes {
        let (label, color) = match &change.change {
            ChangeType::Added => ("Ajouté", "#d4edda"),
            ChangeType::Removed => ("Supprimé", "#f8d7da"),
            ChangeType::Modified { .. } => ("Modifié", "#fff3cd"),
            ChangeType::DangerousPermission => ("Dangereux", "#f5c6cb"),
        };
        rows.push_str(&format!(
            "<tr style='background:{}'><td>{}</td><td>{}</td></tr>\n",
            color, change.path, label
        ));
    }
    format!(
        r#"<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>Rapport AuditFS</title>
<style>body{{font-family:sans-serif;padding:20px}}table{{border-collapse:collapse;width:100%}}
td{{border:1px solid #ddd;padding:8px}}</style></head>
<body><h1>Rapport AuditFS</h1>
<table><tr><th>Fichier</th><th>Changement</th></tr>{}</table>
</body></html>"#,
        rows
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff::compare::{DiffResult, FileChange};

    #[test]
    fn test_to_text_empty() {
        let diff = DiffResult { changes: vec![] };
        assert!(to_text(&diff).contains("Aucun changement"));
    }

    #[test]
    fn test_to_json_valid() {
        let diff = DiffResult {
            changes: vec![FileChange {
                path: "/tmp/f".to_string(),
                change: ChangeType::Added,
            }],
        };
        let json = to_json(&diff).unwrap();
        assert!(json.contains("Added"));
    }

    #[test]
    fn test_to_html_contains_table() {
        let diff = DiffResult { changes: vec![] };
        let html = to_html(&diff);
        assert!(html.contains("<table>"));
    }
}
