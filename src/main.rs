mod generator;
mod lexer;
mod node;
mod parser;
mod token;

use flexi_logger::{colored_default_format, Duplicate, Logger};
use generator::generate;
use lexer::lex;
use log::*;
use parser::parse;
use std::fs::read_to_string;
use std::time::{Duration, Instant};

fn main() {
    Logger::with_env_or_str("debug")
        .duplicate_to_stdout(Duplicate::All)
        .format_for_stdout(colored_default_format)
        .start()
        .unwrap();
    let parse_start = Instant::now();
    let file = read_to_string("src/data/test.s").expect("File not found");
    lex_file(&file);
    parse();
    let parse_end = Instant::now();
    log_time("Parsing", parse_end - parse_start);
    let generate_start = Instant::now();
    generate();
    let generate_end = Instant::now();
    log_time("Generation", generate_end - generate_start);
}

fn lex_file(file: &String) {
    let lex_start = Instant::now();
    lex(file);
    let lex_end = Instant::now();
    log_time("Lexing", lex_end - lex_start);
}

fn log_time(name: &str, dur: Duration) {
    info!("{} took {} micros", name, dur.as_micros());
}
