use crate::node::{Node, NodeType};
use crate::opcode::is_opcode;
use crate::token::{Token, TokenType};

// [[label[:]] [(opcode|directive|macro) [arguments]]] [; comment]

// <program> ::= { <opcode> }
pub fn parse(mut tokens: Vec<Token>) -> Node<String> {
  let mut program_tree = Node::new(&String::from("Program"), NodeType::Program);
  let mut next = peek_next_token(&tokens);
  while next.get_type() == &TokenType::Identifier {
    let child = parse_opcode(&mut tokens);
    program_tree.add_child(child);
    next = peek_next_token(&tokens);
  }
  program_tree
}

// <opcode> ::= <id> [ <args> ]
fn parse_opcode(tokens: &mut Vec<Token>) -> Node<String> {
  let token = get_next_token_checked(tokens, &TokenType::Identifier);
  let next = peek_next_token(tokens);
  let op_node = match is_opcode(&next.get_value()) {
    false => {
      let args = parse_args(tokens);
      Node::new_with_child(token.get_value(), NodeType::ImmediateOpcode, args)
    }
    true => Node::new(token.get_value(), NodeType::ImpliedOpcode),
  };
  op_node
}

// <args> ::= number
fn parse_args(tokens: &mut Vec<Token>) -> Node<String> {
  let token = get_next_token_checked(tokens, &TokenType::Number);
  let mut node = Node::new(&String::from("Number"), NodeType::Number);
  let mut value_chars = token.get_value().chars();
  let first_char = value_chars.next().expect("Value is empty string");
  let data = match first_char {
    '#' => handle_number(value_chars.as_str()),
    _ => handle_number(token.get_value()),
  };
  node.add_data(data.to_string());
  node
}

fn get_next_token_checked(tokens: &mut Vec<Token>, expected: &TokenType) -> Token {
  let token = get_next_token(tokens);
  if token.get_type() != expected {
    panic!("Expected {:?} but got {:?}", expected, token.get_type());
  }
  token
}

fn get_next_token(tokens: &mut Vec<Token>) -> Token {
  tokens.remove(0)
}

fn peek_next_token(tokens: &Vec<Token>) -> Token {
  let option = tokens.get(0);
  match option {
    Some(token) => token.clone(),
    None => Token::new(String::from(""), TokenType::EndOfFile, 0, 0),
  }
}

fn handle_number(num_value: &str) -> u16 {
  let mut chars = num_value.chars();
  let first_char = chars.next().expect("Empty string passed to number handler");
  let (string, base) = match first_char {
    '%' => (chars.as_str(), 2),
    '$' => (chars.as_str(), 16),
    _ => (num_value, 10),
  };
  u16::from_str_radix(string, base).expect(&format!(
    "Invalid number passed to number handler {}",
    num_value
  ))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn handle_num() {
    let result = handle_number("$44");
    assert_eq!(result, 0x44);
    let result = handle_number("68");
    assert_eq!(result, 0x44);
    let result = handle_number("%01000100");
    assert_eq!(result, 0x44);
  }
}
