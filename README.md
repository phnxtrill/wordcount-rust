## Projet TP : `wc` en Rust

Petit projet de TP qui réimplémente une version simplifiée de la commande Unix `wc` (word count) en Rust.

Le programme peut compter :
- le **nombre d'octets** (`-c`)
- le **nombre de lignes** (`-l`)
- le **nombre de mots** (`-w`)

Si aucune option n’est fournie, il affiche les trois (lignes, mots, octets).

### Compilation

Depuis le dossier `wc` :

```bash
cargo build
```

### Exécution avec `cargo run`

Depuis le dossier `wc` :

```bash
# Lignes + mots + octets
cargo run -- test.txt

# Uniquement les octets (équivalent à `ccwc -c test.txt`)
cargo run -- -c test.txt

# Lignes + mots
cargo run -- -l -w test.txt
```

### Exécution via le binaire compilé

Après compilation (`cargo build`), le binaire se trouve dans `target/debug` :

```bash
.\target\debug\wc.exe test.txt
.\target\debug\wc.exe -c test.txt
.\target\debug\wc.exe -l -w test.txt
```


