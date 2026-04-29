// Comparer deux snapshots d'intégrité et
// produire un DiffResult décrivant ce qui a changé.


use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Types publics

/// Catégorie d'un changement détecté sur un fichier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeKind {
    /// Le fichier n'existait pas dans l'ancien snapshot
    Added,
    /// Le fichier n'existe plus dans le nouveau snapshot
    Removed,
    /// Le fichier existe dans les deux snapshots mais son hash a changé
    Modified,
}

/// Décrit un changement sur un fichier précis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    /// Chemin absolu du fichier concerné
    pub path: String,

    /// Type de changement (Added / Removed / Modified)
    pub kind: ChangeKind,

    /// Hash du fichier dans l'ancien snapshot (None si le fichier était absent)
    pub old_hash: Option<String>,

    /// Hash du fichier dans le nouveau snapshot (None si le fichier a disparu)
    pub new_hash: Option<String>,
}

/// Résultat complet de la comparaison entre deux snapshots.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    /// Liste de tous les changements détectés (triée par chemin)
    pub changes: Vec<FileChange>,

    /// Nombre total de fichiers dans l'ancien snapshot
    pub total_old: usize,

    /// Nombre total de fichiers dans le nouveau snapshot
    pub total_new: usize,
}

impl DiffResult {
    /// Retourne uniquement les fichiers ajoutés
    pub fn added(&self) -> Vec<&FileChange> {
        self.changes
            .iter()
            .filter(|c| c.kind == ChangeKind::Added)
            .collect()
    }

    /// Retourne uniquement les fichiers supprimés
    pub fn removed(&self) -> Vec<&FileChange> {
        self.changes
            .iter()
            .filter(|c| c.kind == ChangeKind::Removed)
            .collect()
    }

    /// Retourne uniquement les fichiers modifiés
    pub fn modified(&self) -> Vec<&FileChange> {
        self.changes
            .iter()
            .filter(|c| c.kind == ChangeKind::Modified)
            .collect()
    }

    /// Retourne true si aucun changement n'a été détecté
    pub fn is_clean(&self) -> bool {
        self.changes.is_empty()
    }

    /// Nombre total de changements
    pub fn count(&self) -> usize {
        self.changes.len()
    }
}


// Fonction principale de comparaison

// Compare deux snapshots et retourne un DiffResult.
pub fn compare_snapshots(
    old: &HashMap<String, String>,
    new: &HashMap<String, String>,
) -> DiffResult {
    let mut changes: Vec<FileChange> = Vec::new();

    // Étape 1 : Parcourir l'ancien snapshot
    // Pour chaque fichier connu, vérifier s'il existe encore et si son hash est identique. 
    for (path, old_hash) in old {
        match new.get(path) {
            Some(new_hash) if new_hash == old_hash => {
                // Hash identique → rien à signaler, le fichier est intact
            }
            Some(new_hash) => {
                // Hash différent → le fichier a été modifié
                changes.push(FileChange {
                    path: path.clone(),
                    kind: ChangeKind::Modified,
                    old_hash: Some(old_hash.clone()),
                    new_hash: Some(new_hash.clone()),
                });
            }
            None => {
                // Fichier absent du nouveau snapshot → supprimé
                changes.push(FileChange {
                    path: path.clone(),
                    kind: ChangeKind::Removed,
                    old_hash: Some(old_hash.clone()),
                    new_hash: None,
                });
            }
        }
    }

    // Étape 2 : Parcourir le nouveau snapshot 
    // Chercher les fichiers qui n'existaient pas dans l'ancien snapshot.
    for (path, new_hash) in new {
        if !old.contains_key(path) {
            // Nouveau fichier → ajouté
            changes.push(FileChange {
                path: path.clone(),
                kind: ChangeKind::Added,
                old_hash: None,
                new_hash: Some(new_hash.clone()),
            });
        }
    }

    // Étape 3 : Trier par chemin pour un affichage reproductible
    changes.sort_by(|a, b| a.path.cmp(&b.path));

    DiffResult {
        changes,
        total_old: old.len(),
        total_new: new.len(),
    }
}


// Tests unitaires

#[cfg(test)]
mod tests {
    use super::*;

    // Construit un HashMap<String,String> à partir de paires littérales
    fn snapshot(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn test_aucun_changement() {
        let old = snapshot(&[("/etc/passwd", "aaa"), ("/etc/hosts", "bbb")]);
        let new = old.clone();
        let result = compare_snapshots(&old, &new);
        assert!(result.is_clean(), "Aucun changement attendu");
    }

    #[test]
    fn test_fichier_ajoute() {
        let old = snapshot(&[("/etc/passwd", "aaa")]);
        let new = snapshot(&[("/etc/passwd", "aaa"), ("/etc/shadow", "ccc")]);
        let result = compare_snapshots(&old, &new);
        assert_eq!(result.added().len(), 1);
        assert_eq!(result.added()[0].path, "/etc/shadow");
    }

    #[test]
    fn test_fichier_supprime() {
        let old = snapshot(&[("/etc/passwd", "aaa"), ("/etc/cron", "bbb")]);
        let new = snapshot(&[("/etc/passwd", "aaa")]);
        let result = compare_snapshots(&old, &new);
        assert_eq!(result.removed().len(), 1);
        assert_eq!(result.removed()[0].path, "/etc/cron");
    }

    #[test]
    fn test_fichier_modifie() {
        let old = snapshot(&[("/etc/passwd", "aaa")]);
        let new = snapshot(&[("/etc/passwd", "zzz")]); // hash différent
        let result = compare_snapshots(&old, &new);
        assert_eq!(result.modified().len(), 1);
        let change = &result.modified()[0];
        assert_eq!(change.old_hash, Some("aaa".to_string()));
        assert_eq!(change.new_hash, Some("zzz".to_string()));
    }

    #[test]
    fn test_changements_multiples() {
        let old = snapshot(&[
            ("/a", "hash_a"),
            ("/b", "hash_b"), // sera supprimé
            ("/c", "hash_c"), // sera modifié
        ]);
        let new = snapshot(&[
            ("/a", "hash_a"),             // inchangé
            ("/c", "hash_c_modifie"),     // modifié
            ("/d", "hash_d"),             // ajouté
        ]);
        let result = compare_snapshots(&old, &new);
        assert_eq!(result.added().len(), 1);
        assert_eq!(result.removed().len(), 1);
        assert_eq!(result.modified().len(), 1);
        assert_eq!(result.count(), 3);
    }

    #[test]
    fn test_tri_par_chemin() {
        let old = snapshot(&[("/z", "1"), ("/a", "2")]);
        let new: HashMap<String, String> = HashMap::new();
        let result = compare_snapshots(&old, &new);
        // Les suppressions doivent être triées alphabétiquement
        assert_eq!(result.changes[0].path, "/a");
        assert_eq!(result.changes[1].path, "/z");
    }
}
