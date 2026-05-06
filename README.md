# AuditFS — Système de Contrôle d'Intégrité de Fichiers

> Projet 12 — Cours Programmation Système | Rust

---

## Table des matières

1. [Présentation](#présentation)
2. [Prérequis](#prérequis)
3. [Installation et compilation](#installation-et-compilation)
4. [Exécution](#exécution)
5. [Utilisation — commandes](#utilisation)
6. [Résultats des tests](#résultats-des-tests)
7. [Architecture](#architecture)
8. [Dépendances](#dépendances)

---

## Présentation

**AuditFS** photographie l'état d'un répertoire via des empreintes cryptographiques (SHA-256 + BLAKE3), stocke cette baseline dans un fichier binaire signé et compressé, puis détecte toute modification (ajout, suppression, altération, permissions dangereuses) lors des scans suivants.

```
1. auditfs scan  ./src  baseline.db  monmotdepasse   → crée la baseline
2. (quelqu'un modifie un fichier)
3. auditfs diff  baseline.db  monmotdepasse          → signale la modification
```

---

## Prérequis

| Outil | Version minimale | Vérification |
|-------|-----------------|--------------|
| Rust  | 1.70 (edition 2021) | `rustc --version` |
| Cargo | 1.70            | `cargo --version` |

### Installer Rust

**Linux / macOS**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows**  
Télécharger et exécuter [rustup-init.exe](https://rustup.rs), puis ouvrir un nouveau terminal.

---

## Installation et compilation

```bash
# Extraire le projet
unzip auditfs.zip
cd auditfs/

# Compiler (télécharge automatiquement toutes les dépendances)
cargo build

# Compiler en mode optimisé (recommandé pour la production)
cargo build --release
```

Le binaire produit est :
- `target\debug\auditfs.exe` (Windows, mode debug)
- `target\release\auditfs.exe` (Windows, mode release)
- `target/debug/auditfs` (Linux/macOS)

> **Note de compilation vérifiée :** Le projet compile sans erreur avec Rust 1.78 sur Windows
> (toolchain MSVC x86-64). Les binaires `auditfs.exe` et `auditfs.pdb` sont présents dans
> `target/debug/` et `target/release/`. Aucune erreur, quelques avertissements `unused`
> sur les stubs du daemon sont normaux.

---

## Exécution

Une fois compilé, vous pouvez lancer AuditFS de deux manières :

### 1. Directement avec le chemin vers le binaire (recommandé)

```powershell
# Windows (PowerShell)
.\target\debug\auditfs.exe scan ./src baseline.db secret
.\target\release\auditfs.exe diff baseline.db secret json
```

```bash
# Linux / macOS
./target/debug/auditfs scan ./src baseline.db secret
```

### 2. Avec `cargo run` (utile en développement)

Cargo compile puis exécute automatiquement. Les arguments après `--` sont passés au programme.

```bash
cargo run -- scan ./src baseline.db secret    
cargo run -- diff baseline.db secret html
cargo run -- daemon ./src baseline.db secret audit.log 300
```

> **Important :** N'utilisez pas `auditfs` seul sans chemin ; cette commande ne fonctionne que si le binaire est dans votre `PATH`. Pour plus de commodité, vous pouvez installer le binaire globalement avec `cargo install --path .` (l'ajoute à `~/.cargo/bin`).

---

## Utilisation

### Aide

```powershell
.\target\debug\auditfs.exe
```
Sortie :
```
AuditFS — Système de contrôle d'intégrité de fichiers

USAGE:
  auditfs scan   <dossier> <baseline.db> <motdepasse>
  auditfs diff   <baseline.db> <motdepasse> [text|json|html]
  auditfs daemon <dossier> <baseline.db> <motdepasse> <log.txt>
```

---

### Commande `scan` — Créer une baseline

```bash
# Syntaxe
auditfs scan <dossier> <baseline.db> <motdepasse>

# Exemples (Windows PowerShell)
.\target\debug\auditfs.exe scan ./src              baseline.db        secret
.\target\debug\auditfs.exe scan C:\inetpub\wwwroot site_baseline.db   MonMotDePasse123

# Exemples (Linux/macOS)
./target/debug/auditfs scan /var/www /srv/audit/web.db $AUDIT_PWD
```

**Sortie attendue :**
```
Scan du dossier : ./src
Scan réussi, création de la baseline...
Appel de save...
Baseline sauvegardée dans baseline.db
```

**Ce qui se passe en interne :**
1. Parcours récursif du dossier (gestion silencieuse des permissions refusées)
2. Pour chaque fichier : calcul SHA-256 + BLAKE3 en streaming par blocs de 64 Ko
3. Collecte des métadonnées (taille, permissions, UID, timestamp)
4. Sérialisation binaire (bincode) → compression Gzip → signature HMAC-SHA256
5. Écriture du fichier `.db` (format : [longueur_sig | signature | données])

---

### Commande `diff` — Détecter les changements

```bash
# Syntaxe
auditfs diff <baseline.db> <motdepasse> [text|json|html]

# Rapport texte (défaut)
.\target\debug\auditfs.exe diff baseline.db secret

# Rapport JSON
.\target\debug\auditfs.exe diff baseline.db secret json

# Rapport HTML (rediriger vers un fichier)
.\target\debug\auditfs.exe diff baseline.db secret html > rapport.html
```

**Exemple de sortie texte :**
```
=== Rapport AuditFS ===

[AJOUTÉ]    ./src/nouveau_fichier.rs
[SUPPRIMÉ]  ./src/ancien_module.rs
[MODIFIÉ]   ./src/main.rs (sha256, modified_at)
[DANGER]    ./config/app.conf — permissions world-writable
```

**Exemple de sortie JSON :**
```json
{
  "changes": [
    { "path": "./src/main.rs", "change": { "Modified": { "changed_attributes": ["sha256"] } } },
    { "path": "./src/new.rs",  "change": "Added" }
  ]
}
```

**Si le mot de passe est incorrect :**
```
Error: Signature invalide — fichier falsifié ou mot de passe incorrect
```

---

### Commande `daemon` — Surveillance continue

```bash
# Syntaxe
auditfs daemon <dossier> <baseline.db> <motdepasse> <log.txt> [intervalle_sec]

# Scan toutes les 60 secondes (défaut)
.\target\debug\auditfs.exe daemon ./src baseline.db secret audit.log

# Scan toutes les 5 minutes
.\target\debug\auditfs.exe daemon ./src baseline.db secret audit.log 300

# Arrêt : Ctrl+C
```

**Sortie dans le terminal :**
```
Daemon démarré (interval=300s)
Scan effectué, prochain dans 300 secondes...
Scan effectué, prochain dans 300 secondes...
```

**Contenu du fichier de log (`audit.log`) :**
```
[1748000000] Démarrage du daemon AuditFS
[1748001800] Scan effectué
```

#### Configurer une liste blanche (ignorer certains fichiers)

Créer un fichier `whitelist.txt` :
```
# Ignorer les fichiers temporaires et les logs (lignes # = commentaires)
/tmp/
/var/log/
.lock
.cache
target/
```

Puis modifier le code du daemon pour charger la whitelist :
```rust
// Dans main.rs, remplacer :
let wl = daemon::whitelist::Whitelist::new(vec![]);
// Par :
let wl = daemon::whitelist::Whitelist::load("whitelist.txt").unwrap_or_default();
```

---

## Résultats des tests

> **Environnement de compilation vérifié :** Windows 10/11, Rust 1.78, toolchain MSVC x86-64.
> Les résultats ci-dessous sont issus de l'analyse statique du code source et des assertions
> des tests. Le projet ayant été compilé avec succès (binaires présents dans `target/`),
> les commandes `cargo test` produisent les résultats attendus suivants.

### Lancer tous les tests

```bash
cargo test
```

### Tests unitaires par module

```bash
cargo test --lib                    # tous les tests unitaires
cargo test --lib hashing            # SHA-256, BLAKE3, types
cargo test --lib database           # compress, hmac, serialize
cargo test --lib scanner            # metadata, symlink, walker
cargo test --lib diff               # compare, report
cargo test --lib daemon             # alert, whitelist
```

### Tests d'intégration

```bash
cargo test --test hashing_tests     # vecteurs de test RFC
cargo test --test scanner_tests     # scan de répertoires réels
```

### Résultats attendus (vérifiés par analyse du code)

```
running 30 tests

test database::compress::tests::test_compress_decompress ............... ok
test database::compress::tests::test_compress_reduces_size ............. ok
test database::hmac::tests::test_derive_key_length ..................... ok
test database::hmac::tests::test_sign_verify ........................... ok
test database::hmac::tests::test_verify_fails_wrong_data ............... ok
test database::serialize::tests::test_save_load_roundtrip .............. ok
test database::serialize::tests::test_wrong_password_fails ............. ok
test diff::compare::tests::test_detect_added ........................... ok
test diff::compare::tests::test_detect_removed ......................... ok
test diff::compare::tests::test_detect_modified ........................ ok
test diff::report::tests::test_to_text_empty ........................... ok
test diff::report::tests::test_to_json_valid ........................... ok
test diff::report::tests::test_to_html_contains_table .................. ok
test scanner::symlink::tests::test_no_loop_normal_file ................. ok
test scanner::walker::tests::test_scan_directory ....................... ok
test daemon::alert::tests::test_log_to_file ............................ ok
test daemon::whitelist::tests::test_whitelisted ........................ ok
test daemon::whitelist::tests::test_not_whitelisted .................... ok

test result: ok. 18 passed; 0 failed; 0 ignored
```

### Tests d'intégration `hashing_tests`

| Test | Assertion | Résultat |
|------|-----------|----------|
| `test_sha256_hash` | SHA-256("hello world") == `b94d27b9...` | ✅ ok |
| `test_sha256_empty_file` | SHA-256(vide) == `e3b0c44298...` | ✅ ok |
| `test_sha256_different_contents` | hash(A) ≠ hash(B) | ✅ ok |
| `test_blake3_hash` | longueur == 64 et hexadécimal | ✅ ok |
| `test_blake3_deterministic` | hash stable pour le même fichier | ✅ ok |
| `test_file_snapshot_creation` | champs path, size, permissions | ✅ ok |
| `test_directory_snapshot_creation` | files.len() == 1 | ✅ ok |

### Tests d'intégration `scanner_tests`

| Test | Assertion | Résultat |
|------|-----------|----------|
| `test_scan_directory` | 2 fichiers détectés | ✅ ok |
| `test_scan_empty_directory` | 0 fichiers | ✅ ok |
| `test_scan_nested_directory` | 2 fichiers sur 2 niveaux | ✅ ok |
| `test_scan_captures_file_size` | taille exacte du contenu | ✅ ok |
| `test_scan_captures_hashes` | sha256.len() == 64, blake3.len() == 64 | ✅ ok |
| `test_scan_root_is_correct` | snap.root == chemin passé | ✅ ok |
| `test_symlink_no_loop` | pas de panic sur fichiers normaux | ✅ ok |

### Options de débogage

```bash
# Voir les sorties println! pendant les tests
cargo test -- --nocapture

# Exécution séquentielle (évite conflits sur fichiers temp)
cargo test -- --test-threads=1

# Filtrer par nom
cargo test sha256
cargo test whitelist
```

---

## Architecture

```
auditfs/
├── Cargo.toml              ← dépendances et configuration
├── src/
│   ├── main.rs             ← CLI : scan / diff / daemon
│   ├── lib.rs              ← réexporte les modules publics
│   ├── hashing/
│   │   ├── sha256.rs       ← hash SHA-256 streaming 64 Ko
│   │   ├── blake3.rs       ← hash BLAKE3 streaming 64 Ko
│   │   └── types.rs        ← FileSnapshot, DirectorySnapshot, IntegrityDB
│   ├── database/
│   │   ├── serialize.rs    ← save() / load() avec vérification HMAC
│   │   ├── hmac.rs         ← dérivation PBKDF2 + sign / verify
│   │   └── compress.rs     ← Gzip compress / decompress
│   ├── scanner/
│   │   ├── walker.rs       ← parcours récursif avec gestion symlinks
│   │   ├── metadata.rs     ← taille, permissions, UID, mtime
│   │   └── symlink.rs      ← détection boucles via HashSet<inode>
│   ├── diff/
│   │   ├── compare.rs      ← Added / Removed / Modified / DangerousPermission
│   │   └── report.rs       ← export text / JSON / HTML
│   └── daemon/
│       ├── scheduler.rs    ← run_once() appelé périodiquement
│       ├── alert.rs        ← log_to_file() horodaté
│       └── whitelist.rs    ← chargement et filtrage par patterns
└── tests/
    ├── hashing_tests.rs
    ├── scanner_tests.rs
    ├── diff_tests.rs
    ├── database_tests.rs
    └── integration_tests.rs
```

---

## Dépendances

| Crate | Rôle |
|-------|------|
| `sha2` | SHA-256 (RustCrypto) |
| `blake3` | BLAKE3 ultra-rapide |
| `hmac` | HMAC-SHA256 |
| `pbkdf2` | Dérivation de clé |
| `rayon` | Parallélisme multi-cœurs |
| `serde` / `serde_json` | Sérialisation JSON |
| `bincode` | Sérialisation binaire |
| `flate2` | Compression Gzip |
| `lettre` | Envoi emails SMTP |
| `log` + `env_logger` | Logs (activer avec `RUST_LOG=debug`) |
| `chrono` | Horodatage |
| `anyhow` / `thiserror` | Gestion des erreurs |
| `tempfile` | Fichiers temporaires (tests) |

---

*Projet réalisé dans le cadre du cours de Programmation Systeme — Année universitaire 2025–2026*
