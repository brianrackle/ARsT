use crate::trie::nodes::node::{KeyChildIndex, Node, NodeLocation, NodeOption};
use crate::trie::nodes::node4::Node4;
use std::any::Any;
use crate::trie::nodes::node::NodeLocation::{Exists, Insert};

//TODO performance and memory test storing children directly in keys
#[derive(Debug)]
pub struct Node0 {
    pub(crate) terminal: bool,
}

impl Node0 {
    pub fn new() -> Self {
        Node0 { terminal: false }
    }
}

impl Default for Node0 {
    fn default() -> Self {
        Node0::new()
    }
}

impl Node for Node0 {
    fn is_full(&self) -> bool {
        true
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn is_terminal(&self) -> bool {
        self.terminal
    }

    //TODO test looking for word shorter, same, and longer than what tree has
    fn exists(&self, values: &[u8]) -> bool {
        //if more values exists then a match cant exist
        if let Some((_first, _rest)) = values.split_first() {
            false
        } else {
            self.terminal
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_index(&self, value: u8) -> NodeLocation {
        NodeLocation::Upgrade
    }

    fn exists_add(&mut self, index: &KeyChildIndex, rest: &[u8]) -> NodeOption {
        unimplemented!()
    }

    fn insert_add(&mut self, index: &KeyChildIndex, first: u8, rest: &[u8]) -> NodeOption {
        unimplemented!()
    }

    fn upgrade_add(&mut self, values: &[u8]) -> NodeOption {
        let mut new_node = Node4::from(self);
        new_node.add(values);
        Some(Box::new(new_node))
    }

    fn set_terminal(&mut self, terminal: bool) {
        self.terminal = terminal
    }

    fn get_child(&self, index: usize) -> Option<&Box<dyn Node>> {
        None.as_ref()
    }
}
