use crate::opcode::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Node<T> {
  name: String,
  n_type: NodeType,
  data: Vec<T>,
  children: Vec<Node<T>>,
}

impl<T> Node<T> {
  pub fn new(name: &String, node_type: NodeType) -> Node<T> {
    Node {
      name: name.to_owned(),
      n_type: node_type,
      data: vec![],
      children: vec![],
    }
  }

  pub fn add_child(&mut self, child: Node<T>) {
    self.children.push(child)
  }

  pub fn add_data(&mut self, data: T) {
    self.data.push(data)
  }

  pub fn get_data(&self) -> &Vec<T> {
    &self.data
  }

  pub fn get_type(&self) -> &NodeType {
    &self.n_type
  }

  pub fn get_children(&self) -> &Vec<Node<T>> {
    &self.children
  }

  pub fn get_name(&self) -> &String {
    &self.name
  }
}

impl Node<String> {
  pub fn get_opcode_value(&self) -> u8 {
    match self.n_type {
      NodeType::ImpliedOpcode => get_implied(&self.name),
      NodeType::ImmediateOpcode => get_immediate(&self.name),
      _ => panic!("Expected opcode, got {:?}", self.n_type),
    }
  }

  fn format_self(&self, count: usize) -> String {
    let mut tabs = String::new();
    for _ in 0..count {
      tabs.push_str("  ");
    }
    let mut return_str = format!(
      "{}name: {} | type: {:?} | data: {} | children:\n",
      tabs,
      self.name,
      self.n_type,
      self.data.join(", ")
    );
    for child in self.children.iter() {
      return_str.push_str(&child.format_self(count + 1));
    }
    return_str
  }
}

impl fmt::Display for Node<String> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.format_self(0))
  }
}

#[derive(Clone, Debug)]
pub enum NodeType {
  Program,
  ImpliedOpcode,
  ImmediateOpcode,
}
