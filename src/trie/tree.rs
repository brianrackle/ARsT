use crate::trie::enums::{Case, Match};
use crate::trie::node::{Node, Node4};
use std::borrow::Borrow;
use std::mem;

pub struct Trie {
    matching: Match,
    case: Case,
    root: Box<dyn Node>, //change this to Link for consistency
}

//FIXME Match should be implemented at tree level
// postfix = "cat", "at", "t"
// prefix = ignore terminals
//TODO: add fn options() for discovering autocomplete options
impl Trie {
    pub fn new(matching: Match, case: Case) -> Self {
        Trie {
            matching,
            case,
            root: Box::new(Node4::new())
        }
    }

    // //FIXME make iterative version l
    // fn add_to(node :&mut Box<dyn Node> , value :&[u8]) -> Option<Box<dyn Node>> {
    //     match value {
    //         [] => {
    //             node.terminate();
    //             None
    //         },
    //         [cur_value] => {
    //             if let Some(next_node) = node.add(cur_value) {
    //                 Trie::add_to(next_node, &[]);
    //                 Some(mem::replace(node, Box::new(Node4::new())))
    //             } else {
    //                 let mut node = node.upgrade();
    //                 let next_node = node.add(cur_value).expect("Internal error: Upgrade occurred but add still not possible");
    //                 Trie::add_to(next_node, &[]);
    //                 Some(node)
    //             }
    //         },
    //         [cur_value, remaining_values @ ..] => {
    //             if let Some(next_node) = node.add(cur_value) {
    //                 Trie::add_to(next_node, remaining_values)
    //             } else {
    //                 let mut node = node.upgrade();
    //                 let next_node = node.add(cur_value).expect("Internal error: Upgrade occurred but add still not possible");
    //                 Trie::add_to(next_node, remaining_values);
    //                 Some(node)
    //             }
    //         }
    //     }
    // }

    // pub fn add(&mut self, value: &str) {
    //     // Trie::add_to(&mut self.root, value.as_bytes())
    // }

    // pub fn add(&mut self, value: &str) {
    //     //enum Insert { Full, Found(&dyn Node), NotFound(&dyn Node>) }
    //     //find -> NotFound Exists
    //     let mut prev_node :Option<&mut dyn Node> = None;
    //     let mut cur_node = self.root.as_mut();
    //     for (i, byte) in value.as_bytes().iter().enumerate() {
    //         match cur_node.can_add(byte) {
    //             Location::Exists(index) => {
    //                 if let Some(child) = cur_node.insert(byte, Location::Exists(index)) {
    //                     cur_node = child.as_mut();
    //                 } else {
    //                     panic!()
    //                 }
    //             },
    //             Location::Insert(index) => {
    //                 if let Some(child) = cur_node.insert(byte, Location::Exists(index)) {
    //                     cur_node = child.as_mut();
    //                 } else {
    //                     panic!()
    //                 }
    //             },
    //             Location::Upgrade => {
    //                 if i == 0 {
    //                     self.root = cur_node.upgrade();
    //                     cur_node = self.root.as_mut();
    //                 } else {
    //                     prev_node = Some(cur_node);
    //                     cur_node = prev_node.unwrap().upgrade().as_mut();
    //                 }
    //                 if let Some(child) = cur_node.add(byte) {
    //                     cur_node = child.as_mut();
    //                 } else {
    //                     panic!("Internal error: Upgrade occurred but add still not possible");
    //                 }
    //             },
    //         }
    //     }
    // }


    // pub fn exists(&self, value: &str) -> bool {
    //     let mut cur = &self.index;
    //
    //     for c in self.set_case(value).bytes() {
    //         match cur.exists(c) {
    //             Some(t) => {
    //                 cur = &t;
    //             }
    //             None => return false,
    //         }
    //     }
    //
    //     //look for terminal character if exact match
    //     //use is_terminal on last node instead
    //     match self.matching {
    //         Match::Exact => cur.get_node(256).is_some(), //put in index 0 (\ or null)
    //         _ => true,
    //     }
    // }

    fn set_case(&self, value: &str) -> String {
        match self.case {
            Case::Insensitve => value.to_lowercase(),
            Case::Sensitive => String::from(value),
        }
    }
}

#[cfg(test)] //module should only be compiled for testing
mod test {
    use super::{Case, Match};

    //doesnt check terminal char
    // fn only_has_chars(n: &OldNode, s: &str) -> bool {
    //     for i in 0_u8..255_u8 {
    //         let contain = s.contains(&String::from(i as char));
    //         if contain != n.get_node(i as usize).is_some() {
    //             return false;
    //         }
    //     }
    //     true
    // }

    #[test]
    fn add_string_chars_exist() {
        // {
        //     //EXACT
        //     let mut trie = Trie::new(Match::Exact, Case::Sensitive);
        //     trie.add(&"abc");
        //
        //     let char_1 = trie.index.get_node(ctu('a')).unwrap();
        //     assert!(only_has_chars(&trie.index, "a"));
        //     let char_2 = char_1.get_node(ctu('b')).unwrap();
        //     assert!(only_has_chars(&char_1, "b"));
        //     let char_3 = char_2.get_node(ctu('c')).unwrap();
        //     assert!(only_has_chars(&char_2, "c"));
        //     let char_4 = char_3.get_node(256);
        //     assert!(char_4.is_some());
        // }
        //
        // {
        //     //PREFIX
        //     let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
        //     trie.add(&"abc");
        //
        //     let char_1 = trie.index.get_node(ctu('a')).unwrap();
        //     assert!(only_has_chars(&trie.index, "a"));
        //     let char_2 = char_1.get_node(ctu('b')).unwrap();
        //     assert!(only_has_chars(&char_1, "b"));
        //     let char_3 = char_2.get_node(ctu('c')).unwrap();
        //     assert!(only_has_chars(&char_2, "c"));
        //     let char_4 = char_3.get_node(256);
        //     assert!(!char_4.is_some());
        // }
        //
        // {
        //     //PREFIXPOSTFIX
        //     let mut trie = Trie::new(Match::PrefixPostfix, Case::Sensitive);
        //     trie.add(&"abcd");
        //
        //     let char_1 = trie.index.get_node(ctu('a')).unwrap();
        //     assert!(only_has_chars(&trie.index, "abcd"));
        //     let char_2 = char_1.get_node(ctu('b')).unwrap();
        //     assert!(only_has_chars(&char_2, "c"));
        // }
    }
    //
    // #[test]
    // fn match_empty() {
    //     let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
    //     trie.add(&"");
    //     assert!(trie.exists(&""))
    // }
    //
    // #[test]
    // fn match_no_empty() {
    //     let trie = Trie::new(Match::Prefix, Case::Sensitive);
    //     assert!(trie.exists(&""))
    // }
    //
    // #[test]
    // fn match_char() {
    //     let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
    //     trie.add(&"a");
    //
    //     assert!(trie.exists(&"a"));
    //     assert!(!trie.exists(&"A"));
    // }
    //
    // #[test]
    // fn match_string_case_sensitive() {
    //     {
    //         let mut trie = Trie::new(Match::Exact, Case::Sensitive);
    //         trie.add(&"abcde");
    //         trie.add(&"abc");
    //
    //         assert!(trie.exists(&"abcde"));
    //         assert!(trie.exists(&"abc"));
    //         assert!(!trie.exists(&"ab"));
    //         assert!(!trie.exists(&"ABCDE"));
    //     }
    //
    //     {
    //         let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
    //         trie.add(&"abcde");
    //         trie.add(&"abc");
    //
    //         assert!(trie.exists(&"abcde"));
    //         assert!(trie.exists(&"abc"));
    //         assert!(trie.exists(&"ab"));
    //         assert!(!trie.exists(&"ABCDE"));
    //     }
    //
    //     {
    //         let mut trie = Trie::new(Match::PrefixPostfix, Case::Sensitive);
    //         trie.add(&"abcde");
    //
    //         assert!(trie.exists(&"abcde"));
    //         assert!(trie.exists(&"abc"));
    //         assert!(trie.exists(&"ab"));
    //         assert!(trie.exists(&"bcde"));
    //         assert!(trie.exists(&"cd"));
    //         assert!(!trie.exists(&"ABCDE"));
    //     }
    // }
    //
    // #[test]
    // fn no_match_string() {
    //     let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
    //     trie.add(&"abc");
    //
    //     assert!(!trie.exists(&"bc"));
    //     assert!(!trie.exists(&"AB")); //partial complete match
    // }
}
