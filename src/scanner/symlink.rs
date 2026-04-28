// Partie 3 — Gestion des symlinks (sans boucle infinie)
// TODO: détecter et résoudre les symlinks
// Utiliser un HashSet de inodes déjà visités

pub fn is_symlink_loop(_path: &str, _visited_inodes: &std::collections::HashSet<u64>) -> bool {
    todo!("Détecter les boucles de symlinks via inodes")
}
