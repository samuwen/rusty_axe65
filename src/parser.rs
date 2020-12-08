use crate::common::*;
use crate::node::{Node, NodeType};
use crate::opcode::*;
use crate::token::{Token, TokenType};

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
  let directive = peek_next_token(tokens);
  let mut dir_statement = Node::new(NodeType::DirectiveStatement);
  let child = match directive.get_type() {
    TokenType::DirectiveSegment => parse_dir_segment(tokens),
    _ => parse_dir_other(tokens),
  };
  dir_statement.add_child(child);
  dir_statement
}

// <dir-segment> ::= ".segment" <dir-seg-name>
fn parse_dir_segment(tokens: &mut Vec<Token>) -> Node<String> {
  let directive = get_next_token(tokens);
  if directive.get_value() != "segment" {
    error(&directive);
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
    error(token);
  }
}

// <dir-other> ::= <dir-name> { <dir-arg> }
fn parse_dir_other(tokens: &mut Vec<Token>) -> Node<String> {
  let dir_token = get_next_token(tokens);
  validate_dir_name(&dir_token);
  let mut directive = Node::new(NodeType::from_token_type(dir_token.get_type()));
  let dir_args = parse_dir_args(tokens);
  directive.add_child(dir_args);
  directive
}

// <dir-name> ::= "." <low-case-letter> { <low-case-letter> }
fn validate_dir_name(token: &Token) {
  let val = token.get_value().to_ascii_lowercase();
  if !(val.len() > 1 && &val == token.get_value()) {
    error(token);
  }
}

// <dir-arg> ::= (<string-const>|<expression>) { "," <dir-arg> }
fn parse_dir_args(tokens: &mut Vec<Token>) -> Node<String> {
  let mut dir_args = Node::new(NodeType::DirArgs);
  let next = peek_next_token(tokens);
  let dir_arg = match next.get_type() {
    TokenType::StringConst => parse_string_const(tokens),
    _ => parse_expression(tokens),
  };
  dir_args.add_child(dir_arg);
  let mut next = peek_next_token(tokens);
  while next.get_type() == &TokenType::Comma {
    get_next_token(tokens);
    let token = peek_next_token(tokens);
    let dir_arg = match token.get_type() {
      TokenType::StringConst => parse_string_const(tokens),
      _ => parse_expression(tokens),
    };
    dir_args.add_child(dir_arg);
    next = peek_next_token(tokens);
  }
  dir_args
}

// <string-const> ::= <dir-string-arg> | <dir-value>
fn parse_string_const(tokens: &mut Vec<Token>) -> Node<String> {
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
  let id = get_next_token_checked(tokens, vec![TokenType::LocalLabel]);
  get_next_token_checked(tokens, vec![TokenType::Colon]);
  local_label.add_data(id.get_value());
  local_label
}

// <unnamed-label> ::= ":"
fn parse_unnamed_label(tokens: &mut Vec<Token>) -> Node<String> {
  let unnamed = Node::new(NodeType::UnnamedLabel);
  get_next_token_checked(tokens, vec![TokenType::Colon]);
  unnamed
}

// <opcode> ::= <accumulator-mode> | <immediate-mode> | <direct-memory-mode> | <indirect-memory-mode>
fn parse_opcode(tokens: &mut Vec<Token>) -> Node<String> {
  let mut op_node = Node::new(NodeType::OpcodeStatement);
  let next = peek_next_token(tokens);
  match is_accumulator(next.get_value()) {
    true => {
      op_node.add_child(parse_accumulator(tokens));
    }
    false => {
      let after_next = peek_two_ahead(tokens);
      let child_op_node = match after_next.get_type() {
        TokenType::Hash => parse_immediate(tokens),
        TokenType::OParen => parse_indirect(tokens),
        _ => parse_direct(tokens),
      };
      op_node.add_child(child_op_node);
    }
  }
  op_node
}

// <accumulator-mode> ::= <op-id>
fn parse_accumulator(tokens: &mut Vec<Token>) -> Node<String> {
  let mut acc_node = Node::new(NodeType::AccumulatorMode);
  let code = get_next_token_checked(tokens, vec![TokenType::Opcode]);
  acc_node.add_data(code.get_value());
  acc_node
}

// <immediate-mode> ::= <op-id> "#" <expression>
fn parse_immediate(tokens: &mut Vec<Token>) -> Node<String> {
  let mut imm_node = Node::new(NodeType::ImmediateMode);
  let code = get_next_token_checked(tokens, vec![TokenType::Opcode]);
  imm_node.add_data(code.get_value());
  get_next_token_checked(tokens, vec![TokenType::Hash]);
  let expression = parse_expression(tokens);
  imm_node.add_child(expression);
  imm_node
}

// <direct-memory-mode> ::= <op-id> <expression> { "," <register> }
fn parse_direct(tokens: &mut Vec<Token>) -> Node<String> {
  let code = get_next_token_checked(tokens, vec![TokenType::Opcode]);
  let expression = parse_expression(tokens);
  let next = peek_next_token(tokens);
  let mut dir_node = Node::new(match next.get_type() {
    TokenType::Comma => {
      get_next_token(tokens);
      let reg = get_next_token(tokens);
      match reg.get_type() {
        TokenType::XRegister => NodeType::DirectRegXMode,
        TokenType::YRegister => NodeType::DirectRegYMode,
        _ => error(&reg),
      }
    }
    _ => NodeType::DirectMode,
  });
  dir_node.add_data(code.get_value());
  dir_node.add_child(expression);
  dir_node
}

// <indirect-memory-mode> ::= <indirect-x> | <indirect-y>
fn parse_indirect(tokens: &mut Vec<Token>) -> Node<String> {
  let code = get_next_token_checked(tokens, vec![TokenType::Opcode]);
  get_next_token_checked(tokens, vec![TokenType::OParen]);
  let expression = parse_expression(tokens);
  let disambiguator = get_next_token_checked(tokens, vec![TokenType::Comma, TokenType::CParen]);
  let mut ind_node = match disambiguator.get_type() {
    TokenType::Comma => {
      get_next_token_checked(tokens, vec![TokenType::XRegister]);
      get_next_token_checked(tokens, vec![TokenType::CParen]);
      Node::new(NodeType::IndirectXMode)
    }
    _ => {
      get_next_token_checked(tokens, vec![TokenType::Comma]);
      get_next_token_checked(tokens, vec![TokenType::YRegister]);
      Node::new(NodeType::IndirectYMode)
    }
  };
  ind_node.add_data(code.get_value());
  ind_node.add_child(expression);
  ind_node
}

// <expression> ::= "!" <expression> | <bool-not-exp>
fn parse_expression(tokens: &mut Vec<Token>) -> Node<String> {
  parse_generic_un_exp(
    tokens,
    parse_expression,
    parse_bool_not_exp,
    Token::is_prec_level_seven,
  )
}

// <bool-not-exp> ::= <bool-or-exp> { ("||"|"OR") <bool-or-exp> }
fn parse_bool_not_exp(tokens: &mut Vec<Token>) -> Node<String> {
  parse_bin_exp(tokens, parse_bool_or_exp, Token::is_prec_level_six)
}

// <bool-or-exp> ::= <bool-xor-and-exp> { ("&&"|"XOR"|"AND") <bool-xor-and-exp> }
fn parse_bool_or_exp(tokens: &mut Vec<Token>) -> Node<String> {
  parse_bin_exp(tokens, parse_bool_xor_and_exp, Token::is_prec_level_five)
}

// <bool-xor-and-exp> ::= <relational-exp> { ("="|"<>"|"<"|">"|"<="|">=") <relational-exp> }
fn parse_bool_xor_and_exp(tokens: &mut Vec<Token>) -> Node<String> {
  parse_bin_exp(tokens, parse_relational_exp, Token::is_prec_level_four)
}

// <relational-exp> ::= <binary-add-sub-exp> { ("+"|"-"|"|"|"BITOR") <binary-add-sub-exp> }
fn parse_relational_exp(tokens: &mut Vec<Token>) -> Node<String> {
  parse_bin_exp(tokens, parse_binary_add_sub_exp, Token::is_prec_level_three)
}

// <binary-add-sub-exp> ::= <bitwise-mul-div-exp> { ("_"|"/"|"<<"|">>"|"^"|"&"|"MOD"|"BITAND"|"BITXOR"|"SHL"|"SHR") <bitwise-mul-div-exp> }
fn parse_binary_add_sub_exp(tokens: &mut Vec<Token>) -> Node<String> {
  parse_bin_exp(tokens, parse_bitwise_mul_div_exp, Token::is_prec_level_two)
}

// <bitwise-mul-div-exp> ::= <unary-op> <bitwise-mul-div-exp> | <unary-exp>
fn parse_bitwise_mul_div_exp(tokens: &mut Vec<Token>) -> Node<String> {
  parse_generic_un_exp(
    tokens,
    parse_bitwise_mul_div_exp,
    parse_unary_exp,
    Token::is_prec_level_one,
  )
}

// <unary-exp> ::= <built-in-string-function> <unary-exp> | <factor>
fn parse_unary_exp(tokens: &mut Vec<Token>) -> Node<String> {
  parse_generic_un_exp(
    tokens,
    parse_unary_exp,
    parse_factor,
    Token::is_prec_level_zero,
  )
}

// <factor> ::= "(" <expression> ")" | <id> | <number>
fn parse_factor(tokens: &mut Vec<Token>) -> Node<String> {
  let token = peek_next_token(tokens);
  match token.get_type() {
    TokenType::OParen => {
      get_next_token_checked(tokens, vec![TokenType::OParen]);
      let exp = parse_expression(tokens);
      get_next_token_checked(tokens, vec![TokenType::CParen]);
      exp
    }
    TokenType::Identifier | TokenType::LocalLabel => parse_variable(tokens),
    TokenType::BinNumber | TokenType::HexNumber | TokenType::DecNumber => parse_number(tokens),
    TokenType::ULabel => parse_ulabel(tokens),
    _ => error(&token),
  }
}

fn parse_variable(tokens: &mut Vec<Token>) -> Node<String> {
  let token = get_next_token(tokens);
  let mut node = Node::new(NodeType::Variable);
  node.add_data(token.get_value());
  node
}

fn parse_ulabel(tokens: &mut Vec<Token>) -> Node<String> {
  let token = get_next_token(tokens);
  let mut node = Node::new(NodeType::LabelJump);
  node.add_data(token.get_value());
  node
}

// Take hex/bin/dec number and return it without control chars as decimal number
fn parse_number(tokens: &mut Vec<Token>) -> Node<String> {
  let token = get_next_token(tokens);
  let num = convert_number(&token);
  match num {
    Ok(val) => {
      let mut node = Node::new(NodeType::Number);
      node.add_data(&val.to_string());
      node
    }
    Err(_) => error(&token),
  }
}

fn parse_bin_exp<F: Fn(&mut Vec<Token>) -> Node<String>, N: Fn(&Token) -> bool>(
  tokens: &mut Vec<Token>,
  next_exp: F,
  valid_token: N,
) -> Node<String> {
  let mut expression = next_exp(tokens);
  let mut next = peek_next_token(tokens);
  while valid_token(&next) {
    let op = get_next_token(tokens);
    let mut node = Node::new(NodeType::BinaryOp);
    node.add_data(op.get_value());
    let next_expression = next_exp(tokens);
    node.add_child(expression);
    node.add_child(next_expression);
    expression = node;
    next = peek_next_token(tokens);
  }
  expression
}

fn parse_generic_un_exp<
  F: Fn(&mut Vec<Token>) -> Node<String>,
  G: Fn(&mut Vec<Token>) -> Node<String>,
  N: Fn(&Token) -> bool,
>(
  tokens: &mut Vec<Token>,
  next_exp: F,
  final_exp: G,
  valid_token: N,
) -> Node<String> {
  let next = peek_next_token(tokens);
  match valid_token(&next) {
    true => {
      let token = get_next_token(tokens);
      let mut node = Node::new(NodeType::UnaryOp);
      node.add_data(token.get_value());
      let expression = next_exp(tokens);
      node.add_child(expression);
      node
    }
    false => final_exp(tokens),
  }
}
