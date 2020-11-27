use crate::node::{Node, NodeType};
use crate::opcode::is_opcode;
use crate::token::{Token, TokenType};

// [[label[:]] [(opcode|directive|macro) [arguments]]] [; comment]

// <program> ::= { <opcode> }
pub fn parse(mut tokens: Vec<Token>) -> Node<String> {
  let mut program_tree = Node::new(NodeType::Program);
  let mut next = peek_next_token(&tokens);
  while next.get_type() != &TokenType::EndOfFile {
    let child = match next.get_type() {
      _ => parse_identifier(&mut tokens),
    };
    program_tree.add_child(child);
    next = peek_next_token(&tokens);
  }
  program_tree
}

fn parse_identifier(tokens: &mut Vec<Token>) -> Node<String> {
  let identifier = get_next_token(tokens);
  let next = peek_next_token(tokens);
  match next.get_type() {
    _ => panic!("Unknown next"),
  }
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
