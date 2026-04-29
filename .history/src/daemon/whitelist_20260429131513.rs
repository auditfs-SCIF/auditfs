pub struct Whitelist {
    pub patterns: Vec<String>,
}

impl Whitelist {
    pub fn load(_path: &str) -> anyhow::Result<Self> {
        todo!("Charger la liste blanche depuis un fichier")
    }

    pub fn is_whitelisted(&self, file_path: &str) -> bool {
        self.patterns.iter().any(|p| file_path.contains(p.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitelisted() {
        let wl = Whitelist::new(vec!["/tmp/cache".to_string()]);
        assert!(wl.is_whitelisted("/tmp/cache/file.tmp"));
    }

    #[test]
    fn test_not_whitelisted() {
        let wl = Whitelist::new(vec!["/tmp/cache".to_string()]);
        assert!(!wl.is_whitelisted("/etc/passwd"));
    }
}
