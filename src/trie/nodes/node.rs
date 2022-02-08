use std::any::Any;

use std::cmp::Ordering;
use std::fmt::Debug;
use crate::trie::nodes::node::NodeLocation::{Exists, Insert};

//FIXME remove is_empty and any other unused method
pub trait Node: Debug {
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_terminal(&self) -> bool;
    fn set_terminal(&mut self, terminal: bool);
    fn as_any(&self) -> &dyn Any;
    fn exists(&self, values: &[u8]) -> bool;

    fn add(&mut self, values: &[u8]) -> NodeOption {
        if let Some((first, rest)) = values.split_first() {
            match &self.get_index(*first) {
                Exists(index) => self.exists_add(index, rest),
                Insert(index) => self.insert_add(index, *first, rest),
                Upgrade => self.upgrade_add(values),
            }
        } else {
            self.set_terminal(true);
            None
        }
    }

    //TODO break into different trait
    fn get_index(&self, value: u8) -> NodeLocation;
    fn exists_add(&mut self, index: &KeyChildIndex, rest: &[u8]) -> NodeOption;
    fn insert_add(&mut self, index: &KeyChildIndex, first: u8, rest: &[u8]) -> NodeOption;
    fn upgrade_add(&mut self, values: &[u8]) -> NodeOption;
}


//see: https://www.the-paper-trail.org/post/art-paper-notes/
pub type NodeOption = Option<Box<dyn Node>>;

pub enum NodeLocation {
    Exists(KeyChildIndex), //key child
    Insert(KeyChildIndex),
    Upgrade
}

pub struct KeyChildIndex {
    pub(crate) key: usize,
    pub(crate) child: usize
}

pub fn val_cmp(a: &Option<u8>, b: &Option<u8>) -> Ordering {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trie::nodes::{
        node0::Node0, node16::Node16, node256::Node256, node4::Node4, node48::Node48,
    };

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
        let mut node = NodeOption::Some(Box::new(Node0::new()));
        for i in 0..=3 {
            let upgrade = node.as_mut().unwrap().add(&[i]);
            if upgrade.is_some() {
                node = upgrade;
            }
        }

        assert!(node
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<Node4>()
            .is_some());
        assert!(node.as_ref().unwrap().is_full());

        for i in 4..=15 {
            let upgrade = node.as_mut().unwrap().add(&[i]);
            if upgrade.is_some() {
                node = upgrade;
            }
        }

        assert!(node
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<Node16>()
            .is_some());
        assert!(node.as_ref().unwrap().is_full());

        for i in 16..=47 {
            let upgrade = node.as_mut().unwrap().add(&[i]);
            if upgrade.is_some() {
                node = upgrade;
            }
        }

        assert!(node
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<Node48>()
            .is_some());
        assert!(node.as_ref().unwrap().is_full());

        for i in 48..=255 {
            let upgrade = node.as_mut().unwrap().add(&[i]);
            if upgrade.is_some() {
                node = upgrade;
            }
        }

        assert!(node
            .as_ref()
            .unwrap()
            .as_any()
            .downcast_ref::<Node256>()
            .is_some());
        assert!(node.as_ref().unwrap().is_full());
    }
}
