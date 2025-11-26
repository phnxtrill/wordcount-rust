use std::env;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Options supportées : -c (octets), -l (lignes), -w (mots), -m (caractères)
    let mut opt_c = false;
    let mut opt_l = false;
    let mut opt_w = false;
    let mut opt_m = false;

    // Nom de fichier éventuel (None si on lit depuis stdin)
    let mut filename: Option<String> = None;

    // Parcourt les arguments à partir du second (le premier est le nom du programme)
    for arg in &args[1..] {
        match arg.as_str() {
            "-c" => opt_c = true,
            "-l" => opt_l = true,
            "-w" => opt_w = true,
            "-m" => opt_m = true,
            _ => filename = Some(arg.clone()),
        }
    }

    // Si aucune option n'est spécifiée, on active toutes les options (comportement de `wc`)
    if !opt_c && !opt_l && !opt_w && !opt_m {
        opt_c = true;
        opt_l = true;
        opt_w = true;
    }

    // Lit le contenu depuis un fichier si un nom est fourni, sinon depuis stdin
    let content = match filename {
        Some(ref name) => read_file(name),
        None => read_stdin(),
    };

    // Variables Option pour stocker les différents compteurs
    let mut bytes = None;
    let mut lines = None;
    let mut words = None;
    let mut chars = None;

    // Effectue uniquement les calculs demandés par les options
    if opt_c {
        bytes = Some(count_bytes(&content));
    }
    if opt_l {
        lines = Some(count_lines(&content));
    }
    if opt_w {
        words = Some(count_words(&content));
    }
    if opt_m {
        chars = Some(count_chars(&content));
    }

    print_results(lines, words, chars, bytes, filename.as_deref());
}

fn read_file(path: &str) -> String {
    // Lit tout le fichier en une seule fois dans une String
    std::fs::read_to_string(path).expect("Impossible de lire le fichier")
}

fn read_stdin() -> String {
    // Lit tout ce qui arrive sur stdin dans une String
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Impossible de lire depuis stdin");
    buffer
}

fn count_bytes(content: &str) -> usize {
    // Nombre d'octets du texte (taille en bytes)
    content.as_bytes().len()
}

fn count_lines(content: &str) -> usize {
    // Nombre de lignes (séparées par '\n')
    content.lines().count()
}

fn count_words(content: &str) -> usize {
    // Nombre de mots (séparés par des espaces/blancs)
    content.split_whitespace().count()
}

fn count_chars(content: &str) -> usize {
    // Nombre de caractères Unicode
    content.chars().count()
}

fn print_results(
    _lines: Option<usize>,
    _words: Option<usize>,
    _chars: Option<usize>,
    _bytes: Option<usize>,
    _filename: Option<&str>,
) {
    // Affiche les résultats dans l'ordre : lignes, mots, caractères, octets
    if let Some(l) = _lines {
        print!("{}\t", l);
    }
    if let Some(w) = _words {
        print!("{}\t", w);
    }
    if let Some(m) = _chars {
        print!("{}\t", m);
    }
    if let Some(c) = _bytes {
        print!("{}\t", c);
    }
    if let Some(f) = _filename {
        print!("{}", f);
    }
    println!();
}