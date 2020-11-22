struct Node<T> {
  name: String,
  n_type: NodeType,
  data: Vec<T>,
  children: Node<T>,
}

impl<T> Node<T> {
  pub fn new(node_type: NodeType) -> Node<T> {
    Node {
      name: String::from(""),
      n_type: node_type,
      data: vec![],
      children: vec![],
    }
  }
}

enum NodeType {
  Program,
}
