use std::any::Any;
use crate::trie::nodes::node::{Node, NodeOption};
use crate::trie::nodes::node4::Node4;

//TODO performance and memory test storing children directly in keys
#[derive(Debug)]
pub struct Node0 {
    pub(crate) terminal: bool,
}

impl Node0 {
    pub fn new() -> Self {
        Node0 {
            terminal: false,
        }
    }
}

impl Default for Node0 {
    fn default() -> Self {
        Node0::new()
    }
}

impl Node for Node0 {
    fn add(&mut self, values: &[u8]) -> NodeOption {
        let mut new_node = Node4::from(self);
        new_node.add(values);
        Some(Box::new(new_node))
    }

    fn is_full(&self) -> bool {
        true
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn is_terminal(&self) -> bool {
        self.terminal
    }

    //FIXME test looking for word shorter, same, and longer than what tree has
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
}