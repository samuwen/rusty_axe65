use crate::configuration::{generate_config_data, Configuration, SegType};
use crate::node::{Node, NodeType};
use crate::opcode::*;
use std::collections::HashMap;

pub fn generate(tree: Node<String>, config_file: &String) -> Vec<String> {
  let config = generate_config_data(config_file);
  let mut context = Context::new(&tree, config);
  create_symbols(&tree, &mut context);
  create_size_map(&tree, &mut context);
  populate_data(&tree, &mut context);
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
  match node.get_type() {
    NodeType::UnnamedLabel => context.add_unnamed_label_to_map(),
    _ => {
      let name = node.get_first_data_result();
      context.add_label_to_map(name);
    }
  }
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
      NodeType::AssignmentStatement => (),
      _ => panic!("Invalid statement type {:?}", child.get_type()),
    }
  }
}

fn populate_data(tree: &Node<String>, context: &mut Context) {
  todo!();
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
    context.add_size_to_label(data);
  }
}

fn handle_opcode_statement(node: &Node<String>, context: &mut Context) {
  for child in node.get_children() {
    match child.get_type() {
      NodeType::AccumulatorMode => handle_accumulator_mode(child, context),
      NodeType::ImmediateMode => handle_immediate_mode(child, context),
      NodeType::DirectMode => handle_direct_mode(child, context),
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
          let count = data.chars().count();
          context.add_size_to_current_segment(count);
        }
        NodeType::Number => {
          context.add_size_to_current_segment(1);
        }
        NodeType::BinaryOp => {
          // assuming that a sequence of operations will result in a single byte of data
          context.add_size_to_current_segment(1);
        }
        _ => panic!("nyi"),
      }
    }
  }
}

fn handle_accumulator_mode(node: &Node<String>, context: &mut Context) {
  context.add_size_to_current_segment(1);
}

fn handle_immediate_mode(node: &Node<String>, context: &mut Context) {
  context.add_size_to_current_segment(2);
}

fn handle_direct_mode(node: &Node<String>, context: &mut Context) {
  let operand_node = node.get_first_child();
  match operand_node.get_type() {
    NodeType::Number => {
      let bytes = get_bytes_from_number_node(operand_node);
      let count = match bytes[1] > 0 {
        true => 3,
        false => 2,
      };
      context.add_size_to_current_segment(count);
    }
    NodeType::Variable => {
      let variable = operand_node.get_first_data_result();
      context.add_size_from_variable(variable);
    }
    _ => panic!(
      "Invalid child for direct mode {:?}",
      operand_node.get_type()
    ),
  }
}

fn get_bytes_from_number_node(node: &Node<String>) -> [u8; 2] {
  let data = node.get_first_data_result();
  let full = u16::from_str_radix(data, 10).unwrap();
  full.to_le_bytes()
}

struct Context {
  config: Configuration,
  var_map: HashMap<String, u16>,
  label_map: HashMap<String, Label>,
  segment_list: Vec<Segment>,
  seg_counter: u8,
  current_seg_id: u8,
  unnamed_label_counter: u16,
}

impl Context {
  fn new(tree: &Node<String>, config: Configuration) -> Context {
    let label_count = get_count(&NodeType::LabelStatement, tree.get_children());
    let assign_count = get_count(&NodeType::AssignmentStatement, tree.get_children());
    Context {
      config,
      var_map: HashMap::with_capacity(assign_count),
      label_map: HashMap::with_capacity(label_count),
      segment_list: vec![],
      seg_counter: 0,
      current_seg_id: 0,
      unnamed_label_counter: 0,
    }
  }

  fn add_var_to_map(&mut self, k: &String, v: u16) {
    self.var_map.insert(k.to_owned(), v);
  }

  fn get_var(&self, k: &String) -> Option<&u16> {
    self.var_map.get(k)
  }

  fn get_label(&mut self, k: &String) -> Option<&mut Label> {
    self.label_map.get_mut(k)
  }

  fn handle_variable(&mut self, k: &String) {
    let var_opt = self.get_var(k);
    match var_opt {
      Some(num) => {
        let bytes = num.to_le_bytes();
        match num > &0xFF {
          true => {
            self.add_value_to_current_segment(bytes[0]);
            self.add_value_to_current_segment(bytes[1]);
          }
          false => {
            self.add_value_to_current_segment(bytes[0]);
          }
        }
      }
      None => {
        let label = self.get_label(k).unwrap().clone();
        self.get_current_segment().add_label(label.clone());
      }
    }
  }

  fn add_size_from_variable(&mut self, k: &String) {
    let var_opt = self.get_var(k);
    match var_opt {
      Some(num) => {
        let count = match num > &0xFF {
          true => 3,
          false => 2,
        };
        self.get_current_segment().add_size(count);
      }
      None => {
        let label = self.get_label(k).unwrap();
        let seg_id = label.get_segment();
        let segment = self.get_segment_by_id(seg_id);
        match segment {
          Some(seg) => {
            let count = match seg.get_mode() {
              AddressMode::Absolute => 3,
              _ => 2,
            };
            self.add_size_to_current_segment(count);
          }
          None => panic!("Label has invalid segment id {}", seg_id),
        }
      }
    }
  }

  fn add_label_to_map(&mut self, k: &String) {
    let label = Label::new(self.current_seg_id);
    self.label_map.insert(k.to_owned(), label);
  }

  fn add_unnamed_label_to_map(&mut self) {
    let name = format!("label-{}", self.unnamed_label_counter);
    self.unnamed_label_counter += 1;
    self.add_label_to_map(&name);
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
        let config_entry = self.config.find_segment_by_name(name);
        match config_entry {
          Some(seg_entry) => {
            let seg_type = seg_entry.get_type();
            let segment = Segment::new(id, name, AddressMode::from_seg_type(seg_type));
            self.current_seg_id = id;
            self.segment_list.push(segment);
          }
          None => panic!("Segment undefined in configuration file: {}", name),
        }
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

  fn get_current_segment_size(&mut self) -> u16 {
    self.get_current_segment().get_size()
  }

  fn add_value_to_current_segment(&mut self, byte: u8) {
    let seg = self.get_current_segment();
    seg.add_value(byte);
  }

  fn add_size_to_label(&mut self, label_name: &String) {
    let offset = self.get_current_segment_size();
    let label = self.label_map.get_mut(label_name).unwrap();
    label.add_offset(offset);
  }

  fn add_size_to_current_segment(&mut self, byte: usize) {
    let seg = self.get_current_segment();
    seg.add_size(byte as u16);
  }

  fn get_segment_by_id(&self, id: u8) -> Option<&Segment> {
    self.segment_list.iter().find(|s| s.id == id)
  }
}

#[derive(Clone)]
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

  fn get_segment(&self) -> u8 {
    self.segment_id
  }

  fn add_offset(&mut self, offset: u16) {
    self.offset_from_seg_start = offset;
  }

  fn get_offset(&self) -> u16 {
    self.offset_from_seg_start
  }
}

struct Segment {
  id: u8,
  name: String,
  values: Vec<Storage>,
  size: u16,
  address_mode: AddressMode,
}

impl Segment {
  fn new(id: u8, name: &String, mode: AddressMode) -> Segment {
    Segment {
      id,
      name: name.to_owned(),
      values: vec![],
      size: 0,
      address_mode: mode,
    }
  }

  fn add_value(&mut self, byte: u8) {
    self.values.push(Storage::new_value(byte));
  }

  fn add_label(&mut self, label: Label) {
    self.values.push(Storage::new_label(label));
  }

  fn add_size(&mut self, size: u16) {
    self.size += size;
  }

  fn get_size(&self) -> u16 {
    self.size
  }

  fn get_mode(&self) -> &AddressMode {
    &self.address_mode
  }
}

impl Eq for Segment {}
impl PartialEq for Segment {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

struct Storage {
  value: Option<u8>,
  label: Option<Label>,
}

impl Storage {
  fn new_value(value: u8) -> Storage {
    Storage {
      value: Some(value),
      label: None,
    }
  }

  fn new_label(label: Label) -> Storage {
    Storage {
      value: None,
      label: Some(label),
    }
  }
}

enum AddressMode {
  ZeroPage,
  Absolute,
}

impl AddressMode {
  fn from_seg_type(seg_type: &SegType) -> AddressMode {
    match seg_type {
      SegType::Zp => AddressMode::ZeroPage,
      _ => AddressMode::Absolute,
    }
  }
}
