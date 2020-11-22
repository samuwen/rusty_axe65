pub struct Node<T> {
  name: String,
  n_type: NodeType,
  data: Vec<T>,
  children: Vec<Node<T>>,
}

impl<T> Node<T> {
  pub fn _new(node_type: NodeType) -> Node<T> {
    Node {
      name: String::from(""),
      n_type: node_type,
      data: vec![],
      children: vec![],
    }
  }
}

pub enum NodeType {
  Program,
}
