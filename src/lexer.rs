use crate::char_helper::*;
use crate::token::{Token, TokenType};

pub fn lex(file: &String) -> Vec<Token> {
  if &file[file.len() - 1..] != "\n" {
    panic!("File needs to end in a newline");
  }
  let file = remove_comments(file);
  let file_len = file.len();
  let mut out_vec = Vec::with_capacity(file_len);
  let mut characters = Characters::new(file.chars().collect());
  let mut next = next_token(&mut characters);
  while next.get_type() != &TokenType::EndOfFile {
    out_vec.push(next);
    next = next_token(&mut characters);
  }
  out_vec.push(next);
  out_vec
}

fn next_token(chars: &mut Characters) -> Token {
  let next = chars.get_next();
  if is_num_signifier(next) {
    return handle_number(chars);
  }
  if is_ctrl_command_signifier(next) {
    return handle_control_command(chars);
  }
  if is_local_label_signifier(next) {
    return handle_local_label(chars);
  }
  if is_identifier(next) {
    return handle_identifier(chars);
  }
  Token::new(String::from(""), TokenType::EndOfFile, 0, 0)
}

fn handle_number(chars: &mut Characters) -> Token {
  if is_hex_signifier(chars.get_cur_char()) {
    return create_number_token(chars, is_hex_number, TokenType::HexNumber);
  }
  if is_bin_signifier(chars.get_cur_char()) {
    return create_number_token(chars, is_bin_number, TokenType::BinNumber);
  }
  if is_dec_signifier(chars.get_cur_char()) {
    return create_number_token(chars, is_dec_number, TokenType::DecNumber);
  }
  panic!("Invalid number provided: {}", chars.get_cur_char());
}

fn create_number_token<F: Fn(char) -> bool>(
  chars: &mut Characters,
  keep_going: F,
  t: TokenType,
) -> Token {
  let mut next = chars.get_cur_char();
  let start = chars.get_index() - 1;
  let mut token_string = String::from(next);
  next = chars.get_next();
  while keep_going(next) {
    token_string.push(next);
    next = chars.get_next();
  }
  Token::new(token_string, t, start, chars.get_index())
}

fn handle_control_command(chars: &mut Characters) -> Token {
  let start = chars.get_index();
  let token_string = get_identifier_text(chars);
  let dir_string = token_string.to_ascii_lowercase();
  Token::new(
    token_string,
    TokenType::get_directive_type(&dir_string),
    start,
    chars.get_index(),
  )
}

fn handle_local_label(chars: &mut Characters) -> Token {
  let start = chars.get_index();
  let token_string = get_identifier_text(chars);
  Token::new(
    token_string,
    TokenType::LocalLabel,
    start,
    chars.get_index(),
  )
}

fn handle_identifier(chars: &mut Characters) -> Token {
  let start = chars.get_index() - 1;
  let next = chars.peek_next_char();
  let token_string = match is_identifier(next) {
    true => get_identifier_text(chars),
    false => String::from(chars.get_cur_char()),
  };
  let t = match token_string.len() {
    1 => match token_string.as_ref() {
      "X" => TokenType::XRegister,
      "Y" => TokenType::YRegister,
      _ => panic!("Unknown single char identifier: {}", token_string),
    },
    _ => TokenType::Identifier,
  };
  Token::new(token_string, t, start, chars.get_index())
}

fn get_identifier_text(chars: &mut Characters) -> String {
  let mut next = chars.get_next();
  if !is_id_start(next) {
    panic!("Invalid identifier starting point \"{}\"", next);
  }
  let mut token_string = String::from(next);
  next = chars.get_next();
  while is_identifier(next) {
    token_string.push(next);
    next = chars.get_next();
  }
  token_string
}

// prepass to remove all comments from the file but leave the rest.
// if this is too slow we can revisit
fn remove_comments(file: &String) -> String {
  let mut deleting = false;
  file
    .chars()
    .filter(|c| {
      match c {
        ';' => deleting = true,
        '\n' => deleting = false,
        _ => (),
      }
      deleting == false
    })
    .collect()
}

struct Characters {
  cur_index: usize,
  chars: Vec<char>,
}

impl Characters {
  fn new(chars: Vec<char>) -> Characters {
    Characters {
      cur_index: 0,
      chars,
    }
  }

  fn get_index(&self) -> usize {
    self.cur_index
  }

  fn get_next(&mut self) -> char {
    let c = self.chars[self.cur_index];
    self.cur_index += 1;
    c
  }

  fn get_cur_char(&self) -> char {
    self.chars[self.cur_index - 1]
  }

  fn get_prev_char(&self) -> char {
    self.chars[self.cur_index - 2]
  }

  fn peek_next_char(&mut self) -> char {
    self.chars[self.cur_index]
  }
}
