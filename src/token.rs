struct Token {
  val: String,
  t_type: TokenType,
  start: usize,
}

enum TokenType {
  OParen,
}
