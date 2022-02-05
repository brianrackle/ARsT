use crate::trie::enums::Match;
use crate::trie::node::N::{Empty};
use arr_macro::arr;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem;

pub trait Node : Debug{
    fn add(&mut self, values: &[u8], match_type: &Match) -> N;
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_terminal(&self) -> bool;
}

#[derive(Debug)]
pub struct Node4 {
    keys: [Option<u8>; 4], //Can remove this option and rely only on children option
    children: [N; 4],
    size: usize,
    terminal: bool,
}

#[derive(Debug)]
pub struct Node16 {
    keys: [Option<u8>; 16],
    //value represents value with matching node in children index
    children: [N; 16],
    size: usize,
    terminal: bool,
}

#[derive(Debug)]
pub struct Node48 {
    keys: [Option<u8>; 256],
    //index represents value, and value represents index in children
    children: [N; 48],
    size: usize,
    terminal: bool,
}

#[derive(Debug)]
pub struct Node256 {
    children: [N; 256],
    size: usize,
    terminal: bool,
}

//see: https://www.the-paper-trail.org/post/art-paper-notes/
//FIXME this results in a union with a size equal to the largest object
// consider boxing large variants OR remove enum
#[derive(Debug)]
pub enum N {
    Empty,
    Nx(Box<dyn Node>)
}

impl N {
    pub fn take(&mut self) -> N {
        mem::replace(self, Empty)
    }
}
impl Default for N {
    fn default() -> Self {
        N::Empty
    }
}

impl Node4 {
    pub fn new() -> Self {
        Node4 {
            keys: [None; 4],
            children: arr![N::Empty; 4],
            size: 0,
            terminal: false,
        }
    }
}

impl Default for Node4 {
    fn default() -> Self {
        Node4::new()
    }
}

impl Node for Node4 {
    fn add(&mut self, values: &[u8], match_type: &Match) -> N {
        if let Some((first, rest)) = values.split_first() {
            //check if value exists already
            if let Some(index) = self.keys.iter().position(|v| v.is_some() && v.unwrap() == *first)
            {
                self.children[index] = self.children[index].take().add(rest, match_type);
                N::Nx(Box::new(std::mem::take(self))) //FIXME return NONE if no upgrade occurs
            } else if self.is_full() { //value doesnt exist yet
                //expand to node16 and then add new value
                Node16::from(self).add(values, match_type)
            } else {
                //add value to existing Node4 if there is room
                self.keys[self.size] = Some(*first);
                self.children[self.size] = Node4::new().add(rest, match_type);

                self.size += 1;
                N::Nx(Box::new(std::mem::take(self)))
            }
        } else {
            self.terminal = true;
            N::Nx(Box::new(std::mem::take(self)))
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
            children: arr![N::Empty; 16],
            size: 0,
            terminal: false,
        }
    }

    pub fn from(node: &mut Node4) -> Self {
        let mut new_node = Node16::new();
        //sort the keys and original indices of the keys
        //the original indices will be used to create new arrays with the correct order
        let mut ordered_index_value = node.keys.iter().enumerate().collect::<Vec<_>>();
        ordered_index_value.sort_unstable_by(|(_, a), (_, b)| Node16::val_cmp(a, b));
        //FIXME should be possible to do this without collecting into a vecto
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

impl Node for Node16 {
    fn add(&mut self, values: &[u8], match_type: &Match) -> N {
        if let Some((first, rest)) = values.split_first() {
            match self
                .keys
                .binary_search_by(|probe| Node16::val_cmp(probe, &Some(*first)))
            {
                Ok(index) => {
                    self.children[index] = self.children[index]
                        .take()
                        .add(rest, match_type);

                    N::Nx(Box::new(std::mem::take(self)))
                }
                Err(index) => {
                    //expand to node48 and then add new value
                    if self.is_full() {
                        Node48::from(self).add(values, match_type)
                    } else {
                        //add value in sorted order to existing Node16 if there is room
                        self.keys[index..].rotate_right(1); //shift right from index
                        self.keys[index] = Some(*first);

                        self.children[index..].rotate_right(1);
                        self.children[index] = Node4::new().add(rest, match_type);

                        self.size += 1;
                        N::Nx(Box::new(std::mem::take(self)))
                    }
                }
            }
        } else {
            self.terminal = true;
            N::Nx(Box::new(std::mem::take(self)))
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

impl Default for Node48 {
    fn default() -> Self {
        Node48::new()
    }
}

impl Node48 {
    pub fn new() -> Self {
        Node48 {
            keys: [None; 256],
            children: arr![N::Empty; 48],
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
    fn add(&mut self, values: &[u8], match_type: &Match) -> N {
        if let Some((first, rest)) = values.split_first() {
            let cur_value_index = *first as usize;
            //if exists
            if let Some(key) = self.keys[cur_value_index] {
                let key_index = key as usize;
                self.children[key_index] = self.children[key_index].take().add(rest, match_type);
                N::Nx(Box::new(std::mem::take(self)))
            } else if self.is_full() {
                Node256::from(self).add(values, match_type)
            } else {
                //add to self
                self.keys[cur_value_index] = Some(self.size as u8);
                self.children[self.size] = Node4::new().add(rest, match_type);
                self.size += 1;
                N::Nx(Box::new(std::mem::take(self)))
            }
        } else {
            self.terminal = true;
            N::Nx(Box::new(std::mem::take(self)))
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

impl Default for Node256 {
    fn default() -> Self {
        Node256::new()
    }
}

impl Node256 {
    pub fn new() -> Self {
        Node256 {
            children: arr![N::Empty; 256],
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
}

impl Node for Node256 {
    fn add(&mut self, values: &[u8], match_type: &Match) -> N {
        if let Some((first, rest)) = values.split_first() {
            let cur_value_index = *first as usize;
            //if exists
            match &mut self.children[cur_value_index].take() {
                N::Empty => {
                    self.children[cur_value_index] = Node4::new().add(rest, match_type);
                    self.size += 1;
                    N::Nx(Box::new(std::mem::take(self)))
                }
                node => {
                    self.children[cur_value_index] = node.add(rest, match_type);
                    self.size += 1;
                    N::Nx(Box::new(std::mem::take(self)))
                }
            }
        } else {
            self.terminal = true;
            N::Nx(Box::new(std::mem::take(self)))
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

impl N {
    pub fn add(&mut self, value: &[u8], match_type: &Match) -> Self {
        match self {
            N::Empty => Node4::new().add(value, match_type),
            N::Nx(n) => n.add(value, match_type)
        }
    }
}

//Old implementation starts here
pub type Link = Option<Box<OldNode>>;
pub struct OldNode {
    children: [Link; 257],
}

impl OldNode {
    //TODO: make variable length based off settings
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

    // #[test]
    // fn trial_run_test() {
    //     let mut node = N::N4(Box::new(Node4::new()));
    //     node = node.add("ab".as_bytes(), &Match::Exact);
    //     node = node.add("ad".as_bytes(), &Match::Exact);
    //     node = node.add("as".as_bytes(), &Match::Exact);
    //     node = node.add("at".as_bytes(), &Match::Exact);
    //     node = node.add("ace".as_bytes(), &Match::Exact);
    //
    //     if let N::N4(root_node) = node {
    //         println!("root: {:#?}",root_node);
    //         if let N::N16(a_node) = &root_node.children[0] {
    //             println!("child 1: {:#?}",a_node);
    //         }
    //     }
    //     // println!("root: {:#?}",node);
    // }

    #[test]
    fn test_all_upgrades_occur_exact_match() {
        let mut node = N::Nx(Box::new(Node4::new()));
        for i in 0..=3 {
            node = node.add(&[i], &Match::Exact);
        }
        if let N::Nx(n) = &node {
            assert!(n.is_full());
        }

        for i in 4..=15 {
            node = node.add(&[i], &Match::Exact);
        }
        if let N::Nx(n) = &node {
            assert!(n.is_full());
        }

        for i in 16..=47 {
            node = node.add(&[i], &Match::Exact);
        }
        if let N::Nx(n) = &node {
            assert!(n.is_full());
        }

        for i in 48..=255 {
            node = node.add(&[i], &Match::Exact);
        }
        if let N::Nx(n) = &node {
            assert!(n.is_full());
        }

    }
    //
    // #[test]
    // fn order_preserved_48_exact_match() {
    //     let mut node = N::N4(Box::new(Node4::new()));
    //
    //     for i in 0..=96 {
    //         if i % 2 == 0 {
    //             node = node.add(&[i], &Match::Exact);
    //         }
    //     }
    //
    //     if let N::Nx(n) = node {
    //         for (i, &k) in n.keys.iter().enumerate() {
    //             if i < 96 { //only first entries 48 considered
    //                 match k {
    //                     None => {
    //                         assert_ne!(i % 2, 0);
    //                     },
    //                     Some(c) => {
    //                         assert_eq!(i % 2, 0);
    //                         assert!(matches!(&n.children[c as usize], N::Nx(_)));
    //                     },
    //                     _ => panic!()
    //                 }
    //             }
    //         }
    //     }
    // }
    //
    // #[test]
    // fn order_preserved_256_exact_match() {
    //     let mut node = N::Nx(Box::new(Node4::new()));
    //
    //     for i in 0..=255 {
    //         if i % 2 == 0 {
    //             node = node.add(&[i], &Match::Exact);
    //         }
    //     }
    //     println!("{:#?}",node);
    //     if let N::Nx(n) = node {
    //         for (i, c) in n.children.iter().enumerate() {
    //             match &c {
    //                 N::Empty => assert_ne!(i % 2, 0),
    //                 N::Nx(_) => assert_eq!(i % 2, 0),
    //                 _ => panic!()
    //             }
    //         }
    //     }
    // }
}
