use crate::token::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub fn lex_config_file(config_file: &String) {
  let mut chars = config_file.chars().peekable();
  let mut next = chars.peek();
  while next.is_some() {
    next_token(&mut chars);
    next = chars.peek();
  }
}

fn next_token(chars: &mut Peekable<Chars>) {
  let c = chars.next();
}
