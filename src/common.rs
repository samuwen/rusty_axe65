use crate::token::*;

pub fn get_next_token_checked(tokens: &mut Vec<Token>, expected: Vec<TokenType>) -> Token {
  let token = get_next_token(tokens);
  let valid = expected.iter().any(|t| token.get_type() == t);
  if !valid {
    error(&token);
  }
  token
}

pub fn get_next_token(tokens: &mut Vec<Token>) -> Token {
  tokens.remove(0)
}

pub fn peek_next_token(tokens: &Vec<Token>) -> Token {
  peek(tokens, 0)
}

pub fn peek_two_ahead(tokens: &Vec<Token>) -> Token {
  peek(tokens, 1)
}

fn peek(tokens: &Vec<Token>, count: usize) -> Token {
  let option = tokens.get(count);
  match option {
    Some(token) => token.clone(),
    None => Token::new(String::from(""), TokenType::EndOfFile, 0, 0, 0),
  }
}

pub fn error(token: &Token) -> ! {
  panic!(
    "Invalid token.\nToken Type: {:?}\nToken Value: {}\nLine Number: {}",
    token.get_type(),
    token.get_value(),
    token.get_line()
  );
}

pub fn convert_number(value: &Token) -> Result<u16, std::num::ParseIntError> {
  match value.get_type() {
    TokenType::HexNumber => u16::from_str_radix(&value.get_value()[1..], 16),
    TokenType::BinNumber => u16::from_str_radix(&value.get_value()[1..], 2),
    TokenType::DecNumber => u16::from_str_radix(&value.get_value(), 10),
    _ => error(&value),
  }
}
