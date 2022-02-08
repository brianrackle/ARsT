use crate::trie::nodes::node::{KeyChildIndex, Node, NodeLocation, NodeOption};
use crate::trie::nodes::{node0::Node0, node16::Node16, node256::Node256};
use arr_macro::arr;
use std::any::Any;
use crate::trie::nodes::node::NodeLocation::{Exists, Insert, Upgrade};

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

    fn get_child_index(&self, value: u8) -> NodeLocation {
        let cur_value_index = value as usize;
        if let Some(key_value) = self.keys[cur_value_index] {
            Exists(KeyChildIndex{key: cur_value_index, child: key_value as usize})
        } else if !self.is_full() {
            Insert(KeyChildIndex{key: cur_value_index, child: self.size})
        } else {
            Upgrade
        }
    }

    fn exists_add(&mut self, index: &KeyChildIndex, rest: &[u8]) -> NodeOption {
        let upgraded_node = self.children[index.child]
            .as_mut()
            .map_or_else(|| Box::new(Node0::new()).add(rest), |v| v.add(rest));
        if upgraded_node.is_some() {
            self.children[index.child] = upgraded_node;
        }
        None
    }

    fn insert_add(&mut self, index: &KeyChildIndex, first: u8, rest: &[u8]) -> NodeOption {
        self.keys[index.key] = Some(self.size as u8); //FIXME this is the same as index.child
        self.children[index.child] = Node0::new().add(rest);
        self.size += 1;
        None
    }

    fn upgrade_add(&mut self, values: &[u8]) -> NodeOption {
        let mut upgraded_node = Node256::from(self);
        upgraded_node.add(values);
        Some(Box::new(upgraded_node))
    }
}

impl Node for Node48 {
    fn add(&mut self, values: &[u8]) -> NodeOption {
        if let Some((first, rest)) = values.split_first() {
            match &self.get_child_index(*first) {
                Exists(index) => self.exists_add(index, rest),
                Insert(index) => self.insert_add(index, *first, rest),
                Upgrade => self.upgrade_add(values),
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
    use super::*;
    use crate::trie::nodes::node4::Node4;

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
                if i < 96 {
                    //only first entries 48 considered
                    match k {
                        None => {
                            assert_ne!(i % 2, 0);
                        }
                        Some(c) => {
                            assert_eq!(i % 2, 0);
                            assert!(matches!(&node48.children[c as usize], Some(_)));
                        }
                        _ => panic!(),
                    }
                }
            }
        } else {
            panic!()
        }
    }
}
