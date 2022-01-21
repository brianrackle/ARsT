use std::cmp::Ordering;
use std::mem;
use crate::trie::enums::{Match};
use arr_macro::arr;
use crate::trie::node::NodeEnum::NNone;

pub trait TrieNode: Sized {
    fn add(mut self, value: &[u8], match_type: &Match) -> NodeEnum {
        match value {
            [] => self.add_empty_case(),
            [cur_value] => self.add_single_case(cur_value, match_type),
            [cur_value, remaining_values @..] => self.add_multiple_case(cur_value, remaining_values, match_type)
        }
    }
    fn add_empty_case(self) -> NodeEnum;
    fn add_single_case(self, cur_value :&u8, match_type :&Match) -> NodeEnum;
    fn add_multiple_case(self, cur_value :&u8, remaining_values :&[u8], match_type :&Match) -> NodeEnum;
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_terminal(&self) -> bool;
}

fn option_u8_cmp(a :&Option<u8>, b :&Option<u8>) -> Ordering {
    if a.is_none() && b.is_none() { Ordering::Equal } else if a.is_none() && b.is_some() { Ordering::Greater } else if a.is_some() && b.is_none() { Ordering::Less } else { a.unwrap().cmp(&b.unwrap()) }
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


impl NodeEnum {
    pub fn take(&mut self) -> NodeEnum {
        mem::replace(self, NNone)
    }
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

    //TODO: review all new(terminal) usages to make sure they are correctly assigned
    fn add_single_case(mut self, cur_value :&u8, match_type :&Match) -> NodeEnum {
        //need to upgrade to Node4 to add a value
        Node4::from(self).add_single_case(cur_value, match_type)
    }

    fn add_multiple_case(mut self, cur_value :&u8, remaining_values :&[u8], match_type :&Match) -> NodeEnum {
        //need to upgrade to Node4 to add a value
        Node4::from(self).add_multiple_case(cur_value, remaining_values, match_type)
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
    pub fn new(terminal :bool) -> Self {
        Node4 {
            keys: [None; 4],
            children: Box::new(arr![NodeEnum::NNone; 4]),
            size: 0,
            terminal
        }
    }

    pub fn from(mut node :Node0) -> Self {
        Node4::new(node.terminal)
    }
}

impl TrieNode for Node4 {
    //TODO: will need to be enhanced in the future to support the match types
    fn add_empty_case(mut self) -> NodeEnum {
        self.terminal = true;
        NodeEnum::N4(self)
    }

    fn add_single_case(mut self, cur_value :&u8, match_type :&Match) -> NodeEnum {
        //check if value exists already
        if let Some(index) = self.keys.iter().position(|v| v.is_some() && v.unwrap() == *cur_value) {
            //TODO: consider implementing a swap function or changing add to mutable borrow
            self.children[index] = self.children[index].take().add_empty_case();
            NodeEnum::N4(self)
        } else { //value doesnt exist yet
            //expand to node16 and then add new value
            if self.is_full() {
                Node16::from(self).add_single_case(cur_value, match_type)
            } else { //add value to existing Node4 if there is room
                self.keys[self.size] = Some(*cur_value);
                self.children[self.size] = Node0::new(false).add_empty_case();
                NodeEnum::N4(self)
            }
        }
    }

    fn add_multiple_case(mut self, cur_value :&u8, remaining_values :&[u8], match_type :&Match) -> NodeEnum {
        //check if value exists already
        if let Some(index) = self.keys.iter().position(|v| v.is_some() && v.unwrap() == *cur_value) {
            self.children[index] = self.children[index].take().add(remaining_values, match_type);
            NodeEnum::N4(self)
        } else { //value doesnt exist yet
            //expand to node16 and then add new value
            if self.is_full() {
                //TODO: consider at alternate implementation which joins first and remaining into a vector but allows for using add instead of specific
                // Node16::from(self).add(&[&[*cur_value], remaining_values].concat(), match_type);
                Node16::from(self).add_multiple_case(cur_value, remaining_values, match_type)
            } else { //add value to existing Node4 if there is room
                self.keys[self.size] = Some(*cur_value);
                self.children[self.size] = Node4::new(false).add(remaining_values, match_type);
                NodeEnum::N4(self)
            }
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
}

impl Node16 {
    //keys stored sorted
    pub fn new(terminal :bool) -> Self {
        Node16 {
            keys: [None; 16],
            children: Box::new(arr![NodeEnum::NNone; 16]),
            size: 0,
            terminal
        }
    }

    pub fn from(mut node :Node4) -> Self {
        let mut new_node = Node16::new(node.terminal);
        //sort the keys and original indices of the keys
        //the original indices will be used to create new arrays with the correct order
        let mut ordered_index_value = new_node.keys.iter().enumerate().collect::<Vec<_>>();
        ordered_index_value.sort_unstable_by(|(_, a), (_, b)| option_u8_cmp(a,b));
        let ordered_index = ordered_index_value.iter().map(|(index, _)| *index).collect::<Vec<_>>();
        //order arrays based on the ordered indices
        for (target_i, source_i) in ordered_index.iter().enumerate() {
            new_node.keys[target_i] = node.keys[(*source_i)].take();
            new_node.children[target_i] = node.children[*source_i].take(); //same function used by Option::take to replace element
        }
        new_node.size = node.size;
        new_node.terminal = node.terminal;
        new_node
    }
}

impl TrieNode for Node16 {
    fn add_empty_case(mut self) -> NodeEnum {
        self.terminal = true;
        NodeEnum::N16(self)
    }

    fn add_single_case(mut self, cur_value: &u8, match_type: &Match) -> NodeEnum {
        //check if value exists already
        //binary search
        match self.keys.binary_search_by(|probe| option_u8_cmp(probe, &Some(*cur_value))) {
            Ok(index) => {
                self.children[index] = self.children[index].take().add(&[], match_type);
                NodeEnum::N16(self)
            }
            Err(index) => {
                //expand to node48 and then add new value
                if self.is_full() {
                    Node48::from(self).add_single_case(cur_value, match_type)
                } else { //add value in sorted order to existing Node16 if there is room
                    self.keys[index..].rotate_right(1); //shift right from index
                    self.keys[index] = Some(*cur_value);

                    self.children[index..].rotate_right(1);
                    self.children[index] = Node0::new(false).add_empty_case(); //done this way for consistency of implementations
                    NodeEnum::N16(self)
                }
            }
        }
    }

    //TODO: rename cur_value and cur_value to operating_value or something like that
    fn add_multiple_case(mut self, cur_value: &u8, remaining_values: &[u8], match_type: &Match) -> NodeEnum {
        //check if value exists already
        //binary search
        match self.keys.binary_search_by(|probe| option_u8_cmp(probe, &Some(*cur_value))) {
            Ok(index) => {
                self.children[index] = self.children[index].take().add(&[], match_type);
                NodeEnum::N16(self)
            }
            Err(index) => {
                //expand to node48 and then add new value
                if self.is_full() {
                    Node48::from(self).add_single_case(cur_value, match_type)
                } else { //add value in sorted order to existing Node16 if there is room
                    self.keys[index..].rotate_right(1); //shift right from index
                    self.keys[index] = Some(*cur_value);

                    self.children[index..].rotate_right(1);
                    self.children[index] = Node0::new(false).add_empty_case(); //done this way for consistency of implementations
                    NodeEnum::N16(self)
                }
            }
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
}

impl Node48 {
    pub fn new(terminal :bool) -> Self {
        Node48 {
            keys: [None; 256],
            children: Box::new(arr![NodeEnum::NNone; 48]),
            size: 0,
            terminal
        }
    }

    pub fn from(node :Node16) -> Self {
        todo!()
    }
}

impl TrieNode for Node48 {
    fn add_empty_case(mut self) -> NodeEnum {
        self.terminal = true;
        NodeEnum::N48(self)
    }

    fn add_single_case(mut self, cur_value: &u8, match_type: &Match) -> NodeEnum {
        todo!()
    }

    fn add_multiple_case(mut self, cur_value: &u8, remaining_values: &[u8], match_type: &Match) -> NodeEnum {
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

impl Node256 {
    pub fn new(terminal :bool) -> Self {
        Node256 {
            children: Box::new(arr![NodeEnum::NNone; 256]),
            size: 0,
            terminal
        }
    }

    pub fn from(node :Node48) -> Self {
        todo!()
    }
}

impl TrieNode for Node256 {
    fn add_empty_case(mut self) -> NodeEnum {
        self.terminal = true;
        NodeEnum::N256(self)
    }

    fn add_single_case(mut self, cur_value: &u8, match_type: &Match) -> NodeEnum {
        todo!()
    }

    fn add_multiple_case(mut self, cur_value: &u8, remaining_values: &[u8], match_type: &Match) -> NodeEnum {
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

    fn add_single_case(self, cur_value :&u8, match_type :&Match) -> NodeEnum {
        match self {
            NodeEnum::NNone => NodeEnum::N0(Node0::new(false)),
            NodeEnum::N0(n) => n.add_single_case(cur_value, match_type),
            NodeEnum::N4(n) => n.add_single_case(cur_value, match_type),
            NodeEnum::N16(n) => n.add_single_case(cur_value, match_type),
            NodeEnum::N48(n) => n.add_single_case(cur_value, match_type),
            NodeEnum::N256(n) => n.add_single_case(cur_value, match_type)
        }
    }

    fn add_multiple_case(self, cur_value :&u8, remaining_values :&[u8], match_type :&Match) -> NodeEnum {
        match self {
            NodeEnum::NNone => NodeEnum::N0(Node0::new(false)),
            NodeEnum::N0(n) => n.add_multiple_case(cur_value, remaining_values, match_type),
            NodeEnum::N4(n) => n.add_multiple_case(cur_value, remaining_values, match_type),
            NodeEnum::N16(n) => n.add_multiple_case(cur_value, remaining_values, match_type),
            NodeEnum::N48(n) => n.add_multiple_case(cur_value, remaining_values, match_type),
            NodeEnum::N256(n) => n.add_multiple_case(cur_value, remaining_values, match_type)
        }
    }

}

//Old implementation starts here
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
