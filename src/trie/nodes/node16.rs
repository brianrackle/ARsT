use std::any::Any;
use crate::trie::nodes::node::{Node, NodeOption, val_cmp};
use crate::trie::nodes::{node0::Node0, node4::Node4, node48::Node48};
use arr_macro::arr;

#[derive(Debug)]
pub struct Node16 {
    pub(crate) keys: [Option<u8>; 16],
    pub(crate) children: [NodeOption; 16],     //value represents value with matching node in children index
    pub(crate) size: usize,
    pub(crate) terminal: bool,
}

impl Default for Node16 {
    fn default() -> Self {
        Node16::new()
    }
}

impl Node16 {
    //keys stored sorted
    pub fn new() -> Self {
        Node16 {
            keys: [None; 16],
            children: arr![None; 16],
            size: 0,
            terminal: false,
        }
    }

    pub fn from(node: &mut Node4) -> Self {
        let mut new_node = Node16::new();
        //sort the keys and original indices of the keys
        //the original indices will be used to create new arrays with the correct order
        let mut ordered_index_value = node.keys.iter().enumerate().collect::<Vec<_>>();
        ordered_index_value.sort_unstable_by(|(_, a), (_, b)| val_cmp(a, b));
        //FIXME should be possible to do this without collecting into a vecto
        let ordered_index = ordered_index_value
            .iter()
            .map(|(index, _)| *index)
            .collect::<Vec<_>>();
        //order arrays based on the ordered indices
        for (target_i, source_i) in ordered_index.iter().enumerate() {
            new_node.keys[target_i] = node.keys[(*source_i)].take();
            new_node.children[target_i] = node.children[*source_i].take();
        }

        new_node.terminal = node.terminal;
        new_node.size = node.size;
        new_node
    }
}

impl Node for Node16 {
    fn add(&mut self, values: &[u8]) -> NodeOption {
        if let Some((first, rest)) = values.split_first() {
            match self
                .keys
                .binary_search_by(|probe| val_cmp(probe, &Some(*first)))
            {
                Ok(index) => {

                    let upgraded_node = self.children[index]
                        .as_mut()
                        .map_or_else(| | Box::new(Node0::new()).add(rest),
                                     |v| v.add(rest));
                    if upgraded_node.is_some() {
                        self.children[index] = upgraded_node;
                    }
                    None
                }
                Err(index) => {
                    //expand to node48 and then add new value
                    if self.is_full() {
                        let mut upgraded_node = Node48::from(self);
                        upgraded_node.add(values);
                        Some(Box::new(upgraded_node))
                    } else {
                        //add value in sorted order to existing Node16 if there is room
                        self.keys[index..].rotate_right(1); //shift right from index
                        self.keys[index] = Some(*first);

                        self.children[index..].rotate_right(1);
                        self.children[index] = Node0::new().add(rest);

                        self.size += 1;
                        None
                    }
                }
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

    //FIXME create utility methods for finding key index and child index to cleanup and reduce copy paste
    fn exists(&self, values: &[u8]) -> bool {
        if let Some((first, rest)) = values.split_first() {
            match self
                .keys
                .binary_search_by(|probe| val_cmp(probe, &Some(*first)))
            {
                Ok(index) => {
                    if let Some(child) = self.children[index].as_ref() {
                        child.exists(rest)
                    } else {
                        false
                    }
                }
                Err(_) => {
                    false
                }
            }
        } else {
            self.terminal
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}