use crate::token::Token;

pub fn is_opcode(t: &Token) -> bool {
  match t.get_value().as_ref() {
    "brk" | "sei" => true,
    _ => false,
  }
}

pub fn get_immediate(t: &Token) -> String {
  let num = match t.get_value().as_ref() {
    "brk" => 0x00,
    "sei" => 0x78,
    _ => panic!("Unknown immediate opcode: {}", t.get_value()),
  };
  num.to_string()
}
