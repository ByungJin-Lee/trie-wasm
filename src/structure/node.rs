use std::collections::HashMap;

pub struct Node {
  pub children: Children,
  pub key: String,
  pub item: bool,
}

impl Node {
  pub fn new(key: String) -> Node {
    Node {
      children: Children::new(),
      item: false,
      key: key,
    }
  }

  pub fn root() -> Node {
    Node {
      children: Children::new(),
      key: "".to_string(),
      item: false,
    }
  }

  pub fn travel(&self) -> Vec<&Node> {
    let mut result = Vec::new();
    for (_, node) in self.children.nodes.iter() {
      if node.item {
        result.push(node);
      }
      result.extend(node.travel());
    }
    return result;
  }
}

pub struct Children {
  nodes: HashMap<char, Node>,
}

impl Children {
  pub fn new() -> Children {
    Children {
      nodes: HashMap::new(),
    }
  }

  pub fn has_child(&self, key: char) -> bool {
    self.nodes.contains_key(&key)
  }

  pub fn get_child(&self, key: char) -> &Node {
    self.nodes.get(&key).unwrap()
  }

  pub fn get_mut_child(&mut self, key: char) -> &mut Node {
    self.nodes.get_mut(&key).unwrap()
  }

  pub fn set_child(&mut self, key: char, node: Node) {
    self.nodes.insert(key, node);
  }
}