use std::str::Chars;
use wasm_bindgen::prelude::*;

use crate::structure::node::Node;

fn parse_key(key: &str) -> Chars {
  key.chars()
}

#[wasm_bindgen]
pub struct Trie {
  root: Node,
}

#[wasm_bindgen]
impl Trie {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Trie {
    Trie {
      root: Node::root(),
    }
  }

  pub fn insert(&mut self, key: &str) {
    let mut current = &mut self.root;
    // loop until key is empty
    for code in parse_key(key) {
      let children = &mut current.children;
      // check if current node has child node
      if !children.has_child(code) {
        // create new node
        let new_node = Node::new(
          current.key.clone() + code.to_string().as_str(),
        );
        // add new node to current node
        children.set_child(code, new_node);
      }
      current = children.get_mut_child(code);
    }
    // set current node as item
    current.item = true;
  }

  fn search(&self, key: &str) -> Option<&Node> {
    let mut current = &self.root;

    for code in parse_key(key) {
      let children = &current.children;
      if !children.has_child(code) {
        return None;
      }
      current = children.get_child(code);
    }
    Some(current)
  }

  pub fn exact(&self, key: &str) -> JsValue {
    let node = self.search(key);

    if node.is_some() && node.unwrap().item {
      return serde_wasm_bindgen::to_value(&node.unwrap().key).unwrap();
    }
    return JsValue::NULL;
  }

  pub fn prefix(&self, key: &str) -> JsValue {
    let mut result = Vec::new();
    let node = self.search(key);
    if node.is_none() {
      return JsValue::NULL;
    }
    let node = node.unwrap();
    result.push(node.key.clone());
    result.extend(node.travel().into_iter().map(|node| node.key.clone()));
    return serde_wasm_bindgen::to_value(&result).unwrap();
  }
}