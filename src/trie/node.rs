use std::any::Any;
use crate::trie::node::NodeOption::{Empty};
use arr_macro::arr;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem;

pub trait Node : Debug {
    fn add(&mut self, values: &[u8]) -> NodeOption;
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_terminal(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
}

//TODO performance and memory test storing children directly in keys
#[derive(Debug)]
pub struct Node4 {
    keys: [Option<u8>; 4], //FIXME: Can remove this option and rely only on children option
    children: [NodeOption; 4],
    size: usize,
    terminal: bool,
}

#[derive(Debug)]
pub struct Node16 {
    keys: [Option<u8>; 16],
    //value represents value with matching node in children index
    children: [NodeOption; 16],
    size: usize,
    terminal: bool,
}

#[derive(Debug)]
pub struct Node48 {
    keys: [Option<u8>; 256],
    //index represents value, and value represents index in children
    children: [NodeOption; 48],
    size: usize,
    terminal: bool,
}

#[derive(Debug)]
pub struct Node256 {
    children: [NodeOption; 256],
    size: usize,
    terminal: bool,
}

//see: https://www.the-paper-trail.org/post/art-paper-notes/
#[derive(Debug)]
pub enum NodeOption {
    Empty,
    Nx(Box<dyn Node>)
}

impl Node4 {
    pub fn new() -> Self {
        Node4 {
            keys: [None; 4],
            children: arr![NodeOption::Empty; 4],
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
    fn add(&mut self, values: &[u8]) -> NodeOption {
        if let Some((first, rest)) = values.split_first() {
            //check if value exists already
            if let Some(index) = self.keys.iter().position(|v| v.is_some() && v.unwrap() == *first)
            {
                //FIXME reintroduce N0, make that default, add to it to trigger an upgrade to accomplish and Node enum add wont be needed
                //let upgraded_node = self.children[index].unwrap_or_default().add(rest);
                let upgraded_node = self.children[index].add(rest);
                if upgraded_node.is_node() {
                    self.children[index] = upgraded_node;
                }
                NodeOption::Empty
            } else if self.is_full() { //value doesnt exist yet
                //expand to node16 and then add new value
                let mut upgraded_node = Node16::from(self);
                upgraded_node.add(values);
                NodeOption::Nx(Box::new(upgraded_node))
            } else {
                //add value to existing Node4 if there is room
                self.keys[self.size] = Some(*first);
                let mut new_node = NodeOption::Nx(Box::new(Node4::new()));
                new_node.add(rest);
                self.children[self.size] = new_node;
                self.size += 1;
                NodeOption::Empty
            }
        } else {
            self.terminal = true;
            NodeOption::Empty
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

    fn as_any(&self) -> &dyn Any {
        self
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
            children: arr![NodeOption::Empty; 16],
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
    fn add(&mut self, values: &[u8]) -> NodeOption {
        if let Some((first, rest)) = values.split_first() {
            match self
                .keys
                .binary_search_by(|probe| Node16::val_cmp(probe, &Some(*first)))
            {
                Ok(index) => {
                    let upgraded_node = self.children[index].add(rest);
                    if upgraded_node.is_node() {
                        self.children[index] = upgraded_node;
                    }
                    NodeOption::Empty
                }
                Err(index) => {
                    //expand to node48 and then add new value
                    if self.is_full() {
                        let mut upgraded_node = Node48::from(self);
                        upgraded_node.add(values);
                        NodeOption::Nx(Box::new(upgraded_node))
                    } else {
                        //add value in sorted order to existing Node16 if there is room
                        self.keys[index..].rotate_right(1); //shift right from index
                        self.keys[index] = Some(*first);

                        self.children[index..].rotate_right(1);
                        let mut new_node = NodeOption::Nx(Box::new(Node4::new()));
                        new_node.add(rest);
                        self.children[index] = new_node;

                        self.size += 1;
                        NodeOption::Empty
                    }
                }
            }
        } else {
            self.terminal = true;
            NodeOption::Empty
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

    fn as_any(&self) -> &dyn Any {
        self
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
            children: arr![NodeOption::Empty; 48],
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
    fn add(&mut self, values: &[u8]) -> NodeOption {
        if let Some((first, rest)) = values.split_first() {
            let cur_value_index = *first as usize;
            //if exists
            if let Some(key) = self.keys[cur_value_index] {
                let key_index = key as usize;
                let upgraded_node =self.children[key_index].add(rest);
                if upgraded_node.is_node() {
                    self.children[key_index] = upgraded_node;
                }
                NodeOption::Empty
            } else if self.is_full() {
                let mut upgraded_node = Node256::from(self);
                upgraded_node.add(values);
                NodeOption::Nx(Box::new(upgraded_node))
            } else {
                //add to self
                self.keys[cur_value_index] = Some(self.size as u8);
                let mut new_node = NodeOption::Nx(Box::new(Node4::new()));
                new_node.add(rest);
                self.children[self.size] = new_node;
                self.size += 1;
                NodeOption::Empty
            }
        } else {
            self.terminal = true;
            NodeOption::Empty
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

    fn as_any(&self) -> &dyn Any {
        self
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
            children: arr![NodeOption::Empty; 256],
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
    fn add(&mut self, values: &[u8]) -> NodeOption {
        if let Some((first, rest)) = values.split_first() {
            let cur_value_index = *first as usize;
            //if exists
            if let NodeOption::Nx(_) = &mut self.children[cur_value_index] {
                    let upgraded_node = self.children[cur_value_index].add(rest);
                    if upgraded_node.is_node() {
                        self.children[cur_value_index] = upgraded_node;
                    }
                    NodeOption::Empty
            } else {
                let mut new_node = NodeOption::Nx(Box::new(Node4::new()));
                new_node.add(rest);
                self.children[cur_value_index] = new_node;
                self.size += 1;
                NodeOption::Empty
            }
        } else {
            self.terminal = true;
            NodeOption::Empty
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Default for NodeOption {
    fn default() -> Self {
        NodeOption::Empty
    }
}

impl NodeOption {
    pub fn take(&mut self) -> NodeOption {
        mem::replace(self, Empty)
    }

    pub fn add(&mut self, value: &[u8]) -> Self {
        match self {
            NodeOption::Empty => Node4::new().add(value),
            NodeOption::Nx(n) => n.add(value)
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, NodeOption::Empty)
    }

    pub fn is_node(&self) -> bool {
        matches!(self, NodeOption::Nx(_))
    }

    pub fn unwrap(self) -> Box<dyn Node> {
        match self {
            NodeOption::Nx(val) => val,
            NodeOption::Empty => panic!("called `NodeOption::unwrap()` on an `Empty` value"),
        }
    }

    pub fn unwrap_or_default(self) -> Box<dyn Node> {
        match self {
            NodeOption::Nx(val) => val,
            NodeOption::Empty => Box::new(Node4::new()) //FIXME create default for Node trait
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn trial_run_test() {
    //     let mut node = NodeOption::N4(Box::new(Node4::new()));
    //     node = node.add("ab".as_bytes());
    //     node = node.add("ad".as_bytes());
    //     node = node.add("as".as_bytes());
    //     node = node.add("at".as_bytes());
    //     node = node.add("ace".as_bytes());
    //
    //     if let NodeOption::N4(root_node) = node {
    //         println!("root: {:#?}",root_node);
    //         if let NodeOption::N16(a_node) = &root_node.children[0] {
    //             println!("child 1: {:#?}",a_node);
    //         }
    //     }
    //     // println!("root: {:#?}",node);
    // }

    #[test]
    fn test_all_upgrades_occur_exact_match() {
        let mut node = NodeOption::Nx(Box::new(Node4::new()));
        for i in 0..=3 {
            let upgrade = node.add(&[i]);
            if let NodeOption::Nx(_) = upgrade {
                node = upgrade;
            }
        }
        if let NodeOption::Nx(n) = &node {
            assert!(n.is_full());
        }

        for i in 4..=15 {
            let upgrade = node.add(&[i]);
            if let NodeOption::Nx(_) = upgrade {
                node = upgrade;
            }
        }
        if let NodeOption::Nx(n) = &node {
            assert!(n.is_full());
        }

        for i in 16..=47 {
            let upgrade = node.add(&[i]);
            if let NodeOption::Nx(_) = upgrade {
                node = upgrade;
            }
        }
        if let NodeOption::Nx(n) = &node {
            assert!(n.is_full());
        }

        for i in 48..=255 {
            let upgrade = node.add(&[i]);
            if let NodeOption::Nx(_) = upgrade {
                node = upgrade;
            }
        }
        if let NodeOption::Nx(n) = &node {
            assert!(n.is_full());
        }
        // println!("{:#?}", node);
    }

    #[test]
    fn order_preserved_48_exact_match() {
        let mut node = NodeOption::Nx(Box::new(Node4::new()));

        for i in 0..48 {
            let upgrade =  node.add(&[i * 2]);
            if let NodeOption::Nx(_) = upgrade {
                node = upgrade;
            }
        }

        if let NodeOption::Nx(n) = node {
            let node48 = n.as_any().downcast_ref::<Node48>().unwrap();
            for (i, &k) in node48.keys.iter().enumerate() {
                if i < 96 { //only first entries 48 considered
                    match k {
                        None => {
                            assert_ne!(i % 2, 0);
                        },
                        Some(c) => {
                            assert_eq!(i % 2, 0);
                            assert!(matches!(&node48.children[c as usize], NodeOption::Nx(_)));
                        },
                        _ => panic!()
                    }
                }
            }
        } else {
            panic!()
        }
    }

    #[test]
    fn order_preserved_256_exact_match() {
        let mut node = NodeOption::Nx(Box::new(Node4::new()));

        for i in 0..=255 {
            if i % 2 == 0 {
                let upgrade = node.add(&[i]);
                if let NodeOption::Nx(_) = upgrade {
                    node = upgrade;
                }
            }
        }

        if let NodeOption::Nx(n) = node {
            let node256 = n.as_any().downcast_ref::<Node256>().unwrap();
            for (i, c) in node256.children.iter().enumerate() {
                match &c {
                    NodeOption::Empty => assert_ne!(i % 2, 0),
                    NodeOption::Nx(_) => assert_eq!(i % 2, 0),
                    _ => panic!()
                }
            }
        }
    }
}
