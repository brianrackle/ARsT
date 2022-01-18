use crate::trie::enums::{Case, Match};
use arr_macro::arr;

// //nodes need to be able to upgrade to a new type
//trienode trait provides
trait TrieNode {
    //fn from() -> Self; //creates new trienode based on a lesser sized trienode
    fn add(&mut self, value: &u8, match_type: &Match);
    fn exists(&self, c: char) -> Box<&dyn TrieNode>;
    fn get_size(&self) -> usize;
    fn get_capacity(&self) -> usize;
    fn is_terminal(&self) -> bool;

    fn is_full(&self) -> bool {
        self.get_size() == self.get_capacity()
    }
    fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
}


pub struct Node0 {} //is_terminal should always be true for Node0, not sure if this is needed used only when leaf node is created

pub struct Node4 {
keys: [Option<u8>; 4],
//Can remove this option and rely only on children option
children: Box<[NodeEnum; 4]>,
size: u8,
terminal: bool,
}

pub struct Node16 {
keys: [Option<u8>; 16],
//value represents value with matching node in children index
children: Box<[NodeEnum; 16]>,
size: u8,
terminal: bool,
}

pub struct Node48 {
keys: [Option<u8>; 256],
//index represents value, and value represents index in children
children: Box<[NodeEnum; 48]>,
size: u8,
terminal: bool,
}

pub struct Node256 {
children: Box<[NodeEnum; 256]>,
size: u8,
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
    pub fn new() -> Self {
        Node0{}
    }

    fn add_cases(
        self,
        value :&[u8],
        match_type :&Match,
        empty_case :fn(Node0) -> NodeEnum,
        single_case :fn(Node0, &u8, &Match) -> NodeEnum,
        multiple_case :fn(Node0, &u8, &[u8], &Match) -> NodeEnum) -> NodeEnum {
        match value {
            [] => empty_case(self),
            [only] => single_case(self, only, match_type),
            [first, rest @..] => multiple_case(self, first, rest, match_type)
        }
    }

    fn add_empty_case(self) -> NodeEnum {
        NodeEnum::N0(self)
    }

    fn add_single_case(self, only_value :&u8, match_type :&Match) -> NodeEnum {
        let mut keys = [None; 4];
        let mut children = Box::new(arr![NodeEnum::NNone; 4]);
        keys[0] = Some(*only_value);
        children[0] = Node0::new().add(&[], match_type);
        NodeEnum::N4(Node4 {
            keys: keys,
            children: children,
            size: 1, //can be used to remove need for Option
            terminal: false //update this if its last value in string
        })
    }

    fn add_multiple_case(self, first_value :&u8, remaining_values :&[u8], match_type :&Match) -> NodeEnum {
        let mut keys = [None; 4];
        let mut children = Box::new(arr![NodeEnum::NNone; 4]);
        keys[0] = Some(*first_value);
        children[0] = Node4::new().add(remaining_values, match_type);
        NodeEnum::N4(Node4 {
            keys: keys,
            children: children,
            size: 1, //can be used to remove need for Option
            terminal: false //update this if its last value in string
        })
    }

    //could then move add_cases to trait and have structs implement empty_case, single_case, multiple_case
    pub fn add(self, value: &[u8], match_type: &Match) -> NodeEnum {
        self.add_cases(value, match_type, Node0::add_empty_case, Node0::add_single_case, Node0::add_multiple_case)
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

    pub fn add(self, value: &[u8], match_type: &Match) -> NodeEnum {
        // let mut keys = [None; 4];
        // let mut children = Box::new(arr![NodeEnum::NNone; 4]);
        match value {
            [] => {
                //should not occur because node starts from new_node0
                unimplemented!()
            }
            [only] => {
                // keys[0] = Some(*only);
                // children[0] = NodeEnum::N0(Node0::new());
            }
            [first, rest @..] => {
                // keys[0] = Some(*first);
                // children[0] = NodeEnum::N4(Node4::new());
            }
        }

        todo!()
        // NodeEnum::N4(Node4 {
        //     keys: keys,
        //     children: children,
        //     size: 1, //can be used to remove need for Option
        //     terminal: false //update this if its last value in string
        // })
    }
}

impl Node16 {
    pub fn new() -> Self {
        Node16 {
            keys: [None; 16],
            children: Box::new(arr![NodeEnum::NNone; 16]),
            size: 0,
            terminal: false
        }
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
}

impl Node256 {
    pub fn new() -> Self {
        Node256 {
            children: Box::new(arr![NodeEnum::NNone; 256]),
            size: 0,
            terminal: false
        }
    }
}

impl NodeEnum {
    pub fn add(self, value: &[u8], match_type: &Match) -> Self {
        match self {
            NodeEnum::NNone => NodeEnum::N0(Node0{}),
            NodeEnum::N0(n) => n.add(value, match_type),
            NodeEnum::N4 { .. } => {todo!()}
            NodeEnum::N16 { .. } => {todo!()}
            NodeEnum::N48 { .. } => {todo!()}
            NodeEnum::N256 { .. } => {todo!()}
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
