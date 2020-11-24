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
use token::Token;

fn main() {
    Logger::with_env_or_str("debug")
        .duplicate_to_stdout(Duplicate::All)
        .format_for_stdout(colored_default_format)
        .start()
        .unwrap();
    let parse_start = Instant::now();
    let file = read_to_string("src/data/build.s").expect("File not found");
    let _tokens = lex_file(&file);
    for token in _tokens {
        debug!("{}", token);
    }
    parse();
    let parse_end = Instant::now();
    log_time("Parsing", parse_end - parse_start);
    let generate_start = Instant::now();
    generate();
    let generate_end = Instant::now();
    log_time("Generation", generate_end - generate_start);
}

fn lex_file(file: &String) -> Vec<Token> {
    let lex_start = Instant::now();
    let tokens = lex(file);
    let lex_end = Instant::now();
    log_time("Lexing", lex_end - lex_start);
    tokens
}

fn log_time(name: &str, dur: Duration) {
    info!("{} took {} micros", name, dur.as_micros());
}
