use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log_to_file(path: &str, message: &str) -> anyhow::Result<()> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "[{}] {}", ts, message)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_log_to_file() {
        let f = NamedTempFile::new().unwrap();
        log_to_file(f.path().to_str().unwrap(), "test alerte").unwrap();
        let content = std::fs::read_to_string(f.path()).unwrap();
        assert!(content.contains("test alerte"));
    }
}
