use crate::common::*;
use crate::lexer::lex;
use crate::token::{Token, TokenType};
use std::convert::TryInto;
use std::fs::write;

pub fn generate_config_data(config_file: &String) {
  let mut tokens = lex(config_file, false);
  let to_file = tokens.clone();
  let out: Vec<String> = to_file.iter().map(|t| format!("{}", t)).collect();
  write("src/out/config_lexed.out", out.join("\n")).unwrap();
  parse_config_file(&mut tokens);
}

fn parse_config_file(tokens: &mut Vec<Token>) {
  let token = get_next_token_checked(tokens, vec![TokenType::Identifier]);
  match token.get_value().to_ascii_uppercase().as_ref() {
    "MEMORY" => parse_memory_section(tokens),
    "SEGMENTS" => parse_segment_section(tokens),
    "SYMBOLS" => parse_symbol_section(tokens),
    "FEATURES" => parse_feature_section(tokens),
    _ => panic!(
      "Unrecognized configuration parameter: {}",
      token.get_value()
    ),
  }
}

fn parse_memory_section(tokens: &mut Vec<Token>) {
  let mut memory = Memory::new();
  get_next_token_checked(tokens, vec![TokenType::OCurly]);
  let mut next = peek_next_token(tokens);
  while next.get_type() != &TokenType::CCurly {
    parse_memory_entry(tokens, &mut memory);
    next = peek_next_token(tokens);
  }
  get_next_token_checked(tokens, vec![TokenType::CCurly]);
}

fn parse_memory_entry(tokens: &mut Vec<Token>, memory: &mut Memory) {
  let id = get_next_token_checked(tokens, vec![TokenType::Identifier]);
  let mut memory_entry = MemoryEntry::new(id.get_value());
  get_next_token_checked(tokens, vec![TokenType::Colon]);
  let mut next = peek_next_token(tokens);
  while next.get_type() != &TokenType::Comment {
    memory_entry = parse_mem_attributes(tokens, memory_entry);
    next = peek_next_token(tokens);
    if next.get_type() == &TokenType::Comma {
      get_next_token(tokens);
      next = peek_next_token(tokens);
    }
  }
  get_next_token_checked(tokens, vec![TokenType::Comment]);
  memory.add_entry(memory_entry.build());
}

fn parse_mem_attributes(
  tokens: &mut Vec<Token>,
  mem_entry: MemoryEntryBuilder,
) -> MemoryEntryBuilder {
  let attr_name = get_next_token_checked(tokens, vec![TokenType::Identifier]);
  get_next_token_checked(tokens, vec![TokenType::Equal]);
  let value = get_next_token_checked(
    tokens,
    vec![
      TokenType::String,
      TokenType::HexNumber,
      TokenType::BinNumber,
      TokenType::DecNumber,
      TokenType::Identifier,
      TokenType::StringConst,
    ],
  );
  match attr_name.get_value().as_ref() {
    "start" => {
      let num = convert_number(&value);
      match num {
        Ok(v) => mem_entry.start(v),
        Err(_) => error(&value),
      }
    }
    "size" => {
      let num = convert_number(&value);
      match num {
        Ok(v) => mem_entry.size(v),
        Err(_) => error(&value),
      }
    }
    "fillval" => {
      let num = convert_number(&value);
      match num {
        Ok(v) => mem_entry.fill_val(v.try_into().expect("Invalid value for fillval")),
        Err(_) => error(&value),
      }
    }
    "type" => mem_entry.mem_type(MemType::from_string(value.get_value())),
    "file" => match value.get_type() {
      TokenType::StringConst => mem_entry.file(value.get_value()),
      TokenType::BinNumber => {
        get_next_token_checked(tokens, vec![TokenType::Identifier]);
        mem_entry.file(&String::from("a.out"))
      }
      _ => panic!("Invalid memory file type {:?}", value.get_type()),
    },
    "define" => match value.get_type() == &TokenType::Identifier {
      true => mem_entry.define(match value.get_value().as_str() {
        "yes" => true,
        "no" => false,
        _ => panic!("Invalid memory define instruction {}", value.get_value()),
      }),
      false => panic!("Invalid memory define type {:?}", value.get_type()),
    },
    "fill" => match value.get_type() == &TokenType::Identifier {
      true => mem_entry.fill(match value.get_value().as_str() {
        "yes" => true,
        "no" => false,
        _ => panic!("Invalid memory fill instruction {}", value.get_value()),
      }),
      false => panic!("Invalid memory fill type {:?}", value.get_type()),
    },
    _ => panic!("the disco"),
  }
}

fn parse_segment_section(tokens: &mut Vec<Token>) {
  let mut memory = Memory::new();
  get_next_token_checked(tokens, vec![TokenType::OCurly]);
  let mut next = peek_next_token(tokens);
  while next.get_type() != &TokenType::CCurly {
    parse_memory_entry(tokens, &mut memory);
    next = peek_next_token(tokens);
  }
  get_next_token_checked(tokens, vec![TokenType::CCurly]);
}

fn parse_symbol_section(tokens: &mut Vec<Token>) {
  todo!();
}

fn parse_feature_section(tokens: &mut Vec<Token>) {
  todo!();
}

struct Memory {
  entries: Vec<MemoryEntry>,
}

impl Memory {
  fn new() -> Memory {
    Memory { entries: vec![] }
  }

  fn add_entry(&mut self, entry: MemoryEntry) {
    self.entries.push(entry);
  }
}

struct MemoryEntry {
  name: String,
  start: u16,
  size: u16,
  mem_type: Option<MemType>,
  file: Option<String>,
  define: Option<bool>,
  fill: Option<bool>,
  fill_val: Option<u8>,
}

impl MemoryEntry {
  fn new(name: &String) -> MemoryEntryBuilder {
    MemoryEntryBuilder {
      name: name.to_owned(),
      start: u16::MAX,
      size: u16::MAX,
      mem_type: None,
      file: None,
      define: None,
      fill: None,
      fill_val: None,
    }
  }
}

struct MemoryEntryBuilder {
  name: String,
  start: u16,
  size: u16,
  mem_type: Option<MemType>,
  file: Option<String>,
  define: Option<bool>,
  fill: Option<bool>,
  fill_val: Option<u8>,
}

impl MemoryEntryBuilder {
  fn start(mut self, start: u16) -> Self {
    self.start = start;
    self
  }

  fn size(mut self, size: u16) -> Self {
    self.size = size;
    self
  }

  fn mem_type(mut self, mem_type: MemType) -> Self {
    self.mem_type = Some(mem_type);
    self
  }

  fn file(mut self, file: &String) -> Self {
    self.file = Some(file.to_owned());
    self
  }

  fn define(mut self, define: bool) -> Self {
    self.define = Some(define);
    self
  }

  fn fill(mut self, fill: bool) -> Self {
    self.fill = Some(fill);
    self
  }

  fn fill_val(mut self, fill_val: u8) -> Self {
    self.fill_val = Some(fill_val);
    self
  }

  fn build(self) -> MemoryEntry {
    if self.start == u16::MAX {
      panic!("Memory entry {} does not have start attribute", self.name);
    }
    if self.size == u16::MAX {
      panic!("Memory entry {} does not have size attribute", self.name);
    }
    MemoryEntry {
      name: self.name,
      start: self.start,
      size: self.size,
      mem_type: self.mem_type,
      file: self.file,
      define: self.define,
      fill: self.fill,
      fill_val: self.fill_val,
    }
  }
}

enum MemType {
  Ro,
  Rw,
}

impl MemType {
  fn from_string(string: &String) -> MemType {
    match string.to_ascii_lowercase().as_ref() {
      "ro" => MemType::Ro,
      "rw" => MemType::Rw,
      _ => panic!("Invalid memory type: {}", string),
    }
  }
}

struct Segment {
  entries: Vec<SegmentEntry>,
}

impl Segment {
  fn new() -> Segment {
    Segment { entries: vec![] }
  }
}

struct SegmentEntry {
  load: String,
  seg_type: SegType,
  define: Option<bool>,
  align: Option<u16>,
  start: Option<u16>,
  run: Option<String>,
  offset: Option<u16>,
  fill_val: Option<u8>,
  align_load: Option<u16>,
}

enum SegType {
  Ro,
  Rw,
  Bss,
  Zp,
  Overwrite,
}
