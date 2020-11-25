use crate::node::{Node, NodeType};

pub fn generate(tree: Node<String>) -> Vec<String> {
  let mut out_vec: Vec<String> = vec![];
  for child in tree.get_children() {
    generate_opcode(child, &mut out_vec);
  }
  out_vec
}

fn generate_opcode(opcode: &Node<String>, out_vec: &mut Vec<String>) {
  let opcode_num = opcode.get_opcode_value();
  out_vec.push(format!("{:X}", opcode_num));
}
