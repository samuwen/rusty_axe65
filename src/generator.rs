// use crate::configuration::generate_config_data;
use crate::node::{Node, NodeType};
use crate::opcode::*;
use std::collections::HashMap;

pub fn generate(tree: Node<String>, _config_file: &String) -> Vec<String> {
  let mut context = Context::new(&tree);
  create_symbols(&tree, &mut context);
  create_size_map(&tree, &mut context);
  // let config = generate_config_data(config_file);
  vec![]
}

fn create_symbols(tree: &Node<String>, context: &mut Context) {
  for node in tree.get_children() {
    match node.get_type() {
      NodeType::AssignmentStatement => add_assignment_variables(&node, context),
      NodeType::DirectiveStatement => switch_current_segment(&node.get_first_child(), context),
      NodeType::LabelStatement => add_labels(&node.get_first_child(), context),
      _ => (),
    }
  }
}

fn add_assignment_variables(node: &Node<String>, context: &mut Context) {
  let key = node.get_first_data_result();
  let child = node.get_first_child();
  let value = child.get_first_data_result();
  let value = u16::from_str_radix(value, 10).unwrap();
  context.add_var_to_map(key, value);
}

fn switch_current_segment(node: &Node<String>, context: &mut Context) {
  if node.get_type() == &NodeType::DirectiveSegment {
    context.switch_segment(node.get_first_data_result());
  }
}

fn add_labels(node: &Node<String>, context: &mut Context) {
  let name = node.get_first_data_result();
  context.add_label_to_map(name);
}

fn get_count(node_type: &NodeType, children: &Vec<Node<String>>) -> usize {
  children
    .iter()
    .filter(|c| c.get_type() == node_type)
    .count()
}

fn create_size_map(tree: &Node<String>, context: &mut Context) {
  for child in tree.get_children() {
    match child.get_type() {
      NodeType::DirectiveStatement => handle_directive_statement(child, context),
      NodeType::LabelStatement => handle_label_statement(child, context),
      NodeType::OpcodeStatement => handle_opcode_statement(child, context),
      _ => (),
    }
  }
}

fn handle_directive_statement(node: &Node<String>, context: &mut Context) {
  for child in node.get_children() {
    match child.get_type() {
      NodeType::DirectiveSegment => context.switch_segment(child.get_first_data_result()),
      NodeType::DirectiveByte | NodeType::DirectiveByt => handle_byte_directive(child, context),
      _ => panic!("Directive not found {:?}", child.get_type()),
    }
  }
}

fn handle_label_statement(node: &Node<String>, context: &mut Context) {
  for child in node.get_children() {
    let data = child.get_first_data_result();
    context.add_offset_to_label(data);
  }
}

fn handle_opcode_statement(node: &Node<String>, context: &mut Context) {
  for child in node.get_children() {
    match child.get_type() {
      NodeType::AccumulatorMode => handle_accumulator_mode(child, context),
      NodeType::ImmediateMode => handle_immediate_mode(child, context),
      _ => (),
    }
  }
}

fn handle_byte_directive(node: &Node<String>, context: &mut Context) {
  for child in node.get_children() {
    let dir_args = child.get_children();
    for arg in dir_args {
      match arg.get_type() {
        NodeType::String => {
          let data = arg.get_first_data_result();
          data
            .chars()
            .for_each(|c| context.add_byte_to_current_segment(c as u8));
        }
        NodeType::Number => {
          // we know this is a 1 byte number
          let num = get_bytes_from_number_node(arg);
          context.add_byte_to_current_segment(num[0]);
        }
        _ => panic!("nyi"),
      }
    }
  }
}

fn handle_accumulator_mode(node: &Node<String>, context: &mut Context) {
  let opcode_name = node.get_first_data_result();
  let opcode = get_accumulator(opcode_name);
  context.add_byte_to_current_segment(opcode);
}

fn handle_immediate_mode(node: &Node<String>, context: &mut Context) {
  let opcode_name = node.get_first_data_result();
  let opcode = get_immediate(opcode_name);
  context.add_byte_to_current_segment(opcode);
  let operand_node = node.get_first_child();
  let bytes = match operand_node.get_type() {
    NodeType::Number => get_bytes_from_number_node(operand_node),
    NodeType::Variable => {
      let variable = operand_node.get_first_data_result();
      let num = context.get_var(variable);
      num.to_le_bytes()
    }
    _ => panic!("Invalid child node for immediate mode");
  };
  context.add_byte_to_current_segment(bytes[0]);
}

fn get_bytes_from_number_node(node: &Node<String>) -> [u8; 2] {
  let data = node.get_first_data_result();
  let full = u16::from_str_radix(data, 10).unwrap();
  full.to_le_bytes()
}

struct Context {
  var_map: HashMap<String, u16>,
  label_map: HashMap<String, Label>,
  segment_list: Vec<Segment>,
  seg_counter: u8,
  current_seg_id: u8,
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

  fn get_var(&self, k: &String) -> u16 {
    *self.var_map.get(k).unwrap()
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

  fn get_current_segment(&mut self) -> &mut Segment {
    let index = self
      .segment_list
      .iter()
      .position(|s| s.id == self.current_seg_id)
      .unwrap();
    self.segment_list.get_mut(index).unwrap()
  }

  fn get_current_segment_offset(&mut self) -> u16 {
    self.get_current_segment().get_offset()
  }

  fn add_byte_to_current_segment(&mut self, byte: u8) {
    let seg = self.get_current_segment();
    seg.add_byte(byte);
  }

  fn add_offset_to_label(&mut self, label_name: &String) {
    let offset = self.get_current_segment_offset();
    let label = self.label_map.get_mut(label_name).unwrap();
    label.add_offset(offset);
  }
}

struct Label {
  segment_id: u8,
  offset_from_seg_start: u16,
}

impl Label {
  fn new(segment_id: u8) -> Label {
    Label {
      segment_id,
      offset_from_seg_start: 0,
    }
  }

  fn add_offset(&mut self, offset: u16) {
    self.offset_from_seg_start = offset;
  }
}

struct Segment {
  id: u8,
  name: String,
  read_write: ReadWrite,
  address_mode: AddressMode,
  current_offset: u16,
  bytes: Vec<Option<u8>>,
}

impl Segment {
  fn new(id: u8, name: &String) -> Segment {
    Segment {
      id,
      name: name.to_owned(),
      read_write: ReadWrite::Ro,
      address_mode: AddressMode::Absolute,
      current_offset: 0,
      bytes: vec![],
    }
  }

  fn add_byte(&mut self, byte: u8) {
    self.bytes.push(Some(byte));
  }

  fn add_bytes(&mut self, bytes: &mut Vec<u8>) {
    bytes.iter().for_each(|b| self.bytes.push(Some(*b)));
  }

  fn get_offset(&self) -> u16 {
    self.current_offset
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
