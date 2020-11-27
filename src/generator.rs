use crate::node::Node;
use log::*;

pub fn generate(tree: Node<String>) -> Vec<String> {
  let mut out_vec: Vec<String> = vec![];
  for child in tree.get_children() {
    generate_opcode(child, &mut out_vec);
  }
  debug!("{}", out_vec.join("\n"));
  out_vec
}

fn generate_opcode(opcode: &Node<String>, out_vec: &mut Vec<String>) {
  let mut out_string = String::new();
  for child in opcode.get_children() {
    for data in child.get_data() {
      out_string.push_str(data);
    }
  }
  out_vec.push(out_string);
}
