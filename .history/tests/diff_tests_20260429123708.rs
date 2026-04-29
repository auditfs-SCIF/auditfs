/use auditfs::diff::compare::{compare, ChangeType, DiffResult, FileChange};
use auditfs::diff::report;
use auditfs::hashing::{DirectorySnapshot, FileSnapshot};
use std::collections::HashMap;

fn make_snap(files: Vec<(&str, &str, u64)>) -> DirectorySnapshot {
    let mut map = HashMap::new();
    for (path, hash, size) in files {
        map.insert(
            path.to_string(),
            FileSnapshot {
                path: path.to_string(),
                size,
                sha256: hash.to_string(),
                blake3: hash.to_string(),
                permissions: 0o644,
                owner: 1000,
                modified_at: 0,
            },
        );
    }
    DirectorySnapshot {
        root: "/test".to_string(),
        files: map,
        created_at: 0,
    }
}

#[test]
fn test_detect_added_file() {
    let before = make_snap(vec![]);
    let after = make_snap(vec![("/test/nouveau.txt", "hash123", 10)]);
    let diff = compare(&before, &after);
    assert!(
        diff.changes
            .iter()
            .any(|c| matches!(c.change, ChangeType::Added)),
        "Un fichier ajouté doit être détecté"
    );
}

#[test]
fn test_detect_removed_file() {
    let before = make_snap(vec![("/test/ancien.txt", "hash123", 10)]);
    let after = make_snap(vec![]);
    let diff = compare(&before, &after);
    assert!(
        diff.changes
            .iter()
            .any(|c| matches!(c.change, ChangeType::Removed)),
        "Un fichier supprimé doit être détecté"
    );
}

#[test]
fn test_detect_modified_file() {
    let before = make_snap(vec![("/test/config.txt", "hash_original", 10)]);
    let after = make_snap(vec![("/test/config.txt", "hash_modifie", 10)]);
    let diff = compare(&before, &after);
    assert!(
        diff.changes
            .iter()
            .any(|c| matches!(c.change, ChangeType::Modified { .. })),
        "Un fichier modifié doit être détecté"
    );
}

#[test]
fn test_no_changes_when_identical() {
    let snap = make_snap(vec![("/test/stable.txt", "hash_stable", 10)]);
    let diff = compare(&snap, &snap);
    let real_changes: Vec<_> = diff
        .changes
        .iter()
        .filter(|c| !matches!(c.change, ChangeType::DangerousPermission))
        .collect();
    assert!(
        real_changes.is_empty(),
        "Aucun changement si les snapshots sont identiques"
    );
}

#[test]
fn test_detect_dangerous_permission() {
    let mut map = HashMap::new();
    map.insert(
        "/test/danger.sh".to_string(),
        FileSnapshot {
            path: "/test/danger.sh".to_string(),
            size: 10,
            sha256: "hash1".to_string(),
            blake3: "hash1".to_string(),
            permissions: 0o777, // world-writable
            owner: 1000,
            modified_at: 0,
        },
    );
    let before = make_snap(vec![("/test/danger.sh", "hash1", 10)]);
    let after = DirectorySnapshot {
        root: "/test".to_string(),
        files: map,
        created_at: 0,
    };
    let diff = compare(&before, &after);
    assert!(
        diff.changes
            .iter()
            .any(|c| matches!(c.change, ChangeType::DangerousPermission)),
        "Les permissions world-writable doivent être signalées"
    );
}

#[test]
fn test_export_json() {
    let diff = DiffResult {
        changes: vec![
            FileChange {
                path: "/test/f.txt".to_string(),
                change: ChangeType::Added,
            },
            FileChange {
                path: "/test/g.txt".to_string(),
                change: ChangeType::Removed,
            },
        ],
    };
    let json = report::to_json(&diff).unwrap();
    assert!(json.contains("Added"), "JSON doit contenir Added");
    assert!(json.contains("Removed"), "JSON doit contenir Removed");
    assert!(json.contains("/test/f.txt"));
}

#[test]
fn test_export_text() {
    let diff = DiffResult {
        changes: vec![FileChange {
            path: "/test/nouveau.txt".to_string(),
            change: ChangeType::Added,
        }],
    };
    let text = report::to_text(&diff);
    assert!(
        text.contains("AJOUTÉ"),
        "Le rapport texte doit mentionner AJOUTÉ"
    );
    assert!(text.contains("/test/nouveau.txt"));
}

#[test]
fn test_export_html() {
    let diff = DiffResult {
        changes: vec![FileChange {
            path: "/test/modifie.txt".to_string(),
            change: ChangeType::Modified {
                changed_attributes: vec!["sha256".to_string()],
            },
        }],
    };
    let html = report::to_html(&diff);
    assert!(html.contains("<table>"), "HTML doit contenir une table");
    assert!(
        html.contains("/test/modifie.txt"),
        "HTML doit contenir le chemin du fichier"
    );
    assert!(html.contains("Modifié"), "HTML doit mentionner Modifié");
}

#[test]
fn test_export_text_empty() {
    let diff = DiffResult { changes: vec![] };
    let text = report::to_text(&diff);
    assert!(text.contains("Aucun changement"));
}
*/