use std::cmp::Ordering;
use std::convert::TryInto;
use std::mem;
use std::ops::Index;
use crate::trie::enums::{Case, Match};
use arr_macro::arr;
use crate::trie::node::NodeEnum::NNone;

// //nodes need to be able to upgrade to a new type
//trienode trait provides
// trait TrieNode {
//     //fn from() -> Self; //creates new trienode based on a lesser sized trienode
//     fn add(&mut self, value: &u8, match_type: &Match);
//     fn exists(&self, c: char) -> Box<&dyn TrieNode>;
//     fn get_size(&self) -> usize;
//     fn get_capacity(&self) -> usize;
//     fn is_terminal(&self) -> bool;
//
//     fn is_full(&self) -> bool {
//         self.get_size() == self.get_capacity()
//     }
//     fn is_empty(&self) -> bool {
//         self.get_size() == 0
//     }
// }

pub trait TrieNode: Sized {
    fn add(mut self, value: &[u8], match_type: &Match) -> NodeEnum {
        match value {
            [] => self.add_empty_case(),
            [only_value] => self.add_single_case(only_value, match_type),
            [first_value, remaining_values @..] => self.add_multiple_case(first_value, remaining_values, match_type)
        }
    }
    fn add_empty_case(self) -> NodeEnum;
    fn add_single_case(self, only_value :&u8, match_type :&Match) -> NodeEnum;
    fn add_multiple_case(self, first_value :&u8, remaining_values :&[u8], match_type :&Match) -> NodeEnum;
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_terminal(&self) -> bool;
}

pub struct Node0 {
    terminal: bool
}

pub struct Node4 {
keys: [Option<u8>; 4],
//Can remove this option and rely only on children option
children: Box<[NodeEnum; 4]>,
size: usize,
terminal: bool,
}

pub struct Node16 {
keys: [Option<u8>; 16],
//value represents value with matching node in children index
children: Box<[NodeEnum; 16]>,
size: usize,
terminal: bool,
}

pub struct Node48 {
keys: [Option<u8>; 256],
//index represents value, and value represents index in children
children: Box<[NodeEnum; 48]>,
size: usize,
terminal: bool,
}

pub struct Node256 {
children: Box<[NodeEnum; 256]>,
size: usize,
terminal: bool,
}

//see: https://www.the-paper-trail.org/post/art-paper-notes/
pub enum NodeEnum {
    NNone,
    N0(Node0),     //is_terminal should always be true for Node0, not sure if this is needed used only when leaf node is created
    N4(Node4),
    N16(Node16),
    N48(Node48),
    N256(Node256)
}

impl Default for NodeEnum {
    fn default() -> Self {
        NodeEnum::NNone
    }
}

impl Node0 {
    pub fn new(terminal :bool) -> Self {
        Node0 {
            terminal
        }
    }
}

impl TrieNode for Node0 {
    fn add_empty_case(mut self) -> NodeEnum {
        self.terminal = true;
        NodeEnum::N0(self)
    }

    fn add_single_case(mut self, only_value :&u8, match_type :&Match) -> NodeEnum {
        let mut keys = [None; 4];
        let mut children = Box::new(arr![NodeEnum::NNone; 4]);
        keys[0] = Some(*only_value);
        children[0] = Node0::new(true).add(&[], match_type); //current node is last value so next node is terminal
        NodeEnum::N4(Node4 {
            keys,
            children,
            size: 1,
            terminal: self.terminal //update this if its last value in string
        })
    }

    fn add_multiple_case(mut self, first_value :&u8, remaining_values :&[u8], match_type :&Match) -> NodeEnum {
        let mut keys = [None; 4];
        let mut children = Box::new(arr![NodeEnum::NNone; 4]);
        keys[0] = Some(*first_value);
        children[0] = Node4::new().add(remaining_values, match_type);
        NodeEnum::N4(Node4 {
            keys,
            children,
            size: 1, //can be used to remove need for Option
            terminal: self.terminal //update this if its last value in string
        })
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
}

impl Node4 {
    pub fn new() -> Self {
        Node4 {
            keys: [None; 4],
            children: Box::new(arr![NodeEnum::NNone; 4]),
            size: 0,
            terminal: false
        }
    }
}

impl TrieNode for Node4 {
    fn add_empty_case(mut self) -> NodeEnum {
        self.terminal = true;
        NodeEnum::N4(self)
    }

    fn add_single_case(mut self, only_value :&u8, match_type :&Match) -> NodeEnum {
        // let mut keys = [None; 4];
        // let mut children = Box::new(arr![NodeEnum::NNone; 4]);


        //check if value exists already
        if let Some(index) = self.keys.iter().position(|v| v.is_some() && v.unwrap() == *only_value) {
            //self.children[index].add()
            todo!()
        } else { //value doesnt exist yet

            if self.is_full() {
                //expand to node16
                Node16::from(self).add_single_case(only_value, match_type)
            } else {
                self.keys[self.size] = Some(*only_value); //
                //children
                todo!()
            }
        }
    }

    fn add_multiple_case(mut self, first_value :&u8, remaining_values :&[u8], match_type :&Match) -> NodeEnum {
        // let mut keys = [None; 4];
        // let mut children = Box::new(arr![NodeEnum::NNone; 4]);
        // keys[0] = Some(*first_value);
        // children[0] = Node4::new().add(remaining_values, match_type);
        // NodeEnum::N4(Node4 {
        //     keys: keys,
        //     children: children,
        //     size: 1, //can be used to remove need for Option
        //     terminal: false //update this if its last value in string
        // })
        todo!()
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
}

//     pub fn add(self, value: &[u8], match_type: &Match) -> NodeEnum {
//         // let mut keys = [None; 4];
//         // let mut children = Box::new(arr![NodeEnum::NNone; 4]);
//         match value {
//             [] => {
//                 //should not occur because node starts from new_node0
//                 unimplemented!()
//             }
//             [only] => {
//                 // keys[0] = Some(*only);
//                 // children[0] = NodeEnum::N0(Node0::new());
//             }
//             [first, rest @..] => {
//                 // keys[0] = Some(*first);
//                 // children[0] = NodeEnum::N4(Node4::new());
//             }
//         }
//
//         todo!()
//         // NodeEnum::N4(Node4 {
//         //     keys: keys,
//         //     children: children,
//         //     size: 1, //can be used to remove need for Option
//         //     terminal: false //update this if its last value in string
//         // })
//     }
// }

impl Node16 {
    //keys stored sorted
    pub fn new() -> Self {
        Node16 {
            keys: [None; 16],
            children: Box::new(arr![NodeEnum::NNone; 16]),
            size: 0,
            terminal: false
        }
    }

    //TODO: should be node + value?
    pub fn from(mut node :Node4) -> Self {
        let mut new_node = Node16::new();

        for i in 0..node.keys.len() {
            new_node.keys[i] = node.keys[i].take();
            new_node.children[i] = mem::replace(&mut node.children[i], NNone); //same function used by take
        }

        // let mut i = Some(Node4::new());
        // let j = i.take();
        // //
        // let mut zipped: Vec<_> = node.keys
        //     .iter()
        //     .zip(node.children.iter())
        //     .collect();
        //
        // zipped.sort_unstable_by(|a, b| {
        //     if a.0.is_none() && b.0.is_none() { Ordering::Equal }
        //     else if a.0.is_none() && b.0.is_some() { Ordering::Less }
        //     else if a.0.is_some() && b.0.is_none() { Ordering::Greater }
        //     else { a.0.unwrap().cmp(&b.0.unwrap()) }
        // });
        //
        // let mut new_node = Node16::new();
        // for (i,v) in zipped.iter().collect::<Vec<_>>().enumerate() {
        //     new_node.keys[i] = *v.0;
        //     new_node.children[i] = *v.1;
        // }
        todo!()
        // Node16 {
        //     keys: zipped.iter().map(|mut v| v.0.take()).collect::<Vec<_>>().try_into().expect("Vector does not fit array"),
        //     children: zipped.iter().map(|v| v.1).collect(),
        //     size: node.size,
        //     terminal: node.terminal
        // }
    }
}

impl TrieNode for Node16 {
    fn add_empty_case(self) -> NodeEnum {
        todo!()
    }

    fn add_single_case(self, only_value: &u8, match_type: &Match) -> NodeEnum {
        todo!()
    }

    fn add_multiple_case(self, first_value: &u8, remaining_values: &[u8], match_type: &Match) -> NodeEnum {
        todo!()
    }

    fn is_full(&self) -> bool {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn is_terminal(&self) -> bool {
        todo!()
    }
}

impl Node48 {
    pub fn new() -> Self {
        Node48 {
            keys: [None; 256],
            children: Box::new(arr![NodeEnum::NNone; 48]),
            size: 0,
            terminal: false
        }
    }

    pub fn from(node :Node16) -> Self {
        todo!()
    }
}

impl TrieNode for Node48 {
    fn add_empty_case(self) -> NodeEnum {
        todo!()
    }

    fn add_single_case(self, only_value: &u8, match_type: &Match) -> NodeEnum {
        todo!()
    }

    fn add_multiple_case(self, first_value: &u8, remaining_values: &[u8], match_type: &Match) -> NodeEnum {
        todo!()
    }

    fn is_full(&self) -> bool {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn is_terminal(&self) -> bool {
        todo!()
    }
}

impl Node256 {
    pub fn new() -> Self {
        Node256 {
            children: Box::new(arr![NodeEnum::NNone; 256]),
            size: 0,
            terminal: false
        }
    }

    pub fn from(node :Node48) -> Self {
        todo!()
    }
}

impl TrieNode for Node256 {
    fn add_empty_case(self) -> NodeEnum {
        todo!()
    }

    fn add_single_case(self, only_value: &u8, match_type: &Match) -> NodeEnum {
        todo!()
    }

    fn add_multiple_case(self, first_value: &u8, remaining_values: &[u8], match_type: &Match) -> NodeEnum {
        todo!()
    }

    fn is_full(&self) -> bool {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn is_terminal(&self) -> bool {
        todo!()
    }
}

impl NodeEnum {
    pub fn add(self, value: &[u8], match_type: &Match) -> Self {
        match self {
            NodeEnum::NNone => NodeEnum::N0(Node0::new(false)),
            NodeEnum::N0(n) => n.add(value, match_type),
            NodeEnum::N4(n) => n.add(value, match_type),
            NodeEnum::N16(n) => n.add(value, match_type),
            NodeEnum::N48(n) => n.add(value, match_type),
            NodeEnum::N256(n) => n.add(value, match_type)
        }
    }

    fn add_empty_case(self) -> NodeEnum {
        match self {
            NodeEnum::NNone => NodeEnum::N0(Node0::new(false)),
            NodeEnum::N0(n) => n.add_empty_case(),
            NodeEnum::N4(n) => n.add_empty_case(),
            NodeEnum::N16(n) => n.add_empty_case(),
            NodeEnum::N48(n) => n.add_empty_case(),
            NodeEnum::N256(n) => n.add_empty_case()
        }
    }

    fn add_single_case(self, only_value :&u8, match_type :&Match) -> NodeEnum {
        match self {
            NodeEnum::NNone => NodeEnum::N0(Node0::new(false)),
            NodeEnum::N0(n) => n.add_single_case(only_value, match_type),
            NodeEnum::N4(n) => n.add_single_case(only_value, match_type),
            NodeEnum::N16(n) => n.add_single_case(only_value, match_type),
            NodeEnum::N48(n) => n.add_single_case(only_value, match_type),
            NodeEnum::N256(n) => n.add_single_case(only_value, match_type)
        }
    }

    fn add_multiple_case(self, first_value :&u8, remaining_values :&[u8], match_type :&Match) -> NodeEnum {
        match self {
            NodeEnum::NNone => NodeEnum::N0(Node0::new(false)),
            NodeEnum::N0(n) => n.add_multiple_case(first_value, remaining_values, match_type),
            NodeEnum::N4(n) => n.add_multiple_case(first_value, remaining_values, match_type),
            NodeEnum::N16(n) => n.add_multiple_case(first_value, remaining_values, match_type),
            NodeEnum::N48(n) => n.add_multiple_case(first_value, remaining_values, match_type),
            NodeEnum::N256(n) => n.add_multiple_case(first_value, remaining_values, match_type)
        }
    }

}


//
// impl NodeEnum {
//     //should consume self so that it can be upgraded if needed
//     fn add(self, value: &str, match_type: &Match) -> Self {
//         //needs to upgrade if capacity is reached
//         //insert into next open space in array until filled to capacity
//         match self {
//             NodeEnum::Node0 => {
//                 NodeEnum::Node4 {
//                     keys: [None; 4],
//                     children: [None; 4],
//                     size: 0,
//                     capacity: 4,
//                     terminal: false,
//                 }
//                 // let return_node = NodeEnum::Node4::new();
//                 // return_node
//             }
//             NodeEnum::Node4 { keys, children, count, capacity, terminal } => {
//                 if !self.is_full() { //add to existing
//                     // match value {
//                     //     [first, rest @ ..] =>
//                     // }
//                     // if keys[count]..is_some() {
//                     //
//                     // }
//
//                 } else { //else upgrade to node16
//
//                 }
//                 todo!()
//             }
//             NodeEnum::Node16 { .. } => {
//                 todo!()
//             }
//             NodeEnum::Node48 { .. } => {
//                 todo!()
//             }
//             NodeEnum::Node256 { .. } => {
//                 todo!()
//             }
//         }
//     }
//
//     fn exists(&self, c: char) -> Option<&NodeEnum> {
//         //linearly search array no early exit to allow optimization
//         todo!()
//     }
//
//     fn is_terminal(&self) -> bool {
//         todo!()
//     }
//
//     fn is_full(&self) -> bool {
//         match self {
//             Node0 => true,
//             other => self.size() != self.capacity()
//         }
//     }
//
//     fn is_empty(&self) -> bool {
//         match self {
//             Node0 => false,
//             NodeEnum::Node4{count, ..} => count == 0u8, //this is dumb, should implement as traits
//             NodeEnum::Node16 {count, ..} => count == 0,
//             NodeEnum::Node48 {count, ..} => count == 0,
//             NodeEnum::Node256 {count, ..} => count == 0
//         }
//     }
//
//     fn size(&self) -> usize {
//         match self {
//             Node0 => 0,
//             _ => self.count
//         }
//     }
//
//     fn capacity(&self) -> usize {
//         match self {
//             Node0 => 0,
//             NodeEnum::Node4{capacity, ..} => capacity, //this is dumb, should implement as traits
//             NodeEnum::Node16 {capacity, ..} => capacity,
//             NodeEnum::Node48 {capacity, ..} => capacity,
//             NodeEnum::Node256 {capacity, ..} => capacity
//         }
//     }
// }

// pub struct Node16 {
//     keys: [Option<u8>; 16], //value represents value with matching node in children index
//     children: [Option<NodeEnum>; 16],
//     terminal: bool,
// }

// impl TrieNode for Node16 {
//     fn new() -> Self {
//         todo!()
//     }
//
//     fn add(&mut self, value: &str, match_type: &Match) {
//         //needs to upgrade if capacity is reached
//         //insert into array in sorted order until filled to capacity
//         todo!()
//     }
//
//     fn exists(&self, c: char) -> Option<&NodeEnum> {
//         //binary search array
//         todo!()
//     }
// }

// pub struct Node48 {
//     keys: [u8; 256], //index represents value, and value represents index in children
//     children: [NodeEnum; 48],
//     terminal: bool,
// }

// impl TrieNode for Node48 {
//     fn new() -> Self {
//         todo!()
//     }
//
//     fn add(&mut self, value: &str, match_type: &Match) {
//         //needs to upgrade if capacity is reached
//         //insert into array in sorted order until filled to capacity
//         todo!()
//     }
//
//     fn exists(&self, c: char) -> Option<&NodeEnum> {
//         //binary search array
//         todo!()
//     }
// }

// pub struct Node256 {
//     children: [NodeEnum; 256],
//     terminal: bool,
// }

// impl TrieNode for Node256 {
//     fn new() -> Self {
//         todo!()
//     }
//
//     fn add(&mut self, value: &str, match_type: &Match) {
//         //needs to upgrade if capacity is reached
//         //insert into array in sorted order until filled to capacity
//         todo!()
//     }
//
//     fn exists(&self, c: char) -> Option<&NodeEnum> {
//         //binary search array
//         todo!()
//     }
// }

pub type Link = Option<Box<Node>>;
pub struct Node {
    children: [Link; 257],
}

impl Node {
    //TODO: make variable length based off settings
    pub fn new() -> Self {
        Node {
            children: arr![None; 257],
        }
    }

    pub fn get_node(&self, i: usize) -> Option<&Node> {
        self.children[i].as_ref().map(|c| c.as_ref())
    }

    pub fn add(&mut self, value: &[u8], match_type: &Match) {
        match match_type {
            Match::Exact | Match::Prefix => {
                let mut cur = self;
                for c in value {
                    cur = cur.children[(*c) as usize].get_or_insert(Box::new(Node::new()));
                }
                //add terminal char when match is exact
                if let Match::Exact = match_type {
                    cur.children[257 - 1] = Some(Box::new(Node::new()))
                }
            }
            Match::PrefixPostfix => {
                //takes 0+n first characters off string
                let mut cur: &mut Node;
                for j in 0..value.len() {
                    cur = self;
                    for c in value[j..].iter() {
                        cur = cur.children[(*c) as usize].get_or_insert(Box::new(Node::new()));
                    }
                }
            }
        }
    }

    pub fn exists(&self, c: u8) -> Option<&Node> {
        self.children[c as usize].as_ref().map(|c| c.as_ref())
    }
}
