use crate::common::*;
use crate::lexer::lex;
use crate::token::{Token, TokenType};
use std::fs::write;

pub fn generate_config_data(config_file: &String) -> Configuration {
  let mut tokens = lex(config_file, false);
  let to_file = tokens.clone();
  let out: Vec<String> = to_file.iter().map(|t| format!("{}", t)).collect();
  write("src/out/config_lexed.out", out.join("\n")).unwrap();
  parse_config_file(&mut tokens)
}

fn parse_config_file(tokens: &mut Vec<Token>) -> Configuration {
  let mut config = Configuration::new();
  let mut next = peek_next_token(tokens);
  while next.get_type() != &TokenType::EndOfFile {
    config = match next.get_value().to_ascii_uppercase().as_ref() {
      "MEMORY" => parse_memory_section(tokens, config),
      "SEGMENTS" => parse_segment_section(tokens, config),
      "SYMBOLS" => parse_symbol_section(tokens, config),
      "FEATURES" => parse_feature_section(tokens, config),
      _ => panic!("Unrecognized configuration parameter: {}", next.get_value()),
    };
    next = peek_next_token(tokens);
  }
  config.build()
}

fn parse_memory_section(tokens: &mut Vec<Token>, config: ConfigBuilder) -> ConfigBuilder {
  get_next_token_checked(tokens, vec![TokenType::Identifier]);
  let mut memory = Memory::new();
  get_next_token_checked(tokens, vec![TokenType::OCurly]);
  let mut next = peek_next_token(tokens);
  while next.get_type() != &TokenType::CCurly {
    parse_memory_entry(tokens, &mut memory);
    next = peek_next_token(tokens);
  }
  get_next_token_checked(tokens, vec![TokenType::CCurly]);
  config.memory(memory)
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
    "start" => add_number(value, MemoryEntryBuilder::start, mem_entry),
    "size" => add_number(value, MemoryEntryBuilder::size, mem_entry),
    "fillval" => add_u8(value, MemoryEntryBuilder::fill_val, mem_entry),
    "type" => mem_entry.mem_type(MemType::from_string(value.get_value())),
    "file" => match value.get_type() {
      TokenType::StringConst => mem_entry.file(value.get_value()),
      TokenType::BinNumber => {
        get_next_token_checked(tokens, vec![TokenType::Identifier]);
        mem_entry.file(&String::from("a.out"))
      }
      _ => panic!("Invalid memory file type {:?}", value.get_type()),
    },
    "define" => add_bool(value, MemoryEntryBuilder::define, mem_entry),
    "fill" => add_bool(value, MemoryEntryBuilder::fill, mem_entry),
    _ => panic!("the disco"),
  }
}

fn parse_segment_section(tokens: &mut Vec<Token>, config: ConfigBuilder) -> ConfigBuilder {
  get_next_token_checked(tokens, vec![TokenType::Identifier]);
  let mut segment = Segment::new();
  get_next_token_checked(tokens, vec![TokenType::OCurly]);
  let mut next = peek_next_token(tokens);
  while next.get_type() != &TokenType::CCurly {
    parse_segment_entry(tokens, &mut segment);
    next = peek_next_token(tokens);
  }
  get_next_token_checked(tokens, vec![TokenType::CCurly]);
  config.segments(segment)
}

fn parse_segment_entry(tokens: &mut Vec<Token>, segment: &mut Segment) {
  let id = get_next_token_checked(tokens, vec![TokenType::Identifier]);
  let mut segment_entry = SegmentEntry::new(id.get_value());
  get_next_token_checked(tokens, vec![TokenType::Colon]);
  let mut next = peek_next_token(tokens);
  while next.get_type() != &TokenType::Comment {
    segment_entry = parse_seg_attributes(tokens, segment_entry);
    next = peek_next_token(tokens);
    if next.get_type() == &TokenType::Comma {
      get_next_token(tokens);
      next = peek_next_token(tokens);
    }
  }
  get_next_token_checked(tokens, vec![TokenType::Comment]);
  segment.add_entry(segment_entry.build());
}

fn parse_seg_attributes(
  tokens: &mut Vec<Token>,
  seg_entry: SegmentEntryBuilder,
) -> SegmentEntryBuilder {
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
    "load" => seg_entry.load(value.get_value()),
    "type" => seg_entry.seg_type(SegType::from_str(value.get_value())),
    "define" => add_bool(value, SegmentEntryBuilder::define, seg_entry),
    "align" => add_number(value, SegmentEntryBuilder::align, seg_entry),
    "start" => add_number(value, SegmentEntryBuilder::start, seg_entry),
    "run" => seg_entry.run(value.get_value()),
    "offset" => add_number(value, SegmentEntryBuilder::start, seg_entry),
    "fillval" => add_u8(value, SegmentEntryBuilder::fill_val, seg_entry),
    _ => panic!("Invalid attribute name {}", attr_name.get_value()),
  }
}

fn parse_symbol_section(_tokens: &mut Vec<Token>, _config: ConfigBuilder) -> ConfigBuilder {
  todo!();
}

fn parse_feature_section(_tokens: &mut Vec<Token>, _config: ConfigBuilder) -> ConfigBuilder {
  todo!();
}

pub struct Configuration {
  memory: Memory,
  segments: Segment,
  _symbols: Option<bool>,  // NYI
  _features: Option<bool>, // NYI
}

impl Configuration {
  fn new() -> ConfigBuilder {
    ConfigBuilder {
      memory: None,
      segments: None,
      _symbols: None,
      _features: None,
    }
  }

  pub fn find_segment_by_name(&self, name: &String) -> Option<&SegmentEntry> {
    self.segments.find_segment_by_name(name)
  }

  pub fn find_memory_by_name(&self, name: &String) -> Option<&MemoryEntry> {
    self.memory.find_memory_by_name(name)
  }

  pub fn get_segment_start(&self, name: &String) -> u16 {
    let segment = self.find_segment_by_name(name).unwrap();
    match segment.get_start() {
      Some(start) => start,
      None => {
        let memory_entry = self.find_memory_by_name(segment.get_load()).unwrap();
        memory_entry.get_start()
      }
    }
  }
}

struct ConfigBuilder {
  memory: Option<Memory>,
  segments: Option<Segment>,
  _symbols: Option<bool>,  // NYI
  _features: Option<bool>, // NYI
}

impl ConfigBuilder {
  fn memory(mut self, memory: Memory) -> Self {
    self.memory = Some(memory);
    self
  }

  fn segments(mut self, segments: Segment) -> Self {
    self.segments = Some(segments);
    self
  }

  fn build(self) -> Configuration {
    let memory = match self.memory {
      Some(mem) => mem,
      None => panic!("No memory configuration specified"),
    };
    let segments = match self.segments {
      Some(seg) => seg,
      None => panic!("No segment configuration specified"),
    };
    Configuration {
      memory,
      segments,
      _symbols: None,
      _features: None,
    }
  }
}

struct Memory {
  entries: Vec<MemoryEntry>,
}

impl ConfigSection for Memory {}

impl Memory {
  fn new() -> Memory {
    Memory { entries: vec![] }
  }

  fn add_entry(&mut self, entry: MemoryEntry) {
    self.entries.push(entry);
  }

  fn find_memory_by_name(&self, name: &String) -> Option<&MemoryEntry> {
    self
      .entries
      .iter()
      .find(|e| e.name.to_ascii_uppercase() == name.to_ascii_uppercase())
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

impl ConfigEntry for MemoryEntry {}

impl MemoryEntry {
  fn new(name: &String) -> MemoryEntryBuilder {
    MemoryEntryBuilder {
      name: name.to_owned(),
      start: None,
      size: None,
      mem_type: None,
      file: None,
      define: None,
      fill: None,
      fill_val: None,
    }
  }

  fn get_start(&self) -> u16 {
    self.start
  }
}

struct MemoryEntryBuilder {
  name: String,
  start: Option<u16>,
  size: Option<u16>,
  mem_type: Option<MemType>,
  file: Option<String>,
  define: Option<bool>,
  fill: Option<bool>,
  fill_val: Option<u8>,
}

impl ConfigEntryBuilder for MemoryEntryBuilder {}

impl MemoryEntryBuilder {
  fn start(mut self, start: u16) -> Self {
    self.start = Some(start);
    self
  }

  fn size(mut self, size: u16) -> Self {
    self.size = Some(size);
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
    let start = match self.start {
      Some(st) => st,
      None => panic!("Memory entry {} does not have start attribute", self.name),
    };
    let size = match self.size {
      Some(st) => st,
      None => panic!("Memory entry {} does not have size attribute", self.name),
    };
    MemoryEntry {
      name: self.name,
      start: start,
      size: size,
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

impl ConfigSection for Segment {}

impl Segment {
  fn new() -> Segment {
    Segment { entries: vec![] }
  }

  fn add_entry(&mut self, entry: SegmentEntry) {
    self.entries.push(entry);
  }

  fn find_segment_by_name(&self, name: &String) -> Option<&SegmentEntry> {
    self
      .entries
      .iter()
      .find(|e| e.name.to_ascii_uppercase() == name.to_ascii_uppercase())
  }
}

pub struct SegmentEntry {
  name: String,
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

impl ConfigEntry for SegmentEntry {}

impl SegmentEntry {
  fn new(name: &String) -> SegmentEntryBuilder {
    SegmentEntryBuilder {
      name: name.to_owned(),
      load: None,
      seg_type: None,
      define: None,
      align: None,
      start: None,
      run: None,
      offset: None,
      fill_val: None,
      align_load: None,
    }
  }

  pub fn get_type(&self) -> &SegType {
    &self.seg_type
  }

  fn get_load(&self) -> &String {
    &self.load
  }

  fn get_start(&self) -> Option<u16> {
    self.start
  }
}

struct SegmentEntryBuilder {
  name: String,
  load: Option<String>,
  seg_type: Option<SegType>,
  define: Option<bool>,
  align: Option<u16>,
  start: Option<u16>,
  run: Option<String>,
  offset: Option<u16>,
  fill_val: Option<u8>,
  align_load: Option<u16>,
}

impl ConfigEntryBuilder for SegmentEntryBuilder {}

impl SegmentEntryBuilder {
  fn load(mut self, load: &String) -> SegmentEntryBuilder {
    self.load = Some(load.to_owned());
    self
  }

  fn seg_type(mut self, seg_type: SegType) -> SegmentEntryBuilder {
    self.seg_type = Some(seg_type);
    self
  }

  fn define(mut self, define: bool) -> SegmentEntryBuilder {
    self.define = Some(define);
    self
  }

  fn align(mut self, align: u16) -> SegmentEntryBuilder {
    self.align = Some(align);
    self
  }

  fn start(mut self, start: u16) -> SegmentEntryBuilder {
    self.start = Some(start);
    self
  }

  fn run(mut self, run: &String) -> SegmentEntryBuilder {
    self.run = Some(run.to_owned());
    self
  }

  fn offset(mut self, offset: u16) -> SegmentEntryBuilder {
    self.offset = Some(offset);
    self
  }

  fn fill_val(mut self, fill_val: u8) -> SegmentEntryBuilder {
    self.fill_val = Some(fill_val);
    self
  }

  fn align_load(mut self, align_load: u16) -> SegmentEntryBuilder {
    self.align_load = Some(align_load);
    self
  }

  fn build(self) -> SegmentEntry {
    let load = match self.load {
      Some(l) => l,
      None => panic!("Segment entry missing load parameter"),
    };
    let seg_type = match self.seg_type {
      Some(s) => s,
      None => panic!("Segment entry missing type parameter"),
    };
    SegmentEntry {
      name: self.name,
      load: load,
      seg_type: seg_type,
      define: self.define,
      align: self.align,
      start: self.start,
      run: self.run,
      offset: self.offset,
      fill_val: self.fill_val,
      align_load: self.align_load,
    }
  }
}

pub enum SegType {
  Ro,
  Rw,
  Bss,
  Zp,
  Overwrite,
}

impl SegType {
  fn from_str(value: &String) -> SegType {
    match value.to_ascii_lowercase().as_ref() {
      "ro" => SegType::Ro,
      "rw" => SegType::Rw,
      "bss" => SegType::Bss,
      "zp" => SegType::Zp,
      "overwrite" => SegType::Overwrite,
      _ => panic!("Invalid segment type: {}", value),
    }
  }
}

fn add_number<T: ConfigEntryBuilder>(value: Token, f: fn(T, u16) -> T, entry: T) -> T {
  let num = convert_number(&value);
  match num {
    Ok(v) => f(entry, v),
    Err(_) => error(&value),
  }
}

fn add_u8<T: ConfigEntryBuilder>(value: Token, f: fn(T, u8) -> T, entry: T) -> T {
  let num = convert_number(&value);
  match num {
    Ok(v) => f(entry, v as u8),
    Err(_) => error(&value),
  }
}

fn add_bool<T: ConfigEntryBuilder>(value: Token, f: fn(T, bool) -> T, entry: T) -> T {
  match value.get_type() == &TokenType::Identifier {
    true => f(
      entry,
      match value.get_value().as_str() {
        "yes" => true,
        "no" => false,
        _ => panic!("Invalid boolean instruction {}", value.get_value()),
      },
    ),
    false => panic!("Invalid boolean type {:?}", value.get_type()),
  }
}

trait ConfigSection {}
trait ConfigEntry {}
trait ConfigEntryBuilder {}
