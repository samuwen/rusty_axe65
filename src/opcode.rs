use crate::token::Token;

pub fn is_opcode(t: &Token) -> bool {
  let value = t.get_value().to_ascii_lowercase();
  match value.as_ref() {
    "adc" | "and" | "asl" | "bcc" | "bcs" | "beq" | "bit" | "bmi" | "bne" | "bpl" | "brk"
    | "bvc" | "bvs" | "clc" | "cld" | "cli" | "clv" | "cmp" | "cpx" | "cpy" | "dec" | "dex"
    | "dey" | "eor" | "inc" | "inx" | "iny" | "jmp" | "jsr" | "lda" | "ldx" | "ldy" | "lsr"
    | "nop" | "ora" | "pha" | "php" | "pla" | "plp" | "rol" | "ror" | "rti" | "rts" | "sbc"
    | "sec" | "sed" | "sei" | "sta" | "stx" | "sty" | "tax" | "tay" | "tsx" | "txa" | "txs"
    | "tya" => true,
    _ => false,
  }
}

// Also includes accumulator
pub fn get_implied(code: &str) -> u8 {
  match code.to_ascii_lowercase().as_str() {
    "asl" => 0x0A,
    "brk" => 0x00,
    "clc" => 0x18,
    "cld" => 0xD8,
    "cli" => 0x58,
    "clv" => 0xB8,
    "dex" => 0xCA,
    "dey" => 0x88,
    "inx" => 0xE8,
    "iny" => 0xC8,
    "lsr" => 0x4A,
    "pha" => 0x48,
    "pla" => 0x68,
    "php" => 0x08,
    "plp" => 0x28,
    "nop" => 0xEA,
    "rol" => 0x2A,
    "ror" => 0x6A,
    "rti" => 0x40,
    "rts" => 0x60,
    "sec" => 0x38,
    "sed" => 0xF8,
    "sei" => 0x78,
    "tax" => 0xAA,
    "tay" => 0xA8,
    "tsx" => 0xBA,
    "txa" => 0x8A,
    "txs" => 0x9A,
    "tya" => 0x98,
    _ => panic!("{} is not an implied opcode", code),
  }
}

pub fn get_immediate(code: &str) -> u8 {
  match code.to_ascii_lowercase().as_str() {
    "adc" => 0x69,
    "and" => 0x29,
    "cmp" => 0xC9,
    "cpx" => 0xE0,
    "cpy" => 0xC0,
    "eor" => 0x49,
    "lda" => 0xA9,
    "ldx" => 0xA2,
    "ldy" => 0xA0,
    "ora" => 0x09,
    "sbc" => 0xE9,
    _ => panic!("{} does not have an immediate address mode", code),
  }
}

pub fn _get_absolute(code: &str) -> u8 {
  match code.to_ascii_lowercase().as_str() {
    "adc" => 0x6D,
    "and" => 0x2D,
    "asl" => 0x0E,
    "bit" => 0x2C,
    "cmp" => 0xCD,
    "cpx" => 0xEC,
    "cpy" => 0xCC,
    "dec" => 0xCE,
    "eor" => 0x4D,
    "inc" => 0xEE,
    "jmp" => 0x4C,
    "jsr" => 0x20,
    "lda" => 0xAD,
    "ldx" => 0xAE,
    "ldy" => 0xAC,
    "lsr" => 0x4E,
    "ora" => 0x0D,
    "rol" => 0x2E,
    "ror" => 0x6E,
    "sbc" => 0xED,
    "sta" => 0x8D,
    "stx" => 0x8E,
    "sty" => 0x8C,
    _ => panic!("{} does not have an absolute address mode", code),
  }
}

pub fn _get_absolute_x(code: &str) -> u8 {
  match code.to_ascii_lowercase().as_str() {
    "adc" => 0x7D,
    "and" => 0x3D,
    "asl" => 0x1E,
    "cmp" => 0xDD,
    "dec" => 0xDE,
    "eor" => 0x5D,
    "inc" => 0xFE,
    "lda" => 0xBD,
    "ldx" => 0xBE,
    "ldy" => 0xBC,
    "lsr" => 0x5E,
    "ora" => 0x1D,
    "rol" => 0x3E,
    "ror" => 0x7E,
    "sbc" => 0xFD,
    "sta" => 0x9D,
    _ => panic!("{} does not have an absolute x address mode", code),
  }
}

pub fn _get_absolute_y(code: &str) -> u8 {
  match code.to_ascii_lowercase().as_str() {
    "adc" => 0x79,
    "and" => 0x39,
    "cmp" => 0xD9,
    "eor" => 0x59,
    "lda" => 0xB9,
    "ora" => 0x19,
    "sbc" => 0xF9,
    "sta" => 0x99,
    _ => panic!("{} does not have an absolute y address mode", code),
  }
}

pub fn _get_zero_page(code: &str) -> u8 {
  match code.to_ascii_lowercase().as_str() {
    "adc" => 0x65,
    "and" => 0x25,
    "asl" => 0x06,
    "bit" => 0x24,
    "cmp" => 0xC5,
    "cpx" => 0xE4,
    "cpy" => 0xC4,
    "dec" => 0xC6,
    "eor" => 0x45,
    "inc" => 0xE6,
    "lda" => 0xA5,
    "ldx" => 0xA6,
    "ldy" => 0xA4,
    "lsr" => 0x46,
    "ora" => 0x05,
    "rol" => 0x26,
    "ror" => 0x36,
    "sbc" => 0xE5,
    "sta" => 0x85,
    "stx" => 0x86,
    "sty" => 0x84,
    _ => panic!("{} does not have a zero page address mode", code),
  }
}

pub fn _get_zero_page_x(code: &str) -> u8 {
  match code.to_ascii_lowercase().as_str() {
    "adc" => 0x75,
    "and" => 0x35,
    "asl" => 0x16,
    "cmp" => 0xD5,
    "dec" => 0xD6,
    "eor" => 0x55,
    "inc" => 0xF6,
    "lda" => 0xB5,
    "ldy" => 0xB4,
    "lsr" => 0x56,
    "ora" => 0x15,
    "rol" => 0x66,
    "ror" => 0x76,
    "sbc" => 0xF5,
    "sta" => 0x95,
    "sty" => 0x94,
    _ => panic!("{} does not have a zero page address mode", code),
  }
}

pub fn _get_zero_page_y(code: &str) -> u8 {
  match code.to_ascii_lowercase().as_str() {
    "ldx" => 0xB6,
    "stx" => 0x96,
    _ => panic!("{} does not have a zero page address mode", code),
  }
}

pub fn _get_relative(code: &str) -> u8 {
  match code.to_ascii_lowercase().as_str() {
    "bcc" => 0x90,
    "bcs" => 0xB0,
    "beq" => 0xF0,
    "bmi" => 0x30,
    "bne" => 0xD0,
    "bpl" => 0x10,
    "bvc" => 0x50,
    "bvs" => 0x70,
    _ => panic!("{} does not have a relative address mode", code),
  }
}

pub fn _get_indirect(code: &str) -> u8 {
  match code.to_ascii_lowercase().as_str() {
    "jmp" => 0x6C,
    _ => panic!("{} does not have an indirect address mode", code),
  }
}

pub fn _get_indirect_x(code: &str) -> u8 {
  match code.to_ascii_lowercase().as_str() {
    "adc" => 0x61,
    "and" => 0x21,
    "cmp" => 0xC1,
    "eor" => 0x41,
    "lda" => 0xA1,
    "ora" => 0x01,
    "sbc" => 0xE1,
    "sta" => 0x81,
    _ => panic!("{} does not have an indirect x address mode", code),
  }
}

pub fn _get_indirect_y(code: &str) -> u8 {
  match code.to_ascii_lowercase().as_str() {
    "adc" => 0x71,
    "and" => 0x31,
    "cmp" => 0xD1,
    "eor" => 0x51,
    "lda" => 0xB1,
    "ora" => 0x11,
    "sbc" => 0xF1,
    "sta" => 0x91,
    _ => panic!("{} does not have an indirect y address mode", code),
  }
}
