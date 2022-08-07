#![feature(let_else)]

use std::io::BufWriter;

use five_words::{solve, write_results_csv};

fn main() {
    let Some(file_path) = std::env::args_os().nth(1) else {
        eprintln!("Usage: Supply the source file as the first argument");
        return;
    };
    let Ok(words) = std::fs::read_to_string(file_path) else {
        eprintln!("Failed to open input file");
        return;
    };

    let results = solve(words.lines().collect());

    // Write the result to stdout in csv format.
    let mut output = BufWriter::new(std::io::stdout());
    write_results_csv(results, &mut output);
}
