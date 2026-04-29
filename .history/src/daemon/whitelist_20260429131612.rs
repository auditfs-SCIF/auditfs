pub struct Whitelist {
    pub patterns: Vec<String>,
}

impl Whitelist {
    pub fn new(patterns: Vec<String>) -> Self {
        Whitelist { patterns }
    }

    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let patterns = content
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect();
        Ok(Whitelist { patterns })
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
