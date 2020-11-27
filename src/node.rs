use crate::opcode::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Node<T> {
  n_type: NodeType,
  data: Vec<T>,
  children: Vec<Node<T>>,
}

impl<T> Node<T> {
  pub fn new(node_type: NodeType) -> Node<T> {
    Node {
      n_type: node_type,
      data: vec![],
      children: vec![],
    }
  }

  pub fn new_with_child(node_type: NodeType, child: Node<T>) -> Node<T> {
    let mut node = Node {
      n_type: node_type,
      data: vec![],
      children: vec![],
    };
    node.add_child(child);
    node
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

  pub fn _get_type(&self) -> &NodeType {
    &self.n_type
  }

  pub fn get_children(&self) -> &Vec<Node<T>> {
    &self.children
  }
}

impl Node<String> {
  pub fn _get_opcode_value(&self) -> u8 {
    match self.n_type {
      _ => panic!("Expected opcode, got {:?}", self.n_type),
    }
  }

  fn format_self(&self, count: usize) -> String {
    let mut tabs = String::new();
    for _ in 0..count {
      tabs.push_str("  ");
    }
    let mut return_str = format!(
      "{}type: {:?} | data: {} | children:\n",
      tabs,
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
  Assignment,
}
