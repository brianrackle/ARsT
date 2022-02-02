use std::any::Any;
use crate::trie::enums::Match;
use arr_macro::arr;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

type Link = Option<Box<dyn Node>>;
// type LinkMutRef = Option<&mut Box<dyn Node>>;

//TODO explore recursive solution like add_multiple(remaining_value).add_single(current_value) would require owning
pub trait Node : Debug {
    fn get_location(&self, cur_value :&u8) -> Option<usize>;
    fn insert(&mut self, cur_value: &u8, location :Option<usize>) -> Option<&mut Box<dyn Node>>;
    fn add(&mut self, cur_value: &u8) -> Option<&mut Box<dyn Node>>;
    fn upgrade(&mut self) -> Box<dyn Node>; //FIXME would be best if upgrade moved self look into using another trait see NodeUpgrade
    fn terminate(&mut self);
    fn get_size(&self) -> usize;
    fn get_capacity(&self) -> usize;
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_terminal(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
}

// pub trait NodeUpgrade {
//     fn upgrade(self) -> Box<dyn Node>;
// }

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

//series a equity
//$170
//15%
//likes work wouldnt change
//15-16

impl Node for Node4 {
    fn get_location(&self, cur_value: &u8) -> Option<usize> {
       if let Some(index) = self
            .keys
            .iter()
            .position(|v| if let Some(cmp) = v { cmp == cur_value } else { false }) {
           Some(index) //FIXME might be able to do a comparison in add so that Exists and Insert arent both needed
       } else if !self.is_full() {
           Some(self.size)
       } else {
           None
       }
    }

    // adds a single value and returns the new current if its upgraded, and the node where the value was added
    fn insert(&mut self, cur_value: &u8, location :Option<usize>) -> Option<&mut Box<dyn Node>> {
        match location {
            Some(index) => {
                if let Some(_) = self.keys[index] {
                    self.children[index].as_mut()
                } else {
                    self.keys[index] = Some(*cur_value);
                    self.children[index] = Some(Box::new(Node4::new()));
                    self.size += 1;
                    self.children[index].as_mut()
                }
            },
            None => None,
        }
    }

    // adds a single value and returns the next node if add succeeded
    fn add(&mut self, cur_value: &u8) -> Option<&mut Box<dyn Node>> {
        self.insert(cur_value, self.get_location(cur_value))
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
    fn get_location(&self, cur_value: &u8) -> Option<usize> {
        //check if value exists already
        match self
            .keys
            .binary_search_by(|probe| Node16::val_cmp(probe, &Some(*cur_value))) {
            Ok(index) => {
                Some(index)
            }
            Err(index) => {
                if !self.is_full() { Some(index) } else { None }
            }
        }
    }

    fn insert(&mut self, cur_value: &u8, location: Option<usize>) -> Option<&mut Box<dyn Node>> {
        match location {
            Some(index) => {
                if let Some(_) = self.keys[index] {
                    self.children[index].as_mut()
                } else {
                    self.keys[index..].rotate_right(1); //shift right from index
                    self.keys[index] = Some(*cur_value);

                    self.children[index..].rotate_right(1);
                    self.children[index] = Some(Box::new(Node4::new()));

                    self.size += 1;
                    self.children[index].as_mut()
                }
            },
            None => None,
        }
    }

    fn add(&mut self, cur_value: &u8) -> Option<&mut Box<dyn Node>> {
        self.insert(cur_value, self.get_location(cur_value))
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
    fn get_location(&self, cur_value: &u8) -> Option<usize> {
        let cur_value_index = *cur_value as usize;
        if let Some(_) = self.keys[cur_value_index] {
            Some(cur_value_index)
        } else if !self.is_full() {
            Some(cur_value_index)
        } else {
            None
        }
    }

    fn insert(&mut self, cur_value: &u8, location: Option<usize>) -> Option<&mut Box<dyn Node>> {
        match location {
            Some(index) => {
                if let Some(child_index) = self.keys[index] {
                    self.children[child_index as usize].as_mut()
                } else {
                    let target_index = self.size;
                    self.keys[index] = Some(target_index as u8);
                    self.children[target_index] = Some(Box::new(Node4::new()));
                    self.size += 1;
                    self.children[target_index].as_mut()
                }
            },
            None => None,
        }
    }

    fn add(&mut self, cur_value: &u8) -> Option<&mut Box<dyn Node>> {
        self.insert(cur_value, self.get_location(cur_value))
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
    fn get_location(&self, cur_value: &u8) -> Option<usize> {
        let cur_value_index = *cur_value as usize;
        match self.children[cur_value_index] {
            Some(_) => {
                Some(cur_value_index)
            }
            None => {
                Some(cur_value_index)
            }
        }
    }

    fn insert(&mut self, cur_value: &u8, location: Option<usize>) -> Option<&mut Box<dyn Node>> {
        match location {
            Some(index) => {
                if self.children[index].is_some() {
                    self.children[index].as_mut()
                } else {
                    self.children[index] = Some(Box::new(Node4::new()));
                    self.size += 1;
                    self.children[index].as_mut()
                }
            },
            None=> None,
        }
    }

    fn add(&mut self, cur_value: &u8) -> Option<&mut Box<dyn Node>> {
        self.insert(cur_value, self.get_location(cur_value))
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
