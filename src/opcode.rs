use crate::token::Token;

pub fn is_opcode(t: &Token) -> bool {
  match t.get_value().as_ref() {
    "brk" | "cld" | "inx" | "sei" | "txs" => true,
    _ => false,
  }
}

pub fn get_immediate(name: &str) -> String {
  let num = match name {
    "brk" => 0x00,
    "cld" => 0xD8,
    "inx" => 0xE8,
    "sei" => 0x78,
    "txs" => 0x9A,
    _ => panic!("Unknown immediate opcode: {}", name),
  };
  num.to_string()
}
