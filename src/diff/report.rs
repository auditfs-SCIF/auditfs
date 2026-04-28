// Partie 4 — Export du rapport de diff
// TODO: générer le rapport en texte, JSON et HTML

use super::compare::DiffResult;

pub fn to_text(_diff: &DiffResult) -> String {
    todo!("Implémenter l'export texte")
}

pub fn to_json(_diff: &DiffResult) -> anyhow::Result<String> {
    todo!("Implémenter l'export JSON avec serde_json")
}

pub fn to_html(_diff: &DiffResult) -> String {
    todo!("Implémenter l'export HTML")
}
