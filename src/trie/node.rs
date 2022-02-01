use std::any::Any;
use crate::trie::enums::Match;
use arr_macro::arr;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

type Link = Option<Box<dyn Node>>;
// type LinkMutRef = Option<&mut Box<dyn Node>>;


pub trait Node : Debug {
    // fn add(&mut self, value: &[u8]) -> N {
    //     match value {
    //         [] => self.add_empty_case(),
    //         [cur_value] => self.add_single_case(cur_value),
    //         [cur_value, remaining_values @ ..] => {
    //             self.add_multiple_case(cur_value, remaining_values)
    //         }
    //     }
    // }
    fn add(&mut self, cur_value: &u8) -> Option<&mut Box<dyn Node>>;
    fn upgrade(&mut self) -> Box<dyn Node>;
    fn terminate(&mut self);
    fn get_size(&self) -> usize;
    fn get_capacity(&self) -> usize;
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_terminal(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct Node4 {
    keys: [Option<u8>; 4], //Can remove this option and rely only on children option
    children: [Link; 4],
    size: usize,
    terminal: bool,
}

#[derive(Debug)]
pub struct Node16 {
    keys: [Option<u8>; 16],
    //value represents value with matching node in children index
    children: [Link; 16],
    size: usize,
    terminal: bool,
}

#[derive(Debug)]
pub struct Node48 {
    keys: [Option<u8>; 256],
    //index represents value, and value represents index in children
    children: [Link; 48],
    size: usize,
    terminal: bool,
}

#[derive(Debug)]
pub struct Node256 {
    children: [Link; 256],
    size: usize,
    terminal: bool,
}

//see: https://www.the-paper-trail.org/post/art-paper-notes/
impl Node4 {
    pub fn new() -> Self {
        Node4 {
            keys: [None; 4],
            children: arr![None; 4],
            size: 0,
            terminal: false,
        }
    }
}

impl Node for Node4 {
    // adds a single value and returns the new current if its upgraded, and the node where the value was added
    fn add(&mut self, cur_value: &u8) -> Option<&mut Box<dyn Node>> {
        //check if value exists already
        if let Some(index) = self
            .keys
            .iter()
            .position(|v| v.is_some() && v.unwrap() == *cur_value)
        {
            self.children[index].as_mut()
        } else {
            //value doesnt exist yet
            //expand to node16 and then add new value
            if !self.is_full() {
                //add value to existing Node4 if there is room
                let target_index = self.size;
                self.keys[target_index] = Some(*cur_value);
                self.children[target_index] = Some(Box::new(Node4::new()));
                self.size += 1;
                self.children[target_index].as_mut()
            } else {
                None
            }
        }
    }

    fn upgrade(&mut self) -> Box<dyn Node> {
        Box::new(Node16::from(self))
    }

    fn terminate(&mut self) {
        self.terminal = true;
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_capacity(&self) -> usize {
        self.children.len()
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

    fn as_any(&self) -> &dyn Any {
        self
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

    //sort the keys and original indices of the keys
    //the original indices will be used to create new arrays with the correct order
    pub fn from(node: &mut Node4) -> Self {
        let mut new_node = Node16::new();
        let mut ordered_index_value = node.keys.iter().enumerate().collect::<Vec<_>>();
        ordered_index_value.sort_unstable_by(|(_, a), (_, b)| Node16::val_cmp(a, b));
        //FIXME should be possible to do this without collecting into a vec
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

impl Node for Node16 {
    fn add(&mut self, cur_value: &u8) -> Option<&mut Box<dyn Node>> {
        //check if value exists already
        match self
            .keys
            .binary_search_by(|probe| Node16::val_cmp(probe, &Some(*cur_value)))
        {
            Ok(index) => {
                //FIXME can do None/Some check for extra error checking
                self.children[index].as_mut()
            }
            Err(index) => {
                //expand to node48 and then add new value
                if !self.is_full() {
                    //add value in sorted order to existing Node16 if there is room
                    self.keys[index..].rotate_right(1); //shift right from index
                    self.keys[index] = Some(*cur_value);

                    self.children[index..].rotate_right(1);
                    self.children[index] = Some(Box::new(Node4::new()));

                    self.size += 1;
                    self.children[index].as_mut()
                } else {
                    None
                }
            }
        }
    }

    fn upgrade(&mut self) -> Box<dyn Node> {
        Box::new(Node48::from(self))
    }

    fn terminate(&mut self) {
        self.terminal = true;
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_capacity(&self) -> usize {
        self.children.len()
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

    fn as_any(&self) -> &dyn Any {
        self
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
}


impl Node for Node48 {
    fn add(&mut self, cur_value: &u8) -> Option<&mut Box<dyn Node>> {
        let cur_value_index = *cur_value as usize;
        //if exists
        if let Some(key) = self.keys[cur_value_index] {
            let key_index = key as usize;
            self.children[key_index].as_mut()
        } else if !self.is_full() {
            //add to self
            let target_index = self.size;
            self.keys[cur_value_index] = Some(self.size as u8);
            self.children[target_index] = Some(Box::new(Node4::new()));
            self.size += 1;
            self.children[target_index].as_mut()
        } else {
            None
        }
    }

    fn upgrade(&mut self) -> Box<dyn Node> {
        Box::new(Node256::from(self))
    }

    fn terminate(&mut self) {
        self.terminal = true;
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_capacity(&self) -> usize {
        self.children.len()
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

    fn as_any(&self) -> &dyn Any {
        self
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

    //FIXME change name to indicate that from consumes elements from input node
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
}

impl Node for Node256 {
    fn add(&mut self, cur_value: &u8) -> Option<&mut Box<dyn Node>> {
        let cur_value_index = *cur_value as usize;
        //if exists
        match self.children[cur_value_index] {
            None => {
                self.children[cur_value_index] = Some(Box::new(Node4::new()));
                self.size += 1;
                self.children[cur_value_index].as_mut()
            }
            Some(_) => {
                self.children[cur_value_index].as_mut()
            }
        }
    }

    fn upgrade(&mut self) -> Box<dyn Node> {
        unimplemented!()
    }

    fn terminate(&mut self) {
        self.terminal = true;
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_capacity(&self) -> usize {
        self.children.len()
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}

//Old implementation starts here
pub type OldLink = Option<Box<OldNode>>;
pub struct OldNode {
    children: [OldLink; 257],
}

impl OldNode {
    pub fn new() -> Self {
        OldNode {
            children: arr![None; 257],
        }
    }

    pub fn get_node(&self, i: usize) -> Option<&OldNode> {
        self.children[i].as_ref().map(|c| c.as_ref())
    }

    pub fn add(&mut self, value: &[u8], match_type: &Match) {
        match match_type {
            Match::Exact | Match::Prefix => {
                let mut cur = self;
                for c in value {
                    cur = cur.children[(*c) as usize].get_or_insert(Box::new(OldNode::new()));
                }
                //add terminal char when match is exact
                if let Match::Exact = match_type {
                    cur.children[257 - 1] = Some(Box::new(OldNode::new()))
                }
            }
            Match::PrefixPostfix => {
                //takes 0+n first characters off string
                let mut cur: &mut OldNode;
                for j in 0..value.len() {
                    cur = self;
                    for c in value[j..].iter() {
                        cur = cur.children[(*c) as usize].get_or_insert(Box::new(OldNode::new()));
                    }
                }
            }
        }
    }

    pub fn exists(&self, c: u8) -> Option<&OldNode> {
        self.children[c as usize].as_ref().map(|c| c.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_upgrades_occur_exact_match() {
        let mut node :Box<dyn Node> = Box::new(Node4::new());
        for i in 0..=3 {
            node.add(&i);
        }

        assert!(matches!(node.add(&4), None));
        assert_eq!(node.get_capacity(), 4);
        assert_eq!(node.get_size(), 4);
        assert!(node.is_full());
        assert!(!node.is_empty());

        node = node.upgrade();
        for i in 4..=15 {
            node.add(&i);
        }
        assert!(matches!(node.add(&16), None));
        assert_eq!(node.get_capacity(), 16);
        assert_eq!(node.get_size(), 16);
        assert!(node.is_full());
        assert!(!node.is_empty());

        node = node.upgrade();
        for i in 16..=47 {
            node.add(&i);
        }
        assert!(matches!(node.add(&48), None));
        assert_eq!(node.get_capacity(), 48);
        assert_eq!(node.get_size(), 48);
        assert!(node.is_full());
        assert!(!node.is_empty());

        node = node.upgrade();
        for i in 48..=255 {
            node.add(&i);
        }
        assert_eq!(node.get_capacity(), 256);
        assert_eq!(node.get_size(), 256);
        assert!(node.is_full());
        assert!(!node.is_empty());
    }

    //
    #[test]
    fn order_preserved_48_exact_match() {
        let mut node: Box<dyn Node> = Box::new(Node4::new());

        for i in 0..48 {
            if node.is_full() {
                node = node.upgrade();
            }
            assert!(matches!(node.add(&(i * 2)), Some(_)));
        }

        assert_eq!(node.get_size(), 48);
        let node48 = Box::new(node.as_any().downcast_ref::<Node48>().unwrap());

        for (i, &k) in node48.keys.iter().enumerate() {
            if i < 96 { //only first entries 48 considered
                match k {
                    None => {
                        assert_ne!(i % 2, 0);
                    },
                    Some(c) => {
                        assert_eq!(i % 2, 0);
                        assert!(matches!(&node48.children[c as usize], Some(_)));
                    },
                    _ => panic!()
                }
            }
        }
    }

    #[test]
    fn order_preserved_256_exact_match() {
        let mut node : Box<dyn Node> = Box::new(Node4::new());

        for i in 0..=255 {
            if i % 2 == 0 {
                if node.is_full() {
                    node = node.upgrade();
                }
                assert!(matches!(node.add(&i), Some(_)));
            }
        }

        assert_eq!(node.get_size(), 128);
        let node256 = Box::new(node.as_any().downcast_ref::<Node256>().unwrap());

        for (i, c) in node256.children.iter().enumerate() {
            match &c {
                None => assert_ne!(i % 2, 0),
                Some(n) => assert_eq!(i % 2, 0),
            }
        }
    }
}
