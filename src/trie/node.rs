use crate::trie::enums::{Case, Match};
use crate::trie::helpers;
use arr_macro::arr;

pub type Link = Option<Box<Node>>;

//optimize leaf so it doesnt store array
pub struct Node {
    children: [Link; 257],
}

pub enum NodeEnum {
    N4(Box<Node4>),
    N16(Box<Node16>),
    N48(Box<Node48>),
    N256(Box<Node256>),
}

//nodes need to upgrade to be able to upgrade to a new type
//define trait for Node
trait TrieNode {
    fn new() -> Self;
    fn add(&mut self, value: &String, match_type: &Match);
    fn exists(&self, c: char) -> Option<&NodeEnum>;
}

pub struct Node4 {
    keys: [char; 4],
    children: [NodeEnum; 4],
}

pub struct Node16 {
    keys: [char; 16],
    children: [NodeEnum; 16],
}

pub struct Node48 {
    keys: [u8; 256],
    children: [NodeEnum; 48],
}

pub struct Node256 {
    children: [NodeEnum; 256],
}

impl Node {
    //TODO: index json and store path
    //TODO: make variable length based off settings
    pub fn new() -> Self {
        Node {
            children: arr![None; 257],
        }
    }

    pub fn get_node(&self, i: usize) -> Option<&Node> {
        self.children[i].as_ref().map(|c| c.as_ref())
    }

    pub fn add(&mut self, value: &String, match_type: &Match) {
        match match_type {
            Match::Exact | Match::Prefix => {
                let mut cur = self;
                for c in value.chars() {
                    cur = match helpers::char_to_usize(c) {
                        None => cur, //ignore char and move to next one
                        Some(i) => cur.children[i].get_or_insert(Box::new(Node::new())),
                    }
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
                    for c in value[j..].chars() {
                        cur = match helpers::char_to_usize(c) {
                            None => cur,
                            Some(i) => cur.children[i].get_or_insert(Box::new(Node::new())),
                        }
                    }
                }
            }
        }
    }

    pub fn exists(&self, c: char) -> Option<&Node> {
        helpers::char_to_usize(c).and_then(|i| self.children[i].as_ref().map(|c| c.as_ref()))
    }
}
