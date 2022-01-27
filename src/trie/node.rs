use crate::trie::enums::Match;
use crate::trie::node::NodeEnum::{N256, NNone};
use arr_macro::arr;
use std::cmp::Ordering;
use std::mem;

pub trait TrieNode: Sized {
    fn add(mut self, value: &[u8], match_type: &Match) -> NodeEnum {
        match value {
            [] => self.add_empty_case(match_type),
            [cur_value] => self.add_single_case(cur_value, match_type),
            [cur_value, remaining_values @ ..] => {
                self.add_multiple_case(cur_value, remaining_values, match_type)
            }
        }
    }
    fn add_empty_case(self, match_type :&Match) -> NodeEnum;
    fn add_single_case(self, cur_value: &u8, match_type: &Match) -> NodeEnum;
    fn add_multiple_case(
        self,
        cur_value: &u8,
        remaining_values: &[u8],
        match_type: &Match,
    ) -> NodeEnum;
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_terminal(&self) -> bool;
}

#[derive(Debug)]
pub struct Node0 {
    terminal: bool,
}

#[derive(Debug)]
pub struct Node4 {
    keys: [Option<u8>; 4], //Can remove this option and rely only on children option
    children: Box<[NodeEnum; 4]>,
    size: usize,
    terminal: bool,
}

#[derive(Debug)]
pub struct Node16 {
    keys: [Option<u8>; 16],
    //value represents value with matching node in children index
    children: Box<[NodeEnum; 16]>,
    size: usize,
    terminal: bool,
}

#[derive(Debug)]
pub struct Node48 {
    keys: [Option<u8>; 256],
    //index represents value, and value represents index in children
    children: Box<[NodeEnum; 48]>,
    size: usize,
    terminal: bool,
}

#[derive(Debug)]
pub struct Node256 {
    children: Box<[NodeEnum; 256]>,
    size: usize,
    terminal: bool,
}

//see: https://www.the-paper-trail.org/post/art-paper-notes/
#[derive(Debug)]
pub enum NodeEnum {
    NNone,
    N0(Node0), //is_terminal should always be true for Node0, not sure if this is needed used only when leaf node is created
    N4(Node4),
    N16(Node16),
    N48(Node48), //FIXME consider boxing large variants
    N256(Node256),
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
    pub fn new() -> Self {
        Node0 { terminal: false }
    }
}

impl TrieNode for Node0 {
    fn add_empty_case(mut self, match_type :&Match) -> NodeEnum {
        self.terminal = true;
        NodeEnum::N0(self)
    }

    //TODO: review all new(terminal) usages to make sure they are correctly assigned
    fn add_single_case(mut self, cur_value: &u8, match_type: &Match) -> NodeEnum {
        //need to upgrade to Node4 to add a value
        Node4::from(self).add_single_case(cur_value, match_type)
    }

    fn add_multiple_case(
        mut self,
        cur_value: &u8,
        remaining_values: &[u8],
        match_type: &Match,
    ) -> NodeEnum {
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
    pub fn new() -> Self {
        Node4 {
            keys: [None; 4],
            children: Box::new(arr![NodeEnum::NNone; 4]),
            size: 0,
            terminal: false,
        }
    }

    pub fn from(mut node: Node0) -> Self {
        let mut new_node = Node4::new();
        new_node.terminal = node.terminal;
        new_node.size = 0;
        new_node
    }
}

impl TrieNode for Node4 {
    //TODO: will need to be enhanced in the future to support the match types
    fn add_empty_case(mut self, match_type :&Match) -> NodeEnum {
        self.terminal = true;
        NodeEnum::N4(self)
    }

    fn add_single_case(mut self, cur_value: &u8, match_type: &Match) -> NodeEnum {
        //check if value exists already
        if let Some(index) = self
            .keys
            .iter()
            .position(|v| v.is_some() && v.unwrap() == *cur_value)
        {
            //TODO: consider implementing a swap function or changing add to mutable borrow
            self.children[index] = self.children[index].take().add(&[], match_type);
            NodeEnum::N4(self)
        } else {
            //value doesnt exist yet
            //expand to node16 and then add new value
            if self.is_full() {
                Node16::from(self).add(&[*cur_value], match_type)
            } else {
                //add value to existing Node4 if there is room
                self.keys[self.size] = Some(*cur_value);
                self.children[self.size] = Node0::new().add(&[], match_type);

                self.size += 1;
                NodeEnum::N4(self)
            }
        }
    }

    fn add_multiple_case(
        mut self,
        cur_value: &u8,
        remaining_values: &[u8],
        match_type: &Match,
    ) -> NodeEnum {
        //check if value exists already
        if let Some(index) = self.keys.iter().position(|v| v.is_some() && v.unwrap() == *cur_value)
        {
            self.children[index] = self.children[index].take().add(remaining_values, match_type);
            NodeEnum::N4(self)
        } else if self.is_full() { //value doesnt exist yet
            //expand to node16 and then add new value

            //TODO: consider this alternate implementation which joins first and remaining into a vector but allows for using add instead of specific
            // let mut t = remaining_values.to_vec();
            // t.insert(0, *cur_value);
            // Node16::from(self).add(&t, match_type);
            // OR
            // Node16::from(self).add(&[&[*cur_value], remaining_values].concat(), match_type)
            // OR
            // try std::deque
            Node16::from(self).add_multiple_case(cur_value, remaining_values, match_type)
        } else {
            //add value to existing Node4 if there is room
            self.keys[self.size] = Some(*cur_value);
            self.children[self.size] = Node4::new().add(remaining_values, match_type);

            self.size += 1;
            NodeEnum::N4(self)
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
    pub fn new() -> Self {
        Node16 {
            keys: [None; 16],
            children: Box::new(arr![NodeEnum::NNone; 16]),
            size: 0,
            terminal: false,
        }
    }

    pub fn from(mut node: Node4) -> Self {
        let mut new_node = Node16::new();
        //sort the keys and original indices of the keys
        //the original indices will be used to create new arrays with the correct order
        let mut ordered_index_value = node.keys.iter().enumerate().collect::<Vec<_>>();
        ordered_index_value.sort_unstable_by(|(_, a), (_, b)| Node16::val_cmp(a, b));
        //FIXME should be possible to do this without collecting into a vector
        let ordered_index = ordered_index_value
            .iter()
            .map(|(index, _)| *index)
            .collect::<Vec<_>>();
        //order arrays based on the ordered indices
        for (target_i, source_i) in ordered_index.iter().enumerate() {
            new_node.keys[target_i] = node.keys[(*source_i)].take();
            new_node.children[target_i] = node.children[*source_i].take(); //same function used by Option::take to replace element
        }

        new_node.terminal = node.terminal;
        new_node.size = node.size;
        new_node
    }

    fn val_cmp(a: &Option<u8>, b: &Option<u8>) -> Ordering {
        if a.is_none() && b.is_none() {
            Ordering::Equal
        } else if a.is_none() && b.is_some() {
            Ordering::Greater
        } else if a.is_some() && b.is_none() {
            Ordering::Less
        } else {
            a.unwrap().cmp(&b.unwrap())
        }
    }
}

impl TrieNode for Node16 {
    fn add_empty_case(mut self, match_type :&Match) -> NodeEnum {
        self.terminal = true;
        NodeEnum::N16(self)
    }

    fn add_single_case(mut self, cur_value: &u8, match_type: &Match) -> NodeEnum {
        //check if value exists already
        match self
            .keys
            .binary_search_by(|probe| Node16::val_cmp(probe, &Some(*cur_value)))
        {
            Ok(index) => {
                self.children[index] = self.children[index].take().add(&[], match_type);
                NodeEnum::N16(self)
            }
            Err(index) => {
                //expand to node48 and then add new value
                if self.is_full() {
                    //FIXME these return nodes are inconsistent. Some are returning the self (upgraded) and other are returning the child that is created
                    // need to determine which one should be returned, self or created child?
                    // need to map it out to determine what is getting returned and if its correct
                    Node48::from(self).add(&[*cur_value], match_type)
                } else {
                    //add value in sorted order to existing Node16 if there is room
                    self.keys[index..].rotate_right(1); //shift right from index
                    self.keys[index] = Some(*cur_value);

                    self.children[index..].rotate_right(1);
                    self.children[index] = Node0::new().add(&[], match_type);

                    self.size += 1;
                    NodeEnum::N16(self)
                }
            }
        }
    }

    fn add_multiple_case(
        mut self,
        cur_value: &u8,
        remaining_values: &[u8],
        match_type: &Match,
    ) -> NodeEnum {
        //check if value exists already
        match self
            .keys
            .binary_search_by(|probe| Node16::val_cmp(probe, &Some(*cur_value)))
        {
            //TODO: can this be simplified to add_single_case.add(remaining_values, match_type),
            // add single case would instead need to return the child that is created or used, so figuring out ownership would be hard
            // add multiple is just add single to upgraded self, and add rest to it's newly created child...
            //instead
            Ok(index) => {
                self.children[index] = self.children[index]
                    .take()
                    .add(remaining_values, match_type);
                NodeEnum::N16(self)
            }
            Err(index) => {
                //expand to node48 and then add new value
                if self.is_full() {
                    Node48::from(self).add_multiple_case(cur_value, remaining_values, match_type)
                } else {
                    //add value in sorted order to existing Node16 if there is room
                    self.keys[index..].rotate_right(1); //shift right from index
                    self.keys[index] = Some(*cur_value);

                    self.children[index..].rotate_right(1);
                    self.children[index] = Node0::new().add(remaining_values, match_type);

                    self.size += 1;
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
    pub fn new() -> Self {
        Node48 {
            keys: [None; 256],
            children: Box::new(arr![NodeEnum::NNone; 48]),
            size: 0,
            terminal: false,
        }
    }

    pub fn from(mut node: Node16) -> Self {
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

impl TrieNode for Node48 {
    fn add_empty_case(mut self, match_type :&Match) -> NodeEnum {
        self.terminal = true;
        NodeEnum::N48(self)
    }

    //TODO: can specific adds (e.g. add_empty_case) be mutable borrows and general add be a move?
    fn add_single_case(mut self, cur_value: &u8, match_type: &Match) -> NodeEnum {
        let cur_value_index = *cur_value as usize;
        //if exists
        if let Some(key) = self.keys[cur_value_index] {
            let key_index = key as usize;
            self.children[key_index] = self.children[key_index].take().add(&[], match_type);
            NodeEnum::N48(self)
        } else if self.is_full() {
            Node256::from(self).add_single_case(cur_value, match_type)
        } else {
            //add to self
            self.keys[cur_value_index] = Some(self.size as u8);
            self.children[self.size] = Node0::new().add(&[], match_type);
            self.size += 1;
            NodeEnum::N48(self)
        }
    }

    fn add_multiple_case(
        mut self,
        cur_value: &u8,
        remaining_values: &[u8],
        match_type: &Match,
    ) -> NodeEnum {
        let cur_value_index = *cur_value as usize;
        //if exists
        if let Some(key) = self.keys[cur_value_index] {
            let key_index = key as usize;
            self.children[key_index] = self.children[key_index].take().add(remaining_values, match_type);
            NodeEnum::N48(self)
        } else if self.is_full() {

            Node256::from(self).add_multiple_case(cur_value, remaining_values, match_type)
        } else {
            //add to self
            self.keys[cur_value_index] = Some(self.size as u8);
            self.children[self.size] = Node0::new().add(remaining_values, match_type);
            self.size += 1;
            NodeEnum::N48(self)
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

impl Node256 {
    pub fn new() -> Self {
        Node256 {
            children: Box::new(arr![NodeEnum::NNone; 256]),
            size: 0,
            terminal: false,
        }
    }

    pub fn from(mut node: Node48) -> Self {
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
}

impl TrieNode for Node256 {
    fn add_empty_case(mut self, match_type :&Match) -> NodeEnum {
        self.terminal = true;
        NodeEnum::N256(self)
    }

    fn add_single_case(mut self, cur_value: &u8, match_type: &Match) -> NodeEnum {
        let cur_value_index = *cur_value as usize;
        //if exists
        match self.children[cur_value_index].take() {
            NodeEnum::NNone => {
                //TODO: does this mean I should implement a NodeNull type?
                self.children[cur_value_index] = Node0::new().add(&[], match_type);
                NodeEnum::N256(self)
            }
            node => {
                self.children[cur_value_index] = node.add(&[*cur_value], match_type);
                self.size += 1;
                NodeEnum::N256(self)
            }
        }
    }

    fn add_multiple_case(
        mut self,
        cur_value: &u8,
        remaining_values: &[u8],
        match_type: &Match,
    ) -> NodeEnum {
        let cur_value_index = *cur_value as usize;
        //if exists
        match self.children[cur_value_index].take() {
            NodeEnum::NNone => {
                //TODO: does this mean I should implement a NodeNull type?
                self.children[cur_value_index] = Node0::new().add(remaining_values, match_type);
                NodeEnum::N256(self)
            }
            node => {
                self.children[cur_value_index] = node.add(remaining_values, match_type);
                self.size += 1;
                NodeEnum::N256(self)
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

impl NodeEnum {
    pub fn add(self, value: &[u8], match_type: &Match) -> Self {
        match self {
            NodeEnum::NNone => NodeEnum::N0(Node0::new()),
            NodeEnum::N0(n) => n.add(value, match_type),
            NodeEnum::N4(n) => n.add(value, match_type),
            NodeEnum::N16(n) => n.add(value, match_type),
            NodeEnum::N48(n) => n.add(value, match_type),
            NodeEnum::N256(n) => n.add(value, match_type),
        }
    }

    fn add_empty_case(self, match_type :&Match) -> NodeEnum {
        match self {
            NodeEnum::NNone => NodeEnum::N0(Node0::new()), //FIXME new probably needs match_type as well
            NodeEnum::N0(n) => n.add_empty_case(match_type),
            NodeEnum::N4(n) => n.add_empty_case(match_type),
            NodeEnum::N16(n) => n.add_empty_case(match_type),
            NodeEnum::N48(n) => n.add_empty_case(match_type),
            NodeEnum::N256(n) => n.add_empty_case(match_type),
        }
    }

    fn add_single_case(self, cur_value: &u8, match_type: &Match) -> NodeEnum {
        match self {
            NodeEnum::NNone => NodeEnum::N0(Node0::new()),
            NodeEnum::N0(n) => n.add_single_case(cur_value, match_type),
            NodeEnum::N4(n) => n.add_single_case(cur_value, match_type),
            NodeEnum::N16(n) => n.add_single_case(cur_value, match_type),
            NodeEnum::N48(n) => n.add_single_case(cur_value, match_type),
            NodeEnum::N256(n) => n.add_single_case(cur_value, match_type),
        }
    }

    fn add_multiple_case(
        self,
        cur_value: &u8,
        remaining_values: &[u8],
        match_type: &Match,
    ) -> NodeEnum {
        match self {
            NodeEnum::NNone => NodeEnum::N0(Node0::new()),
            NodeEnum::N0(n) => n.add_multiple_case(cur_value, remaining_values, match_type),
            NodeEnum::N4(n) => n.add_multiple_case(cur_value, remaining_values, match_type),
            NodeEnum::N16(n) => n.add_multiple_case(cur_value, remaining_values, match_type),
            NodeEnum::N48(n) => n.add_multiple_case(cur_value, remaining_values, match_type),
            NodeEnum::N256(n) => n.add_multiple_case(cur_value, remaining_values, match_type),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trial_run_test() {
        let mut node = NodeEnum::N0(Node0::new());
        node = node.add("ab".as_bytes(), &Match::Exact);
        node = node.add("ad".as_bytes(), &Match::Exact);
        node = node.add("as".as_bytes(), &Match::Exact);
        node = node.add("at".as_bytes(), &Match::Exact);
        node = node.add("ace".as_bytes(), &Match::Exact);

        if let NodeEnum::N4(root_node) = node {
            println!("root: {:#?}",root_node);
            if let NodeEnum::N16(a_node) = &root_node.children[0] {
                println!("child 1: {:#?}",a_node);
            }
        }
    }
}
