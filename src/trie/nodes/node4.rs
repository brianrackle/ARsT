use crate::trie::nodes::node::{KeyChildIndex, Node, NodeLocation, NodeOption};
use crate::trie::nodes::{node0::Node0, node16::Node16};
use arr_macro::arr;
use std::any::Any;
use crate::trie::nodes::node::NodeLocation::{Exists, Insert, Upgrade};

#[derive(Debug)]
pub struct Node4 {
    pub(crate) keys: [Option<u8>; 4], //FIXME: Can remove this option and rely only on children option
    pub(crate) children: [NodeOption; 4],
    pub(crate) size: usize,
    pub(crate) terminal: bool,
}

//compression ->
// keys: [Option<Vec<u8>>;4]
// children: [NodeOption; 4]
// [[1,2,3] -> terminal true == "123", [4] -> terminal true, [5] -> terminal true, [6,2,3] -> terminal false == "6234" "6235" "62367"]
//if get_child(value.rest.first) == None
//then store as compressed
// need to think how terminal will be handled with compression. How will terminal be migrated when expanding a compressed node
// a terminal node will always be pointed to by a compressed node, so decompressing will retain the terminal node as the leaf
//make node a generic that implements u8 (uncompressed), and vec<u8> (compressed)
//How to have both uncompressed and compressed in a single array
//optional: can add expand and collapse method to tree

impl Node4 {
    pub fn new() -> Self {
        Node4 {
            keys: [None; 4],
            children: arr![None; 4],
            size: 0,
            terminal: false,
        }
    }

    pub fn from(node: &mut Node0) -> Self {
        let mut new_node = Node4::new();
        new_node.terminal = node.terminal;
        new_node
    }

    //TODO add extension capabilities to node which would allow compression and indexing
    // maybe have a pre-processing and post-processing capabilities along with arbitrary data storage per node
}

impl Default for Node4 {
    fn default() -> Self {
        Node4::new()
    }
}

impl Node for Node4 {
    fn is_full(&self) -> bool {
        self.size == self.children.len()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_terminal(&self) -> bool {
        self.terminal
    }

    fn set_terminal(&mut self, terminal: bool) {
        self.terminal = terminal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_index(&self, value: u8) -> NodeLocation {
        if let Some(index) = self.keys
            .iter()
            .position(|v| v.is_some() && v.unwrap() == value) {
            Exists(KeyChildIndex{key: index, child: index})
        } else if !self.is_full() {
            Insert(KeyChildIndex{key: self.size, child: self.size})
        } else {
            Upgrade
        }
    }

    fn get_child(&self, index: usize) -> Option<&Box<dyn Node>> {
        self.children[index].as_ref()
    }

    fn exists_add(&mut self, index: &KeyChildIndex, rest: &[u8]) -> NodeOption {
        //if None create Node0 and add rest, if Some add content
        let upgraded_node = self.children[index.child]
            .get_or_insert_with(|| Box::new(Node0::new()))
            .as_mut()
            .add(rest);
        if upgraded_node.is_some() {
            self.children[index.child] = upgraded_node;
        }
        None
    }

    fn insert_add(&mut self, index: &KeyChildIndex, first: u8, rest: &[u8]) -> NodeOption {
        //add value to existing Node4 if there is room
        self.keys[index.key] = Some(first);
        let mut new_node = Node0::new();
        self.children[index.child] = new_node.add(rest).or_else(|| Some(Box::new(new_node)));
        self.size += 1;
        None
    }

    fn upgrade_add(&mut self, values: &[u8]) -> NodeOption {
        //expand to node16 and then add new value
        let mut upgraded_node = Node16::from(self);
        upgraded_node.add(values);
        Some(Box::new(upgraded_node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_words_to_node4() {
        let mut node = NodeOption::Some(Box::new(Node0::new()));
        for i in 0..=3 {
            let upgrade = node.as_mut().unwrap().add(&[0, i]);
            if upgrade.is_some() {
                node = upgrade;
            }
        }
        if let Some(root) = node {
            let child = root.as_any().downcast_ref::<Node4>().unwrap().children[0]
                .as_ref()
                .unwrap();
            assert!(child.is_full());
        }
    }
}
