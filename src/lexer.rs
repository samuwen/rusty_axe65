use crate::char_helper::*;
use crate::token::{Token, TokenType};

pub fn lex(file: &String) -> Vec<Token> {
  if &file[file.len() - 1..] != "\n" {
    panic!("File needs to end in a newline");
  }
  let file_len = file.len();
  let mut out_vec = Vec::with_capacity(file_len);
  let mut characters = Characters::new(file.chars().collect(), file_len);
  let mut next = next_token(&mut characters);
  while next.get_type() != &TokenType::EndOfFile {
    out_vec.push(next);
    next = next_token(&mut characters);
  }
  out_vec.push(next);
  out_vec
    .into_iter()
    .filter(|t| t.get_type() != &TokenType::Whitespace && t.get_type() != &TokenType::Comment)
    .collect()
}

fn next_token(chars: &mut Characters) -> Token {
  let empty = String::from("");
  let start = chars.get_index();
  if chars.get_index() >= chars.max_size() {
    return Token::new(empty, TokenType::EndOfFile, start, chars.max_size());
  }
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
  if is_operator(next) {
    return handle_operator(chars);
  }
  if is_whitespace(next) {
    return Token::new(empty, TokenType::Whitespace, start, chars.get_index());
  }
  if is_newline(next) {
    return Token::new(empty, TokenType::Newline, start, chars.get_index());
  }
  Token::new(String::from(""), TokenType::EndOfFile, 0, 0)
}

fn handle_number(chars: &mut Characters) -> Token {
  if is_hex_signifier(chars.get_current()) {
    return create_number_token(chars, is_hex_number, TokenType::HexNumber);
  }
  if is_bin_signifier(chars.get_current()) {
    return create_number_token(chars, is_bin_number, TokenType::BinNumber);
  }
  if is_dec_signifier(chars.get_current()) {
    return create_number_token(chars, is_dec_number, TokenType::DecNumber);
  }
  panic!("Invalid number provided: {}", chars.get_current());
}

fn create_number_token<F: Fn(char) -> bool>(
  chars: &mut Characters,
  keep_going: F,
  t: TokenType,
) -> Token {
  let start = chars.get_index() - 1;
  let mut next = chars.get_current();
  let mut token_string = String::from(next);
  next = chars.peek_next();
  while keep_going(next) {
    let c = chars.get_next();
    token_string.push(c);
    next = chars.peek_next();
  }
  Token::new(token_string, t, start, chars.get_index())
}

fn handle_control_command(chars: &mut Characters) -> Token {
  let start = chars.get_index();
  let period = chars.get_current();
  if period != '.' {
    panic!("Expected '.' got {}", period);
  }
  chars.get_next();
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
  let period = chars.get_current();
  if period != '@' {
    panic!("Expected '@' got {}", period);
  }
  chars.get_next();
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
  let next = chars.peek_next();
  let token_string = match is_identifier(next) {
    true => get_identifier_text(chars),
    false => String::from(chars.get_current()),
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

fn handle_operator(chars: &mut Characters) -> Token {
  let start = chars.get_index() - 1;
  let current = chars.get_current();
  let next = chars.peek_next();
  let end = chars.get_index();
  match is_combo_operator(current) {
    true => match current {
      '<' => match next {
        '<' => handle_combo_operator("<<", TokenType::Shl, start, end),
        '=' => handle_combo_operator("<=", TokenType::LessThanOrEqual, start, end),
        _ => handle_single_operator(current, TokenType::LessThan, start, end),
      },
      '>' => match next {
        '>' => handle_combo_operator(">>", TokenType::Shr, start, end),
        '=' => handle_combo_operator(">=", TokenType::GreaterThanOrEqual, start, end),
        _ => handle_single_operator(current, TokenType::GreaterThan, start, end),
      },
      ':' => match next {
        ':' => handle_combo_operator("::", TokenType::Namespace, start, end),
        '=' => handle_combo_operator(":=", TokenType::Assignment, start, end),
        '+' => handle_unnamed_label(TokenType::ULabel, start, end, chars, '+'),
        '-' => handle_unnamed_label(TokenType::ULabel, start, end, chars, '-'),
        _ => handle_single_operator(current, TokenType::Colon, start, end),
      },
      _ => panic!("Not yet implemented {}", current),
    },
    false => match current {
      ';' => handle_comment(chars),
      '+' => handle_single_operator(current, TokenType::Addition, start, end),
      '-' => handle_single_operator(current, TokenType::Subtraction, start, end),
      '*' => handle_single_operator(current, TokenType::Multiplication, start, end),
      '/' => handle_single_operator(current, TokenType::Division, start, end),
      '=' => handle_single_operator(current, TokenType::Equal, start, end),
      '^' => handle_single_operator(current, TokenType::Xor, start, end),
      '&' => handle_single_operator(current, TokenType::And, start, end),
      '|' => handle_single_operator(current, TokenType::Or, start, end),
      ',' => handle_single_operator(current, TokenType::Comma, start, end),
      '~' => handle_single_operator(current, TokenType::Not, start, end),
      '!' => handle_single_operator(current, TokenType::BoolNot, start, end),
      '<' => handle_single_operator(current, TokenType::LessThan, start, end),
      '>' => handle_single_operator(current, TokenType::GreaterThan, start, end),
      '(' => handle_single_operator(current, TokenType::OParen, start, end),
      ')' => handle_single_operator(current, TokenType::CParen, start, end),
      '[' => handle_single_operator(current, TokenType::OBracket, start, end),
      ']' => handle_single_operator(current, TokenType::CBracket, start, end),
      '{' => handle_single_operator(current, TokenType::OCurly, start, end),
      '}' => handle_single_operator(current, TokenType::CCurly, start, end),
      '#' => handle_single_operator(current, TokenType::Hash, start, end),
      ':' => handle_single_operator(current, TokenType::Colon, start, end),
      '"' => handle_string_constant(chars, start),
      _ => panic!("Unrecognized operator: {}", current),
    },
  }
}

fn get_identifier_text(chars: &mut Characters) -> String {
  let mut next = chars.get_current();
  if !is_id_start(next) {
    panic!("Invalid identifier starting point \"{}\"", next);
  }
  let mut token_string = String::from(next);
  next = chars.peek_next();
  while is_identifier(next) {
    let c = chars.get_next();
    token_string.push(c);
    next = chars.peek_next();
  }
  token_string
}

fn handle_single_operator(c: char, t: TokenType, s: usize, e: usize) -> Token {
  Token::new(String::from(c), t, s, e)
}

fn handle_combo_operator(c: &str, t: TokenType, s: usize, e: usize) -> Token {
  Token::new(String::from(c), t, s, e)
}

fn handle_unnamed_label(
  t: TokenType,
  s: usize,
  e: usize,
  chars: &mut Characters,
  first: char,
) -> Token {
  let mut out_string = String::from(chars.get_current());
  let mut next = chars.peek_next();
  while next == first {
    let c = chars.get_next();
    out_string.push(c);
    next = chars.peek_next();
  }
  Token::new(out_string, t, s, e)
}

fn handle_comment(chars: &mut Characters) -> Token {
  let start = chars.get_index() - 1;
  let mut next = chars.peek_next();
  let mut end = String::new();
  while next != '\n' {
    let c = chars.get_next();
    end.push(c);
    next = chars.peek_next();
  }
  Token::new(end, TokenType::Comment, start, chars.get_index())
}

fn handle_string_constant(chars: &mut Characters, s: usize) -> Token {
  let mut next = chars.peek_next();
  let mut out_string = String::new();
  while next != '"' {
    let c = chars.get_next();
    out_string.push(c);
    next = chars.peek_next();
  }
  let close_quote = chars.get_next();
  if close_quote != '"' {
    panic!("String constant quotes not closed");
  }
  Token::new(out_string, TokenType::StringConst, s, chars.get_index())
}

struct Characters {
  cur_index: usize,
  chars: Vec<char>,
  max_size: usize,
}

impl Characters {
  fn new(chars: Vec<char>, max: usize) -> Characters {
    Characters {
      cur_index: 0,
      chars,
      max_size: max,
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

  fn get_current(&self) -> char {
    self.chars[self.cur_index - 1]
  }

  fn get_previous(&self) -> char {
    self.chars[self.cur_index - 2]
  }

  fn peek_next(&mut self) -> char {
    self.chars[self.cur_index]
  }

  fn max_size(&self) -> usize {
    self.max_size
  }
}
