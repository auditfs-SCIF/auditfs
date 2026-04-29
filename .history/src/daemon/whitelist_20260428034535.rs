// Partie 5 — Liste blanche de patterns (faux positifs)
// TODO: charger une liste de patterns glob/regex
// Filtrer les changements qui matchent la liste blanche

pub struct Whitelist {
    pub patterns: Vec<String>,
}

impl Whitelist {
    pub fn load(_path: &str) -> anyhow::Result<Self> {
        todo!("Charger la liste blanche depuis un fichier")
    }

    pub fn is_whitelisted(&self, _file_path: &str) -> bool {
        todo!("Vérifier si un chemin correspond à un pattern")
    }
}
