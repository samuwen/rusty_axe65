mod char_helper;
mod generator;
mod lexer;
mod node;
mod opcode;
mod parser;
mod token;

use flexi_logger::{colored_default_format, Duplicate, Logger};
use generator::generate;
use lexer::lex;
use log::*;
use node::Node;
use parser::parse;
use std::fs::{read_to_string, write};
use std::time::{Duration, Instant};
use token::Token;

fn main() {
    Logger::with_env_or_str("debug")
        .duplicate_to_stdout(Duplicate::All)
        .format_for_stdout(colored_default_format)
        .start()
        .unwrap();
    let file = read_to_string("src/data/build.s").expect("File not found");
    let tokens = lex_file(&file);
    let tree = parse_file(tokens);
    // generate_file(tree);
}

fn lex_file(file: &String) -> Vec<Token> {
    let lex_start = Instant::now();
    let tokens = lex(file);
    let lex_end = Instant::now();
    log_time("Lexing", lex_end - lex_start);
    let to_file = tokens.clone();
    let out: Vec<String> = to_file.iter().map(|t| format!("{}", t)).collect();
    write("src/out/lexed.out", out.join("\n")).unwrap();
    tokens
}

fn parse_file(tokens: Vec<Token>) -> Node<String> {
    let parse_start = Instant::now();
    let tree = parse(tokens);
    let parse_end = Instant::now();
    log_time("Parsing", parse_end - parse_start);
    let to_file = tree.clone();
    let out = format!("{}", to_file);
    write("src/out/parsed.out", out).unwrap();
    tree
}

fn generate_file(tree: Node<String>) {
    let generate_start = Instant::now();
    let generated = generate(tree);
    let generate_end = Instant::now();
    log_time("Generation", generate_end - generate_start);
    write("src/out/generated.out", generated.join("\n")).unwrap();
}

fn log_time(name: &str, dur: Duration) {
    info!("{} took {} micros", name, dur.as_micros());
}
