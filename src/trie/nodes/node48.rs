use std::any::Any;
use crate::trie::nodes::node::{Node, NodeOption};
use crate::trie::nodes::{node0::Node0, node16::Node16, node256::Node256};
use arr_macro::arr;

#[derive(Debug)]
pub struct Node48 {
    pub(crate) keys: [Option<u8>; 256],
    //index represents value, and value represents index in children
    pub(crate) children: [NodeOption; 48],
    pub(crate) size: usize,
    pub(crate) terminal: bool,
}

impl Default for Node48 {
    fn default() -> Self {
        Node48::new()
    }
}

impl Node48 {
    pub fn new() -> Self {
        Node48 {
            keys: [None; 256],
            children: arr![None; 48],
            size: 0,
            terminal: false,
        }
    }

    pub fn from(node: &mut Node16) -> Self {
        //add keys which point to appropriate child index
        let mut new_node = Node48::new();
        //index in within keys represents the u8 and its value represents the index in children
        for i in 0..node.size as u8 {
            let index = i as usize;
            new_node.keys[node.keys[index].unwrap() as usize] = Some(i);
            new_node.children[index] = node.children[index].take();
        }

        new_node.terminal = node.terminal;
        new_node.size = node.size;
        new_node
    }
}

impl Node for Node48 {
    fn add(&mut self, values: &[u8]) -> NodeOption {
        if let Some((first, rest)) = values.split_first() {
            let cur_value_index = *first as usize;
            //if exists
            if let Some(key) = self.keys[cur_value_index] {
                let key_index = key as usize;
                let upgraded_node =  self.children[key_index]
                    .as_mut()
                    .map_or_else(| | Box::new(Node0::new()).add(rest),
                                 |v| v.add(rest));
                if upgraded_node.is_some() {
                    self.children[key_index] = upgraded_node;
                }
                None
            } else if self.is_full() {
                let mut upgraded_node = Node256::from(self);
                upgraded_node.add(values);
                Some(Box::new(upgraded_node))
            } else {
                //add to self
                self.keys[cur_value_index] = Some(self.size as u8);
                self.children[self.size] = Node0::new().add(rest);
                self.size += 1;
                None
            }
        } else {
            self.terminal = true;
            None
        }
    }

    fn is_full(&self) -> bool {
        self.size == self.children.len()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_terminal(&self) -> bool {
        self.terminal
    }

    fn exists(&self, values: &[u8]) -> bool {
        if let Some((first, rest)) = values.split_first() {
            let cur_value_index = *first as usize;
            //if exists
            if let Some(key) = self.keys[cur_value_index] {
                let key_index = key as usize;
                if let Some(child) = self.children[key_index].as_ref() {
                    child.exists(rest)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            self.terminal
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::trie::nodes::node4::Node4;
    use super::*;

    #[test]
    fn order_preserved_48_exact_match() {
        let mut node = NodeOption::Some(Box::new(Node4::new()));

        for i in 0..48 {
            let upgrade = node.as_mut().unwrap().add(&[i * 2]);
            if upgrade.is_some() {
                node = upgrade;
            }
        }

        if let Some(n) = node {
            let node48 = n.as_any().downcast_ref::<Node48>().unwrap();
            for (i, &k) in node48.keys.iter().enumerate() {
                if i < 96 { //only first entries 48 considered
                    match k {
                        None => {
                            assert_ne!(i % 2, 0);
                        },
                        Some(c) => {
                            assert_eq!(i % 2, 0);
                            assert!(matches!(&node48.children[c as usize], Some(_)));
                        },
                        _ => panic!()
                    }
                }
            }
        } else {
            panic!()
        }
    }
}