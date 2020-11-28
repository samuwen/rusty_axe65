use crate::node::{Node, NodeType};
use crate::token::{Token, TokenType};
use log::*;

// [label] ((mnemonoic|macro|ctrl_cmd) [arguments]|assignment "=" value)

// <program> ::= { <opcode> }
pub fn parse(mut tokens: Vec<Token>) -> Node<String> {
  let mut program_tree = Node::new(NodeType::Program);
  let mut next = peek_next_token(&tokens);
  while next.get_type() != &TokenType::EndOfFile {
    let child = parse_statement(&mut tokens);
    if child.get_type() != &NodeType::Undef {
      program_tree.add_child(child);
    }
    next = peek_next_token(&tokens);
  }
  program_tree
}

fn parse_statement(tokens: &mut Vec<Token>) -> Node<String> {
  let next = peek_next_token(tokens);
  let statement = match next.get_type() {
    TokenType::Opcode => parse_opcode(tokens),
    TokenType::Identifier => parse_const_assignment(tokens),
    _ => match next.get_type().is_directive() {
      true => parse_directive(tokens),
      false => parse_expression(tokens),
    },
  };
  statement
}

fn parse_opcode(tokens: &mut Vec<Token>) -> Node<String> {
  let mut opcode_node = Node::new(NodeType::OpcodeStatement);
  let token = get_next_token(tokens);
  opcode_node.add_data(token.get_value());
  let mut next = peek_next_token(tokens);
  while next.get_type() != &TokenType::Newline {
    let child = parse_arguments(tokens);
    opcode_node.add_child(child);
    next = peek_next_token(tokens);
  }
  opcode_node
}

fn parse_const_assignment(tokens: &mut Vec<Token>) -> Node<String> {
  let mut assignment = Node::new(NodeType::AssignmentStatement);
  let var = get_next_token_checked(tokens, vec![TokenType::Identifier]);
  let mut variable = Node::new(NodeType::Variable);
  variable.add_data(var.get_value());
  assignment.add_child(variable);
  let operator = get_next_token_checked(tokens, vec![TokenType::Equal]);
  assignment.add_data(operator.get_value());
  let val = get_next_token_checked(
    tokens,
    vec![
      TokenType::HexNumber,
      TokenType::BinNumber,
      TokenType::DecNumber,
    ],
  );
  let mut value = Node::new(NodeType::Variable);
  value.add_data(val.get_value());
  assignment.add_child(value);
  assignment
}

fn parse_directive(tokens: &mut Vec<Token>) -> Node<String> {
  let mut directive = Node::new(NodeType::DirectiveStatement);
  let dir_name = get_next_token(tokens);
  if !dir_name.get_type().is_directive() {
    panic!("Expected directive, got {:?}", dir_name.get_type());
  }
  let n_type = NodeType::from_token_type(dir_name.get_type());
  let mut child = Node::new(n_type);
  let argument = get_next_token(tokens);
  child.add_data(argument.get_value());
  let mut next = peek_next_token(tokens);
  while next.get_type() == &TokenType::Comma {
    let _ = get_next_token(tokens);
    let t = get_next_token(tokens);
    child.add_data(t.get_value());
    next = peek_next_token(tokens);
  }
  directive.add_child(child);
  directive
}

fn parse_arguments(tokens: &mut Vec<Token>) -> Node<String> {
  todo!();
}

// !
fn parse_expression(tokens: &mut Vec<Token>) -> Node<String> {
  let boolean_not_exp = parse_boolean_or(tokens);
  boolean_not_exp
}

// ||
fn parse_boolean_or(tokens: &mut Vec<Token>) -> Node<String> {
  let boolean_or_exp = parse_boolean_and_xor(tokens);
  boolean_or_exp
}

// && .XOR
fn parse_boolean_and_xor(tokens: &mut Vec<Token>) -> Node<String> {
  let boolean_and_xor_exp = parse_relational(tokens);
  boolean_and_xor_exp
}

// < <= > >= = <>
fn parse_relational(tokens: &mut Vec<Token>) -> Node<String> {
  let relational_exp = parse_binary_op_bit_or(tokens);
  relational_exp
}

// + - |
fn parse_binary_op_bit_or(tokens: &mut Vec<Token>) -> Node<String> {
  let binary_op_bit_or_exp = parse_exp(tokens);
  binary_op_bit_or_exp
}

// * / % & ^ << >>
fn parse_exp(tokens: &mut Vec<Token>) -> Node<String> {
  let exp = parse_term(tokens);
  exp
}

// .BANKBYTE < > (LOBYTE HIBYTE) ~ unary + unary -, built-in pseudos
fn parse_term(tokens: &mut Vec<Token>) -> Node<String> {
  let term = parse_factor(tokens);
  term
}

// build in string functions
fn parse_factor(tokens: &mut Vec<Token>) -> Node<String> {
  let token = get_next_token(tokens);
  let n_type = match token.get_type() {
    TokenType::Newline => NodeType::Undef,
    TokenType::Identifier => NodeType::Identifier,
    _ => NodeType::Undef,
  };
  Node::new(n_type)
}

fn get_next_token_checked(tokens: &mut Vec<Token>, expected: Vec<TokenType>) -> Token {
  let token = get_next_token(tokens);
  let valid = expected.iter().any(|t| token.get_type() == t);
  if !valid {
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
