use crate::trie::enums::{Case, Match};
use crate::trie::node::{OldLink, OldNode};
use std::borrow::Borrow;

pub struct Trie {
    matching: Match,
    case: Case,
    index: OldNode, //change this to Link for consistency
}

//FIXME Match should be implemented at tree level
// postfix = "cat", "at", "t"
// prefix = ignore terminals
//TODO: add fn options() for discovering autocomplete options
impl Trie {
    pub fn new(matching: Match, case: Case) -> Self {
        Trie {
            matching: matching,
            case: case,
            index: OldNode::new(),
        }
    }

    pub fn add(&mut self, value: &str) {
        if value.len() != 0 {
            self.index.add(value.as_bytes(), &self.matching)
        }
    }

    pub fn exists(&self, value: &str) -> bool {
        let mut cur = &self.index;

        for c in self.set_case(value).bytes() {
            match cur.exists(c) {
                Some(t) => {
                    cur = &t;
                }
                None => return false,
            }
        }

        //look for terminal character if exact match
        //use is_terminal on last node instead
        match self.matching {
            Match::Exact => cur.get_node(256).is_some(), //put in index 0 (\ or null)
            _ => true,
        }
    }

    fn set_case(&self, value: &str) -> String {
        match self.case {
            Case::Insensitve => value.to_lowercase(),
            Case::Sensitive => String::from(value),
        }
    }
}

#[cfg(test)] //module should only be compiled for testing
mod test {
    use super::{Case, Match, OldNode, Trie};

    //doesnt check terminal char
    fn only_has_chars(n: &OldNode, s: &str) -> bool {
        for i in 0_u8..255_u8 {
            let contain = s.contains(&String::from(i as char));
            if contain != n.get_node(i as usize).is_some() {
                return false;
            }
        }
        true
    }

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

    #[test]
    fn match_empty() {
        let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
        trie.add(&"");
        assert!(trie.exists(&""))
    }

    #[test]
    fn match_no_empty() {
        let trie = Trie::new(Match::Prefix, Case::Sensitive);
        assert!(trie.exists(&""))
    }

    #[test]
    fn match_char() {
        let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
        trie.add(&"a");

        assert!(trie.exists(&"a"));
        assert!(!trie.exists(&"A"));
    }

    #[test]
    fn match_string_case_sensitive() {
        {
            let mut trie = Trie::new(Match::Exact, Case::Sensitive);
            trie.add(&"abcde");
            trie.add(&"abc");

            assert!(trie.exists(&"abcde"));
            assert!(trie.exists(&"abc"));
            assert!(!trie.exists(&"ab"));
            assert!(!trie.exists(&"ABCDE"));
        }

        {
            let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
            trie.add(&"abcde");
            trie.add(&"abc");

            assert!(trie.exists(&"abcde"));
            assert!(trie.exists(&"abc"));
            assert!(trie.exists(&"ab"));
            assert!(!trie.exists(&"ABCDE"));
        }

        {
            let mut trie = Trie::new(Match::PrefixPostfix, Case::Sensitive);
            trie.add(&"abcde");

            assert!(trie.exists(&"abcde"));
            assert!(trie.exists(&"abc"));
            assert!(trie.exists(&"ab"));
            assert!(trie.exists(&"bcde"));
            assert!(trie.exists(&"cd"));
            assert!(!trie.exists(&"ABCDE"));
        }
    }

    #[test]
    fn no_match_string() {
        let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
        trie.add(&"abc");

        assert!(!trie.exists(&"bc"));
        assert!(!trie.exists(&"AB")); //partial complete match
    }
}
