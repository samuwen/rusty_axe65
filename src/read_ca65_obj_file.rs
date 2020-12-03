// Intended to be a helper to understand what's in an obj file from ca65

use std::collections::HashMap;
use std::fs::read;

pub fn parse_file(path: String) {
  let mut bytes = read_file(path);
  read_file_header(&mut bytes);
  read_str_pool(&mut bytes);
}

fn read_file(path: String) -> Vec<u8> {
  read(path).expect("File not found")
}

fn read_file_header(bytes: &mut Vec<u8>) -> ObjectFileHeader {
  let magic = read_32(bytes);
  let version = read_16(bytes);
  if magic != 0x616E7A55 || version != 0x0011 {
    panic!("Not a valid object file");
  }
  ObjectFileHeader::new(bytes)
}

fn read_str_pool(bytes: &mut Vec<u8>) {
  let string_count = read_var(bytes);
  let mut strings = vec![];
  for _ in 0..string_count {
    strings.push(read_str(bytes));
  }
}

fn read_var(bytes: &mut Vec<u8>) -> usize {
  let mut character = 0xFF;
  let mut variable = 0;
  let mut shift = 0;
  while (character & 0x80) > 0 {
    character = read_8(bytes);
    variable |= ((character & 0x7f) as usize) << shift;
    shift += 7;
  }
  variable
}

fn read_str(bytes: &mut Vec<u8>) -> u8 {
  let len = read_var(bytes);
  let mut buf = StringBuffer::new(len as usize, 0, 0);
  read_data(bytes, &mut buf, len);
  let id = get_str_buf_id(&mut buf);
  0
}
/*{
  /* Insert it into the string pool and remember the id */
  Id = GetStrBufId(&Buf);

  /* Free the memory buffer */
  SB_Done(&Buf);

  /* Return the string id */
  return Id;
}*/

fn read_data(bytes: &mut Vec<u8>, data: &mut StringBuffer, size: usize) {
  if size > 0 {
    if fread(data.get_buffer(), 1, size, bytes) != size {
      panic!("Read error in file");
    }
  }
}

fn get_str_buf_id(ptr: &mut StringBuffer) -> usize {
  let mut pool = StringPool::new();
  string_pool_add(&mut pool, ptr)
}

fn string_pool_add(pool: &mut StringPool, ptr: &mut StringBuffer) -> usize {
  let string = ptr.get_buf_as_string();
  let hi = "hi";
  0
}

fn fread(ptr: &mut Vec<char>, size: usize, num_eles: usize, bytes: &mut Vec<u8>) -> usize {
  let mut count = 0;
  for _ in 0..num_eles {
    for _ in 0..size {
      count += 1;
      ptr.push(bytes.remove(0) as char);
    }
  }
  count
}

fn read_32(bytes: &mut Vec<u8>) -> u32 {
  let lo = read_16(bytes) as u32;
  let hi = read_16(bytes) as u32;
  (hi << 16) | lo
}

fn read_16(bytes: &mut Vec<u8>) -> u16 {
  let lo = read_8(bytes);
  let hi = read_8(bytes);
  u16::from_le_bytes([lo, hi])
}

fn read_8(bytes: &mut Vec<u8>) -> u8 {
  bytes.remove(0)
}

struct ObjectFileHeader {
  flags: u16,
  option_offs: u32,
  option_size: u32,
  file_offs: u32,
  file_size: u32,
  seg_offs: u32,
  seg_size: u32,
  import_offs: u32,
  import_size: u32,
  export_offs: u32,
  export_size: u32,
  dbg_sym_offs: u32,
  dbg_sym_size: u32,
  line_info_offs: u32,
  line_info_size: u32,
  str_pool_offs: u32,
  str_pool_size: u32,
  assert_offs: u32,
  assert_size: u32,
  scope_offs: u32,
  scope_size: u32,
  span_offs: u32,
  span_size: u32,
}

impl ObjectFileHeader {
  fn new(bytes: &mut Vec<u8>) -> ObjectFileHeader {
    ObjectFileHeader {
      flags: read_16(bytes),
      option_offs: read_32(bytes),
      option_size: read_32(bytes),
      file_offs: read_32(bytes),
      file_size: read_32(bytes),
      seg_offs: read_32(bytes),
      seg_size: read_32(bytes),
      import_offs: read_32(bytes),
      import_size: read_32(bytes),
      export_offs: read_32(bytes),
      export_size: read_32(bytes),
      dbg_sym_offs: read_32(bytes),
      dbg_sym_size: read_32(bytes),
      line_info_offs: read_32(bytes),
      line_info_size: read_32(bytes),
      str_pool_offs: read_32(bytes),
      str_pool_size: read_32(bytes),
      assert_offs: read_32(bytes),
      assert_size: read_32(bytes),
      scope_offs: read_32(bytes),
      scope_size: read_32(bytes),
      span_offs: read_32(bytes),
      span_size: read_32(bytes),
    }
  }
}

struct StringBuffer {
  buf: Vec<char>,
  len: usize,
  index: usize,
  allocated: usize,
}

impl StringBuffer {
  fn new(len: usize, index: usize, allocated: usize) -> StringBuffer {
    StringBuffer {
      buf: vec![],
      len,
      index,
      allocated,
    }
  }

  fn get_buffer(&mut self) -> &mut Vec<char> {
    &mut self.buf
  }

  fn get_buf_as_string(&self) -> String {
    let mut out = String::new();
    for c in self.buf.iter() {
      out.push(*c);
    }
    out
  }
}

struct StringPool {
  entries: Vec<StringPoolEntry>,
  size: usize,
  table: HashMap<String, String>,
}

impl StringPool {
  fn new() -> StringPool {
    StringPool {
      entries: vec![],
      size: 0,
      table: HashMap::new(),
    }
  }
}

struct StringPoolEntry {
  id: usize,
  buf: StringBuffer,
}

impl StringPoolEntry {
  fn new_empty(buf: StringBuffer) -> StringPoolEntry {
    StringPoolEntry { id: 0, buf }
  }

  fn new(buf: StringBuffer, id: usize) -> StringPoolEntry {
    StringPoolEntry { id, buf }
  }
}
