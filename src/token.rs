use std::fmt;

pub struct Token {
  val: String,
  t_type: TokenType,
  start: usize,
  end: usize,
}

impl Token {
  pub fn new(val: &String, t: TokenType, start: usize, end: usize) -> Token {
    Token {
      val: val.to_owned(),
      t_type: t,
      start: start,
      end: end,
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "type: {:?}, text: {}, start: {}, end: {}",
      self.t_type, self.val, self.start, self.end
    )
  }
}

#[derive(Debug)]
pub enum TokenType {
  Thingy,
}
