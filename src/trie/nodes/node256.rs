use crate::trie::nodes::node::{KeyChildIndex, Node, NodeLocation, NodeOption};
use crate::trie::nodes::{node0::Node0, node48::Node48};
use arr_macro::arr;
use std::any::Any;
use crate::trie::nodes::node::NodeLocation::{Exists, Insert};

#[derive(Debug)]
pub struct Node256 {
    pub(crate) children: [NodeOption; 256],
    pub(crate) size: usize,
    pub(crate) terminal: bool,
}

impl Default for Node256 {
    fn default() -> Self {
        Node256::new()
    }
}

impl Node256 {
    pub fn new() -> Self {
        Node256 {
            children: arr![None; 256],
            size: 0,
            terminal: false,
        }
    }

    pub fn from(node: &mut Node48) -> Self {
        let mut new_node = Node256::new();

        for (index, key) in node.keys.iter().enumerate() {
            if let Some(key) = *key {
                let key_index = key as usize;
                new_node.children[index] = node.children[key_index].take();
            }
        }

        new_node.terminal = node.terminal;
        new_node.size = node.size;
        new_node
    }

    fn get_child_index(&self, value: u8) -> NodeLocation {
        let cur_value_index = value as usize;
        if self.children[cur_value_index].is_some() {
            NodeLocation::Exists(KeyChildIndex{key: 0, child: cur_value_index})
        } else {
            NodeLocation::Insert(KeyChildIndex{key: 0, child: cur_value_index})
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
        self.children[index.child] = Node0::new().add(rest);
        self.size += 1;
        None
    }

    fn upgrade_add(&mut self, values: &[u8]) -> NodeOption {
        unimplemented!()
    }
}

impl Node for Node256 {
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
            if self.children[cur_value_index].is_some() {
                if let Some(child) = self.children[cur_value_index].as_ref() {
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
    fn order_preserved_256_exact_match() {
        let mut node = NodeOption::Some(Box::new(Node4::new()));

        for i in 0..=255 {
            if i % 2 == 0 {
                let upgrade = node.as_mut().unwrap().add(&[i]);
                if upgrade.is_some() {
                    node = upgrade;
                }
            }
        }

        if let Some(n) = node {
            let node256 = n.as_any().downcast_ref::<Node256>().unwrap();
            for (i, c) in node256.children.iter().enumerate() {
                match &c {
                    None => assert_ne!(i % 2, 0),
                    Some(_) => assert_eq!(i % 2, 0),
                    _ => panic!(),
                }
            }
        }
    }
}
