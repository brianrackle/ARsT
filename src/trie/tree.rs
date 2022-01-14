use crate::trie::enums::{Case, Match};
use crate::trie::node::{Link, Node, Node16, Node256, Node4, Node48, NodeEnum};
use std::borrow::Borrow;

pub struct Trie {
    matching: Match,
    case: Case,
    index: Node, //change this to Link for consistency
}

impl Trie {
    pub fn new(matching: Match, case: Case) -> Self {
        Trie {
            matching: matching,
            case: case,
            index: Node::new(),
        }
    }

    pub fn add(&mut self, value: &str) {
        value
            .chars()
            .next()
            .map(|_| self.index.add(value, &self.matching));
    }

    pub fn exists(&self, value: &str) -> bool {
        let mut cur = &self.index;

        for c in self.set_case(value).chars() {
            match cur.exists(c) {
                Some(t) => {
                    cur = &t;
                }
                None => return false,
            }
        }

        //look for terminal character if exact match
        match self.matching {
            Match::Exact => cur.get_node(256).is_some(),
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
    use super::{Case, Match, Node, Trie};
    use crate::trie::helpers;

    fn ctu(c: char) -> usize {
        helpers::char_to_usize(c).expect("ERR")
    }

    //doesnt check terminal char
    fn only_has_chars(n: &Node, s: &str) -> bool {
        for i in 0_u8..255_u8 {
            let contain = s.contains(&String::from(i as char));
            if contain != n.get_node(i as usize).is_some() {
                return false
            }
        }
        true
    }

    #[test]
    fn add_string_chars_exist() {
        {
            //EXACT
            let mut trie = Trie::new(Match::Exact, Case::Sensitive);
            trie.add(&"abc");

            let char_1 = trie.index.get_node(ctu('a')).unwrap();
            assert!(only_has_chars(&trie.index, "a"));
            let char_2 = char_1.get_node(ctu('b')).unwrap();
            assert!(only_has_chars(&char_1, "b"));
            let char_3 = char_2.get_node(ctu('c')).unwrap();
            assert!(only_has_chars(&char_2, "c"));
            let char_4 = char_3.get_node(256);
            assert!(char_4.is_some());
        }

        {
            //PREFIX
            let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
            trie.add(&"abc");

            let char_1 = trie.index.get_node(ctu('a')).unwrap();
            assert!(only_has_chars(&trie.index, "a"));
            let char_2 = char_1.get_node(ctu('b')).unwrap();
            assert!(only_has_chars(&char_1, "b"));
            let char_3 = char_2.get_node(ctu('c')).unwrap();
            assert!(only_has_chars(&char_2, "c"));
            let char_4 = char_3.get_node(256);
            assert!(!char_4.is_some());
        }

        {
            //PREFIXPOSTFIX
            let mut trie = Trie::new(Match::PrefixPostfix, Case::Sensitive);
            trie.add(&"abcd");

            let char_1 = trie.index.get_node(ctu('a')).unwrap();
            assert!(only_has_chars(&trie.index, "abcd"));
            let char_2 = char_1.get_node(ctu('b')).unwrap();
            assert!(only_has_chars(&char_2, "c"));
        }
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
