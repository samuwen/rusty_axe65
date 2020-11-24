use std::cmp::Ordering;
use std::fmt;

pub struct Token {
  val: String,
  t_type: TokenType,
  start: usize,
  end: usize,
}

impl Token {
  pub fn new(val: String, t: &TokenType, start: usize, end: usize) -> Token {
    Token {
      val: val,
      t_type: t.clone(),
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

impl PartialOrd for Token {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Token {
  fn cmp(&self, other: &Self) -> Ordering {
    self.start.cmp(&other.start)
  }
}

impl Eq for Token {}

impl PartialEq for Token {
  fn eq(&self, other: &Self) -> bool {
    self.start == other.start && self.val == other.val
  }
}

#[derive(Clone, Debug)]
pub enum TokenType {
  Directive,
  Identifier,
  Label,
  Number,
  Comma,
  XRegister,
  YRegister,
}
