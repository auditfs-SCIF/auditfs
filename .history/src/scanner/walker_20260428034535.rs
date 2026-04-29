// Partie 3 — Scan récursif avec rayon (parallèle)
// TODO: parcourir l'arborescence en parallèle avec rayon::par_iter
// Gérer les erreurs de permission sans planter

use crate::hashing::DirectorySnapshot;

pub fn scan(_root: &str) -> anyhow::Result<DirectorySnapshot> {
    todo!("Implémenter le scan parallèle avec rayon")
}
