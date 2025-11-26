use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opt_c = false;
    let mut opt_l = false;
    let mut opt_w = false;

    let mut filename: Option<String> = None;

    for arg in &args[1..] {
        match arg.as_str() {
            "-c" => opt_c = true,
            "-l" => opt_l = true,
            "-w" => opt_w = true,
            _ => filename = Some(arg.clone()),
        }
    }

    if !opt_c && !opt_l && !opt_w {
        opt_c = true;
        opt_l = true;
        opt_w = true;
    }

    let file = match filename {
        Some(name) => name,
        None => return,
    };

    let content = read_file(&file);

    let mut bytes = None;
    let mut lines = None;
    let mut words = None;

    if opt_c {
        bytes = Some(count_bytes(&content));
    }
    if opt_l {
        lines = Some(count_lines(&content));
    }
    if opt_w {
        words = Some(count_words(&content));
    }

    print_results(lines, words, bytes, &file);
}

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("Impossible de lire le fichier")
}

fn count_bytes(content: &str) -> usize {
    content.as_bytes().len()
}


fn count_lines(content: &str) -> usize {
    content.lines().count()
}


fn count_words(_content: &str) -> usize {
    _content.split_whitespace().count()
}

fn print_results(_lines: Option<usize>, _words: Option<usize>, _bytes: Option<usize>, _filename: &str) {
    if let Some(l) = _lines {
        print!("{}\t", l);
    }
    if let Some(w) = _words {
        print!("{}\t", w);
    }
    if let Some(c) = _bytes {
        print!("{}\t", c);
    }
    println!("{}", _filename);
}

