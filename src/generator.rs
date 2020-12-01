use crate::node::{Node, NodeType};
use log::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static LINE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static SPAN_COUNTER: AtomicUsize = AtomicUsize::new(0);
static SEG_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn generate(tree: Node<String>) -> Vec<String> {
  let mut out_vec: Vec<String> = vec![];
  let mut context = Context::new();
  for child in tree.get_children() {
    generate_code(child, &mut context);
  }
  out_vec
}

fn generate_code(node: &Node<String>, context: &mut Context) {
  match node.get_type() {
    NodeType::AssignmentStatement => {
      let (name, value) = parse_assignment_statement(node);
      context.add_var(name, value);
    }
    NodeType::DirectiveStatement => {
      for child in node.get_children() {
        match child.get_type() {
          NodeType::DirectiveSegment => {
            let segment = parse_dir_segment(child);
            context.change_segments(segment);
          }
          NodeType::DirectiveByte => {
            let dir_args = child
              .get_children()
              .get(0)
              .expect("Byte directive with no dir args");
            parse_line(dir_args, context);
          }
          _ => todo!(),
        }
      }
    }
    _ => panic!("the disco"),
  }
}

fn parse_line(node: &Node<String>, context: &mut Context) {
  let mut byte_count = 0;
  for child in node.get_children() {
    for data in child.get_data() {
      let try_num_parse = u8::from_str_radix(data, 10);
      let bytes = match try_num_parse {
        Ok(_) => 1,
        Err(_) => data.as_bytes().len(),
      };
      byte_count += bytes;
    }
  }
  context.add_line(byte_count);
}

fn parse_dir_segment(node: &Node<String>) -> Segment {
  let seg_string = node
    .get_data()
    .get(0)
    .expect("Segment switch missing the segment data")
    .to_ascii_uppercase();
  Segment::new(seg_string).build()
}

fn parse_assignment_statement(node: &Node<String>) -> (String, usize) {
  let var_name = node
    .get_data()
    .get(0)
    .expect("Assignment statement missing variable name");
  let val_node = node
    .get_children()
    .get(0)
    .expect("Assignment statement missing child");
  let value = val_node
    .get_data()
    .get(0)
    .expect("Assignment statement missing value");
  (
    var_name.to_owned(),
    usize::from_str_radix(&value, 10).unwrap(),
  )
}

struct Line {
  id: usize,
  file: u8,
  line: u16,
  span: Span,
}

impl Line {
  fn new(span: Span) -> LineBuilder {
    LineBuilder {
      id: LINE_COUNTER.fetch_add(1, Ordering::Relaxed),
      file: 0,
      line: 0,
      span,
    }
  }
}

struct LineBuilder {
  id: usize,
  file: u8,
  line: u16,
  span: Span,
}

impl LineBuilder {
  fn file(mut self, f: u8) -> LineBuilder {
    self.file = f;
    self
  }

  fn line(mut self, l: u16) -> LineBuilder {
    self.line = l;
    self
  }

  fn build(self) -> Line {
    Line {
      id: self.id,
      file: self.file,
      line: self.line,
      span: self.span,
    }
  }
}

struct Span {
  id: usize,
  seg: Segment,
  start: u16,
  size: u8,
}

impl Span {
  fn new(segment: &Segment) -> SpanBuilder {
    SpanBuilder {
      id: SPAN_COUNTER.fetch_add(1, Ordering::Relaxed),
      start: 0,
      size: 0,
      seg: Segment::from(segment),
    }
  }
}

struct SpanBuilder {
  id: usize,
  seg: Segment,
  start: u16,
  size: u8,
}

impl SpanBuilder {
  fn start(mut self, start: u16) -> SpanBuilder {
    self.start = start;
    self
  }

  fn size(mut self, size: usize) -> SpanBuilder {
    self.size = size as u8;
    self
  }

  fn build(self) -> Span {
    Span {
      id: self.id,
      seg: self.seg,
      start: self.start,
      size: self.size,
    }
  }
}

struct Segment {
  id: usize,
  size: u16,
  name: String,
  address_type: AddressType,
  start: u16,
  style: Style,
}

impl Segment {
  fn new(name: String) -> SegmentBuilder {
    SegmentBuilder {
      id: SEG_COUNTER.fetch_add(1, Ordering::Relaxed),
      size: 0,
      name,
      address_type: AddressType::Absolute,
      start: 0,
      style: Style::Ro,
    }
  }

  fn from(s: &Segment) -> Segment {
    Segment {
      id: s.id,
      size: s.size,
      name: s.name.to_owned(),
      address_type: s.address_type.clone(),
      start: s.start,
      style: s.style.clone(),
    }
  }

  fn add_size(&mut self, count: usize) {
    self.size += count as u16;
  }
}

impl Eq for Segment {}

impl PartialEq for Segment {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

struct SegmentBuilder {
  id: usize,
  size: u16,
  name: String,
  address_type: AddressType,
  start: u16,
  style: Style,
}

impl SegmentBuilder {
  fn start(mut self, start: u16) -> SegmentBuilder {
    self.start = start;
    self
  }

  fn size(mut self, size: u16) -> SegmentBuilder {
    self.size = size;
    self
  }

  fn address_type(mut self, addr: AddressType) -> SegmentBuilder {
    self.address_type = addr;
    self
  }

  fn style(mut self, style: Style) -> SegmentBuilder {
    self.style = style;
    self
  }

  fn build(self) -> Segment {
    Segment {
      id: self.id,
      start: self.start,
      size: self.size,
      address_type: self.address_type,
      style: self.style,
      name: self.name,
    }
  }
}

#[derive(Clone)]
enum AddressType {
  Absolute,
  Zeropage,
}

#[derive(Clone)]
enum Style {
  Rw,
  Ro,
}

struct Context {
  var_map: HashMap<String, usize>,
  line_list: Vec<Line>,
  seg_list: Vec<Segment>,
  current_segment: Segment,
}

impl Context {
  fn new() -> Context {
    Context {
      var_map: HashMap::new(),
      line_list: vec![],
      seg_list: vec![],
      current_segment: Segment::new(String::from("NULL")).build(),
    }
  }

  fn add_var(&mut self, k: String, v: usize) {
    self.var_map.insert(k, v);
  }

  fn change_segments(&mut self, segment: Segment) {
    let defined = self.seg_list.iter().find(|s| s == &&segment);
    self.current_segment = match defined {
      Some(seg) => Segment::from(seg),
      None => {
        self.seg_list.push(Segment::from(&segment));
        segment
      }
    };
  }

  fn add_line(&mut self, bytes: usize) {
    let start = self.current_segment.size;
    self.current_segment.add_size(bytes);
    let span = Span::new(&self.current_segment)
      .size(bytes)
      .start(start)
      .build();
    self.line_list.push(Line::new(span).file(0).build());
  }
}
