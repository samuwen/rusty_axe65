use crate::configuration::generate_config_data;
use crate::node::{Node, NodeType};
use std::collections::HashMap;

pub fn generate(tree: Node<String>, config_file: &String) -> Vec<String> {
  let mut context = Context::new(&tree);
  create_symbols(&tree, &mut context);
  parse_config_file(config_file);
  vec![]
}

fn create_symbols(tree: &Node<String>, context: &mut Context) {
  for node in tree.get_children() {
    match node.get_type() {
      NodeType::AssignmentStatement => handle_assignment_statement(&node, context),
      NodeType::DirectiveStatement => handle_directive_statement(&node.get_first_child(), context),
      NodeType::LabelStatement => handle_label_statement(&node.get_first_child(), context),
      _ => (),
    }
  }
}

fn handle_assignment_statement(node: &Node<String>, context: &mut Context) {
  let key = node.get_first_data_result();
  let child = node.get_first_child();
  let value = child.get_first_data_result();
  let value = u16::from_str_radix(value, 10).unwrap();
  context.add_var_to_map(key, value);
}

fn handle_directive_statement(node: &Node<String>, context: &mut Context) {
  if node.get_type() == &NodeType::DirectiveSegment {
    context.switch_segment(node.get_first_data_result());
  }
}

fn handle_label_statement(node: &Node<String>, context: &mut Context) {
  let name = node.get_first_data_result();
  context.add_label_to_map(name);
}

fn parse_config_file(config_file: &String) {
  let result = generate_config_data(config_file);
}

struct Context {
  var_map: HashMap<String, u16>,
  label_map: HashMap<String, Label>,
  segment_list: Vec<Segment>,
  seg_counter: u8,
  current_seg_id: u8,
}

fn get_count(node_type: &NodeType, children: &Vec<Node<String>>) -> usize {
  children
    .iter()
    .filter(|c| c.get_type() == node_type)
    .count()
}

impl Context {
  fn new(tree: &Node<String>) -> Context {
    let label_count = get_count(&NodeType::LabelStatement, tree.get_children());
    let assign_count = get_count(&NodeType::AssignmentStatement, tree.get_children());
    Context {
      var_map: HashMap::with_capacity(assign_count),
      label_map: HashMap::with_capacity(label_count),
      segment_list: vec![],
      seg_counter: 0,
      current_seg_id: 0,
    }
  }

  fn add_var_to_map(&mut self, k: &String, v: u16) {
    self.var_map.insert(k.to_owned(), v);
  }

  fn add_label_to_map(&mut self, k: &String) {
    let label = Label::new(self.current_seg_id);
    self.label_map.insert(k.to_owned(), label);
  }

  fn switch_segment(&mut self, name: &String) {
    let found = self.segment_list.iter().find(|s| &s.name == name);
    match found {
      Some(seg) => {
        self.current_seg_id = seg.id;
      }
      None => {
        let id = self.seg_counter;
        self.seg_counter += 1;
        let segment = Segment::new(id, name);
        self.current_seg_id = id;
        self.segment_list.push(segment);
      }
    }
  }
}

struct Label {
  segment_id: u8,
}

impl Label {
  fn new(segment_id: u8) -> Label {
    Label { segment_id }
  }
}

struct Segment {
  id: u8,
  name: String,
  read_write: ReadWrite,
  address_mode: AddressMode,
}

impl Segment {
  fn new(id: u8, name: &String) -> Segment {
    Segment {
      id,
      name: name.to_owned(),
      read_write: ReadWrite::Ro,
      address_mode: AddressMode::Absolute,
    }
  }
}

impl Eq for Segment {}
impl PartialEq for Segment {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

enum ReadWrite {
  Ro,
  Rw,
}

enum AddressMode {
  Absolute,
  Zeropage,
}
