use crate::node::{Node, NodeType};
use crate::opcode::is_opcode;
use crate::token::{Token, TokenType};
use log::*;

// <program> ::= { <statement> }
pub fn parse(mut tokens: Vec<Token>) -> Node<String> {
  let mut program_tree = Node::new(NodeType::Program);
  let mut next = peek_next_token(&tokens);
  while next.get_type() != &TokenType::EndOfFile {
    program_tree.add_child(parse_statement(&mut tokens));
    next = peek_next_token(&tokens);
  }
  program_tree
}

// <statement> ::= <assignment> | <directive> | <label> | <opcode>
fn parse_statement(tokens: &mut Vec<Token>) -> Node<String> {
  let next = peek_next_token(tokens);
  if is_opcode(next.get_value()) {
    return parse_opcode(tokens);
  }
  if next.get_type().is_directive() {
    return parse_directive(tokens);
  }
  let after_next = peek_two_ahead(tokens);
  if after_next.get_type() == &TokenType::Equal {
    return parse_assignment(tokens);
  }
  parse_label(tokens)
}

// <assignment> ::= <id> "=" <expression>
fn parse_assignment(tokens: &mut Vec<Token>) -> Node<String> {
  let id = get_next_token_checked(tokens, vec![TokenType::Identifier]);
  let _op = get_next_token_checked(tokens, vec![TokenType::Equal]);
  let val = parse_expression(tokens);
  let mut assignment = Node::new(NodeType::AssignmentStatement);
  assignment.add_data(id.get_value());
  assignment.add_child(val);
  assignment
}

// <directive> ::= <dir-segment> | <dir-other>
fn parse_directive(tokens: &mut Vec<Token>) -> Node<String> {
  let directive = get_next_token(tokens);
  let mut dir_statement = Node::new(NodeType::DirectiveStatement);
  let child = match directive.get_type() {
    TokenType::DirectiveSegment => parse_dir_segment(tokens, directive),
    _ => parse_dir_other(tokens),
  };
  dir_statement.add_child(child);
  dir_statement
}

// <dir-segment> ::= ".segment" <dir-seg-name>
fn parse_dir_segment(tokens: &mut Vec<Token>, directive: Token) -> Node<String> {
  if directive.get_value() != "segment" {
    panic!("Invalid segment token {}", directive.get_value());
  }
  let mut segment = Node::new(NodeType::DirectiveSegment);
  let name = get_next_token_checked(tokens, vec![TokenType::StringConst]);
  validate_dir_seg_name(&name);
  segment.add_data(name.get_value());
  segment
}

// <dir-seg-name> ::= <double-quote> <up-case-letter> { <up-case-letter> } <double-quote>
fn validate_dir_seg_name(token: &Token) {
  let val = token.get_value().to_ascii_uppercase();
  if !(val.len() > 1 && &val == token.get_value()) {
    panic!("Segment directive name is invalid: {}", token.get_value());
  }
}

// <dir-other> ::= <dir-name> { <dir-arg> }
fn parse_dir_other(tokens: &mut Vec<Token>) -> Node<String> {
  let dir_token = get_next_token(tokens);
  validate_dir_name(&dir_token);
  let mut directive = Node::new(NodeType::from_token_type(dir_token.get_type()));
  let mut next = peek_next_token(tokens);
  while !next.is_terminus() {
    let dir_arg = parse_dir_arg(tokens);
    directive.add_child(dir_arg);
    next = peek_next_token(tokens);
  }
  directive
}

// <dir-name> ::= "." <low-case-letter> { <low-case-letter> }
fn validate_dir_name(token: &Token) {
  let val = token.get_value().to_ascii_lowercase();
  if !(val.len() > 1 && &val == token.get_value()) {
    panic!("Directive name is invalid: {}", token.get_value());
  }
}

// <dir-arg> ::= <dir-string-arg> | <id> | <dir-value> { "," <dir-arg> }
fn parse_dir_arg(tokens: &mut Vec<Token>) -> Node<String> {
  let mut dir_args = Node::new(NodeType::DirArgs);
  let dir_arg = parse_dir_value(tokens);
  dir_args.add_child(dir_arg);
  let mut next = peek_next_token(tokens);
  while next.get_type() == &TokenType::Comma {
    let dir_arg = parse_dir_value(tokens);
    dir_args.add_child(dir_arg);
    next = peek_next_token(tokens);
  }
  dir_args
}

// <dir-value> ::= (<double-quote>|<single-quote>) <letter> { (<letter>|<symbol>) } (<double-quote>|<single-quote>)
fn parse_dir_value(tokens: &mut Vec<Token>) -> Node<String> {
  let mut string = Node::new(NodeType::String);
  let token = get_next_token(tokens);
  string.add_data(token.get_value());
  string
}

// <label> ::= <normal-label> | <local-label> | <unnamed-label>
fn parse_label(tokens: &mut Vec<Token>) -> Node<String> {
  let mut label_statement = Node::new(NodeType::LabelStatement);
  let next = peek_next_token(tokens);
  let child = match next.get_type() {
    TokenType::Identifier => parse_normal_label(tokens),
    TokenType::LocalLabel => parse_local_label(tokens),
    _ => parse_unnamed_label(tokens),
  };
  label_statement.add_child(child);
  label_statement
}

// <normal-label> ::= <id> ":"
fn parse_normal_label(tokens: &mut Vec<Token>) -> Node<String> {
  let mut normal_label = Node::new(NodeType::Label);
  let id = get_next_token_checked(tokens, vec![TokenType::Identifier]);
  get_next_token_checked(tokens, vec![TokenType::Colon]);
  normal_label.add_data(id.get_value());
  normal_label
}

// <local-label> ::= "@" <id> ":"
fn parse_local_label(tokens: &mut Vec<Token>) -> Node<String> {
  let mut local_label = Node::new(NodeType::LocalLabel);
  let id = get_next_token_checked(tokens, vec![TokenType::Identifier]);
  get_next_token_checked(tokens, vec![TokenType::Colon]);
  local_label.add_data(id.get_value());
  local_label
}

// <unnamed-label> ::= ":"
fn parse_unnamed_label(tokens: &mut Vec<Token>) -> Node<String> {
  let unnamed = Node::new(NodeType::LocalLabel);
  get_next_token_checked(tokens, vec![TokenType::Colon]);
  unnamed
}

fn parse_opcode(tokens: &mut Vec<Token>) -> Node<String> {
  todo!();
}

fn parse_expression(tokens: &mut Vec<Token>) -> Node<String> {
  todo!();
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
  peek(tokens, 0)
}

fn peek_two_ahead(tokens: &Vec<Token>) -> Token {
  peek(tokens, 1)
}

fn peek(tokens: &Vec<Token>, count: usize) -> Token {
  let option = tokens.get(count);
  match option {
    Some(token) => token.clone(),
    None => Token::new(String::from(""), TokenType::EndOfFile, 0, 0),
  }
}
