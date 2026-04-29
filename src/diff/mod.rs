// Ce module expose les fonctions publiques.Il regroupe :
//   - compare   : comparer deux snapshots et produire un DiffResult
//   - report    : sérialiser le résultat en TEXT,JSON ou HTML


pub mod compare;
pub mod report;

// Ré-exporte les types principaux
pub use compare::{DiffResult, FileChange, ChangeKind};
pub use report::{to_text, to_json, to_html};
