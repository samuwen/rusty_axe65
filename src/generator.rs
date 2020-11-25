use crate::node::{Node, NodeType};
use crate::opcode::*;

pub fn generate(tree: Node<String>) -> Vec<String> {
  let mut out_vec: Vec<String> = vec![];
  for child in tree.get_children() {
    generate_opcode(child, &mut out_vec);
  }
  out_vec
}

fn generate_opcode(opcode: &Node<String>, out_vec: &mut Vec<String>) {
  let opcode_num = get_immediate(opcode.get_name());
  out_vec.push(format!("{}", opcode_num));
}
