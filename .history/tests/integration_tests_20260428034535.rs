// Tests d'intégration — toutes les parties ensemble
// À compléter en fin de projet (Étape 7)

#[cfg(test)]
mod tests {
    #[test]
    fn test_full_scan_diff_cycle() {
        // 1. Scanner un dossier → baseline
        // 2. Modifier un fichier
        // 3. Scanner à nouveau
        // 4. Comparer → vérifier que la modification est détectée
        todo!("Test d'intégration complet")
    }

    #[test]
    fn test_daemon_detects_change() {
        todo!("Test du daemon sur une modification simulée")
    }
}
