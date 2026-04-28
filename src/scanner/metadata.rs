// Partie 3 — Collecte des métadonnées système
// TODO: taille, permissions Unix, propriétaire (uid), timestamps

use crate::hashing::FileSnapshot;

pub fn collect(_path: &str) -> anyhow::Result<FileSnapshot> {
    todo!("Implémenter la collecte des métadonnées")
}
