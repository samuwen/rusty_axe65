use crate::configuration::{generate_config_data, Configuration, SegType};
use crate::node::{Node, NodeType};
use crate::opcode::*;
use std::collections::HashMap;
use std::fs::{metadata, read};

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
  context.reset_label_count();
  for child in tree.get_children() {
    match child.get_type() {
      NodeType::DirectiveStatement => add_directive_sizes(child, context),
      NodeType::LabelStatement => add_label_sizes(child, context),
      NodeType::OpcodeStatement => add_opcode_sizes(child, context),
      NodeType::AssignmentStatement => (),
      _ => panic!("Invalid statement type {:?}", child.get_type()),
    }
  }
}

fn add_directive_sizes(node: &Node<String>, context: &mut Context) {
  for child in node.get_children() {
    match child.get_type() {
      NodeType::DirectiveSegment => context.switch_segment(child.get_first_data_result()),
      NodeType::DirectiveByte | NodeType::DirectiveByt => add_byte_sizes(child, context),
      NodeType::DirectiveIncbin => add_incbin_sizes(child, context),
      NodeType::DirectiveWord => add_word_sizes(child, context),
      NodeType::DirectiveRes => add_res_sizes(child, context),
      _ => panic!("Directive not found {:?}", child.get_type()),
    }
  }
}

fn add_label_sizes(node: &Node<String>, context: &mut Context) {
  for child in node.get_children() {
    match child.get_type() {
      NodeType::UnnamedLabel => {
        context.add_size_to_unnamed_label();
      }
      NodeType::Label | NodeType::LocalLabel => {
        let data = child.get_first_data_result();
        context.add_size_to_label(data);
      }
      _ => panic!("Invalid label type {:?}", child.get_type()),
    }
  }
}

fn add_opcode_sizes(node: &Node<String>, context: &mut Context) {
  for child in node.get_children() {
    match child.get_type() {
      NodeType::AccumulatorMode => add_accumulator_sizes(context),
      NodeType::ImmediateMode => add_immediate_mode_sizes(context),
      NodeType::DirectMode => add_direct_mode_sizes(child, context),
      _ => (),
    }
  }
}

fn add_byte_sizes(node: &Node<String>, context: &mut Context) {
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

fn add_incbin_sizes(node: &Node<String>, context: &mut Context) {
  let dir_args = node.get_first_child();
  for arg in dir_args.get_children() {
    let file_name = arg.get_first_data_result();
    let path = format!("src/data/{}", file_name);
    let meta = metadata(path).expect("file not found");
    context.add_size_to_current_segment(meta.len() as usize);
  }
}

fn add_word_sizes(node: &Node<String>, context: &mut Context) {
  let dir_args = node.get_first_child();
  for _ in dir_args.get_children() {
    context.add_size_to_current_segment(2);
  }
}

fn add_res_sizes(node: &Node<String>, context: &mut Context) {
  let dir_args = node.get_first_child();
  let size = dir_args.get_first_child();
  let number = usize::from_str_radix(size.get_first_data_result(), 10).unwrap();
  context.add_size_to_current_segment(number);
}

fn add_accumulator_sizes(context: &mut Context) {
  context.add_size_to_current_segment(1);
}

fn add_immediate_mode_sizes(context: &mut Context) {
  context.add_size_to_current_segment(2);
}

fn add_direct_mode_sizes(node: &Node<String>, context: &mut Context) {
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
    NodeType::LabelJump => {
      let data = operand_node.get_first_data_result();
      let is_pos = data.chars().any(|c| c == '+');
      let f = |op| data.chars().filter(|c| c == &op).count();
      let count = match is_pos {
        true => f('+'),
        false => f('-'),
      };
      context.add_size_to_label_jump(is_pos, count);
    }
    NodeType::BinaryOp => {
      // in the case of a label we need to parse through the expression tree
      // to figure out how big the number is
      let result = evaluate_binary_op_for_size(operand_node, context);
      let size = match result > 0xFF {
        true => 3,
        false => 2,
      };
      context.add_size_to_current_segment(size);
    }
    _ => panic!(
      "Invalid child for direct mode {:?}",
      operand_node.get_type()
    ),
  }
}

fn evaluate_binary_op_for_size(node: &Node<String>, context: &mut Context) -> u16 {
  let left = node.get_first_child();
  let right = node.get_children().get(1).unwrap();
  let operator = node.get_first_data_result();
  let left_num = evaluate_term_for_size(left, context);
  let right_num = evaluate_term_for_size(right, context);
  match operator.as_str() {
    "+" => left_num + right_num,
    "-" => left_num - right_num,
    "/" => left_num / right_num,
    "*" => left_num * right_num,
    "|" => left_num | right_num,
    "&" => left_num & right_num,
    "<<" => left_num << right_num,
    ">>" => left_num >> right_num,
    _ => panic!("unsupported operator {}", operator),
  }
}

fn evaluate_term_for_size(node: &Node<String>, context: &mut Context) -> u16 {
  let name = node.get_first_data_result();
  match node.get_type() {
    NodeType::Number => u16::from_str_radix(name, 10).unwrap(),
    NodeType::BinaryOp => evaluate_binary_op_for_size(node, context),
    NodeType::Variable => {
      let var_opt = context.get_var(name);
      match var_opt {
        Some(num) => *num,
        // if we're not in zero page add 0x100 to determine byte size
        None => match context.is_label_in_zero_page(name) {
          true => 0,
          false => 0x100,
        },
      }
    }
    _ => panic!("unknown term: {:?}", node.get_type()),
  }
}

fn get_bytes_from_number_node(node: &Node<String>) -> [u8; 2] {
  let data = node.get_first_data_result();
  let full = u16::from_str_radix(data, 10).unwrap();
  full.to_le_bytes()
}

fn populate_data(tree: &Node<String>, context: &mut Context) {
  context.reset_label_count();
  for child in tree.get_children() {
    match child.get_type() {
      NodeType::AssignmentStatement => (),
      NodeType::DirectiveStatement => populate_directive_data(child, context),
      NodeType::LabelStatement => {
        let label_type_node = child.get_first_child();
        if label_type_node.get_type() == &NodeType::UnnamedLabel {
          context.advance_unnamed_label_counter();
        }
      }
      NodeType::OpcodeStatement => populate_opcode_data(child, context),
      _ => panic!("Invalid statement type {:?}", child.get_type()),
    }
  }
}

fn populate_directive_data(node: &Node<String>, context: &mut Context) {
  for child in node.get_children() {
    match child.get_type() {
      NodeType::DirectiveSegment => context.switch_segment(child.get_first_data_result()),
      NodeType::DirectiveByte | NodeType::DirectiveByt => populate_bytes(child, context),
      NodeType::DirectiveIncbin => populate_incbin(child, context),
      NodeType::DirectiveWord => populate_words(child, context),
      NodeType::DirectiveRes => (),
      _ => panic!("Unimplemented type {:?}", child.get_type()),
    }
  }
}

fn populate_bytes(node: &Node<String>, context: &mut Context) {
  for child in node.get_children() {
    for arg in child.get_children() {
      match arg.get_type() {
        NodeType::String => {
          let data = arg.get_first_data_result();
          data
            .chars()
            .for_each(|c| context.add_value_to_current_segment(c as u8));
        }
        NodeType::Number => {
          let num = arg.get_first_data_result();
          let num = u8::from_str_radix(num, 10).unwrap();
          context.add_value_to_current_segment(num);
        }
        NodeType::BinaryOp => {
          let num = evaluate_binary_expression(arg, context);
          match num > 0xFF {
            true => {
              let bytes = num.to_le_bytes();
              context.add_value_to_current_segment(bytes[0]);
              context.add_value_to_current_segment(bytes[1]);
            }
            false => {
              context.add_value_to_current_segment(num as u8);
            }
          }
        }
        _ => panic!("nyi"),
      }
    }
  }
}

fn populate_incbin(node: &Node<String>, context: &mut Context) {
  for child in node.get_children() {
    for arg in child.get_children() {
      for file_name in arg.get_data() {
        let path = format!("src/data/{}", file_name);
        let error = format!("File not found {}", file_name);
        let bytes = read(path).expect(&error);
        for byte in bytes.iter() {
          context.add_value_to_current_segment(*byte);
        }
      }
    }
  }
}

fn populate_words(node: &Node<String>, context: &mut Context) {
  for child in node.get_children() {
    for arg in child.get_children() {
      match arg.get_type() {
        NodeType::Variable => {
          let name = arg.get_first_data_result();
          let var_opt = context.get_var(name);
          let value = match var_opt {
            Some(num) => *num,
            None => context.get_label_address(name),
          };
          let bytes = value.to_le_bytes();
          context.add_value_to_current_segment(bytes[0]);
          context.add_value_to_current_segment(bytes[1]);
        }
        _ => panic!("Not yet implemented: {:?}", arg.get_type()),
      }
    }
  }
}

fn populate_opcode_data(node: &Node<String>, context: &mut Context) {
  for child in node.get_children() {
    match child.get_type() {
      NodeType::AccumulatorMode => populate_accumulator_mode(child, context),
      NodeType::ImmediateMode => populate_immediate_mode(child, context),
      NodeType::DirectMode => populate_direct_mode(child, context),
      NodeType::RelativeMode => populate_relative_mode(child, context),
      _ => panic!("Not yet implemented: {:?}", child.get_type()),
    }
  }
}

fn populate_accumulator_mode(node: &Node<String>, context: &mut Context) {
  let opcode = node.get_first_data_result();
  let num = get_accumulator(opcode);
  context.add_value_to_current_segment(num);
}

fn populate_immediate_mode(node: &Node<String>, context: &mut Context) {
  let opcode = node.get_first_data_result();
  let num = get_immediate(opcode);
  context.add_value_to_current_segment(num);
  let operand_node = node.get_first_child();
  let operand_data = match operand_node.get_type() {
    NodeType::Number => {
      let data = operand_node.get_first_data_result();
      u8::from_str_radix(data, 10).unwrap()
    }
    NodeType::Variable => {
      let data = operand_node.get_first_data_result();
      let var_num = match context.get_var(data) {
        Some(val) => *val,
        None => context.get_label_address(data),
      };
      match var_num > 0xFF {
        false => var_num as u8,
        true => panic!("Immediate mode number has more than one byte"),
      }
    }
    // a unary op in this case is almost certainly hibyte or lobyte
    NodeType::UnaryOp => {
      let op = operand_node.get_first_data_result();
      let child = operand_node.get_first_child();
      let data = match child.get_type() {
        NodeType::Variable => context.get_label_address(child.get_first_data_result()),
        _ => panic!("Unknown child type: {:?}", child.get_type()),
      };
      let bytes = data.to_le_bytes();
      match op.as_str() {
        "<" => bytes[0],
        ">" => bytes[1],
        _ => panic!("Unknown operator for immediate unary: {:?}", op),
      }
    }
    _ => panic!("honk"),
  };
  context.add_value_to_current_segment(operand_data);
}

fn populate_direct_mode(node: &Node<String>, context: &mut Context) {
  let opcode = node.get_first_data_result();
  let op_node = node.get_first_child();
  let num = match op_node.get_type() {
    NodeType::Number => {
      let data = op_node.get_first_data_result();
      u16::from_str_radix(data, 10).unwrap()
    }
    NodeType::Variable => {
      let name = op_node.get_first_data_result();
      let var_opt = context.get_var(name);
      match var_opt {
        Some(num) => *num,
        None => context.get_label_address(name),
      }
    }
    _ => panic!("Invalid node child {:?}", op_node.get_type()),
  };
  let bytes = num.to_le_bytes();
  match num > 0xFF {
    true => {
      let opcode_byte = get_absolute(opcode);
      context.add_value_to_current_segment(opcode_byte);
      context.add_value_to_current_segment(bytes[0]);
      context.add_value_to_current_segment(bytes[1]);
    }
    false => {
      let opcode_byte = get_zero_page(opcode);
      context.add_value_to_current_segment(opcode_byte);
      context.add_value_to_current_segment(bytes[0]);
    }
  }
}

fn populate_relative_mode(node: &Node<String>, context: &mut Context) {
  let opcode = node.get_first_data_result();
  let op_node = node.get_first_child();
  match op_node.get_type() {
    NodeType::LabelJump => {
      let data = op_node.get_first_data_result();
      let is_pos = data.chars().any(|c| c == '+');
      let f = |op| data.chars().filter(|c| c == &op).count();
      let count = match is_pos {
        true => f('+'),
        false => f('-'),
      };
      let address = context.get_address_for_label_jump(is_pos, count);
      let opcode_byte = get_relative(opcode);
      context.add_value_to_current_segment(opcode_byte);
      context.add_value_to_current_segment(address);
    }
    _ => panic!("stuff"),
  }
}

fn evaluate_binary_expression(node: &Node<String>, context: &mut Context) -> u16 {
  let left = node.get_first_child();
  let right = node.get_children().get(1).unwrap();
  let operator = node.get_first_data_result();
  let left_num = evaluate_term(left, context);
  let right_num = evaluate_term(right, context);
  match operator.as_str() {
    "+" => left_num + right_num,
    "-" => left_num - right_num,
    "/" => left_num / right_num,
    "*" => left_num * right_num,
    "|" => left_num | right_num,
    "&" => left_num & right_num,
    "<<" => left_num << right_num,
    ">>" => left_num >> right_num,
    _ => panic!("unsupported operator {}", operator),
  }
}

fn evaluate_term(node: &Node<String>, context: &mut Context) -> u16 {
  let name = node.get_first_data_result();
  match node.get_type() {
    NodeType::Number => u16::from_str_radix(name, 10).unwrap(),
    NodeType::BinaryOp => evaluate_binary_op_for_size(node, context),
    NodeType::Variable => {
      let var_opt = context.get_var(name);
      match var_opt {
        Some(num) => *num,
        None => context.get_label_address(name),
      }
    }
    _ => panic!("unknown term: {:?}", node.get_type()),
  }
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

  fn get_formatted_name(&mut self, count: u16) -> String {
    format!("label-{}", count)
  }

  fn advance_unnamed_label_counter(&mut self) {
    self.unnamed_label_counter += 1;
  }

  fn get_unnamed_label_now(&mut self) -> String {
    let name = self.get_formatted_name(self.unnamed_label_counter);
    self.unnamed_label_counter += 1;
    name
  }

  fn add_unnamed_label_to_map(&mut self) {
    let name = self.get_unnamed_label_now();
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

  fn is_label_in_zero_page(&self, label_name: &String) -> bool {
    let label = self.label_map.get(label_name).unwrap();
    let segment = self.get_segment_by_id(label.get_segment()).unwrap();
    segment.get_mode() == &AddressMode::ZeroPage
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

  fn get_address_for_label_jump(&mut self, is_pos: bool, count: usize) -> u8 {
    let count = count as u16;
    let num = match is_pos {
      true => self.unnamed_label_counter + count - 1,
      false => self.unnamed_label_counter - count,
    };
    let offset = self.get_current_segment_size();
    let target_name = self.get_formatted_name(num);
    let target_label = self.label_map.get(&target_name).unwrap();
    match is_pos {
      true => (target_label.get_offset() - offset) as u8,
      false => {
        let diff = offset - target_label.get_offset();
        (!diff + 1) as u8
      }
    }
  }

  fn add_size_to_label(&mut self, label_name: &String) {
    let offset = self.get_current_segment_size();
    let label = self.label_map.get_mut(label_name).unwrap();
    label.add_offset(offset);
  }

  fn add_size_to_unnamed_label(&mut self) {
    let offset = self.get_current_segment_size();
    let name = self.get_unnamed_label_now();
    let label = self.label_map.get_mut(&name).unwrap();
    label.add_offset(offset);
  }

  fn add_size_to_current_segment(&mut self, byte: usize) {
    let seg = self.get_current_segment();
    seg.add_size(byte as u16);
  }

  fn get_segment_by_id(&self, id: u8) -> Option<&Segment> {
    self.segment_list.iter().find(|s| s.id == id)
  }

  fn reset_label_count(&mut self) {
    self.unnamed_label_counter = 0;
  }

  fn add_size_to_label_jump(&mut self, is_pos: bool, count: usize) {
    let count = count as u16;
    let num = match is_pos {
      true => self.unnamed_label_counter + count - 1,
      false => self.unnamed_label_counter - count,
    };
    let name = self.get_formatted_name(num);
    self.add_size_from_variable(&name);
  }

  fn get_label_address(&self, name: &String) -> u16 {
    let label = self.label_map.get(name).unwrap();
    let segment = self.get_segment_by_id(label.get_segment()).unwrap();
    let start = self.config.get_segment_start(segment.get_name());
    start + label.get_offset()
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

  fn get_name(&self) -> &String {
    &self.name
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

#[derive(Eq, PartialEq)]
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
