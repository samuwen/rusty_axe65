mod generator;
mod lexer;
mod parser;

use generator::generate;
use lexer::lex;
use parser::parse;

fn main() {
    lex();
    parse();
    generate();
}
