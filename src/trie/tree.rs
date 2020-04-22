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

    pub fn add(&mut self, value: &String) {
        value
            .chars()
            .next()
            .map(|_| self.index.add(value, self.matching.borrow()));
    }

    pub fn exists(&self, value: &String) -> bool {
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

    fn set_case(&self, value: &String) -> String {
        match self.case {
            Case::Insensitve => value.to_lowercase(),
            Case::Sensitive => value.clone(),
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
    fn only_has_chars(n: &Node, s: String) -> bool {
        let mut result = true;
        for i in 0_u8..255_u8 {
            let mut schar = String::new();
            schar.push(i as char);
            let contain = s.contains(&schar);
            let should_contain = match &n.get_node(i as usize) {
                None => false,
                Some(c) => true,
            };
            if contain != should_contain {
                result = false;
                break;
            }
        }
        result
    }

    #[test]
    fn add_string_chars_exist() {
        {
            //EXACT
            let mut trie = Trie::new(Match::Exact, Case::Sensitive);
            trie.add(&"abc".to_owned());

            let char_1 = trie.index.get_node(ctu('a'));
            assert!(only_has_chars(&trie.index, "a".to_owned()));
            let char_2 = char_1.expect("ERR").get_node(ctu('b'));
            assert!(only_has_chars(&char_1.expect("ERR"), "b".to_owned()));
            let char_3 = char_2.expect("ERR").get_node(ctu('c'));
            assert!(only_has_chars(&char_2.expect("ERR"), "c".to_owned()));
            let char_4 = char_3.expect("ERR").get_node(256);
            assert!(char_4.is_some());
        }

        {
            //PREFIX
            let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
            trie.add(&"abc".to_owned());

            let char_1 = trie.index.get_node(ctu('a'));
            assert!(only_has_chars(&trie.index, "a".to_owned()));
            let char_2 = char_1.expect("ERR").get_node(ctu('b'));
            assert!(only_has_chars(&char_1.expect("ERR"), "b".to_owned()));
            let char_3 = char_2.expect("ERR").get_node(ctu('c'));
            assert!(only_has_chars(&char_2.expect("ERR"), "c".to_owned()));
            let char_4 = char_3.expect("ERR").get_node(256);
            assert!(!char_4.is_some());
        }

        {
            //PREFIXPOSTFIX
            let mut trie = Trie::new(Match::PrefixPostfix, Case::Sensitive);
            trie.add(&"abcd".to_owned());

            let char_1 = trie.index.get_node(ctu('a'));
            assert!(only_has_chars(&trie.index, "abcd".to_owned()));
            let char_2 = char_1.expect("ERR").get_node(ctu('b'));
            assert!(only_has_chars(&char_2.expect("ERR"), "c".to_owned()));
        }
    }

    #[test]
    fn match_empty() {
        let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
        trie.add(&"".to_owned());
        assert!(trie.exists(&"".to_owned()))
    }

    #[test]
    fn match_no_empty() {
        let trie = Trie::new(Match::Prefix, Case::Sensitive);
        assert!(trie.exists(&"".to_owned()))
    }

    #[test]
    fn match_char() {
        let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
        trie.add(&"a".to_owned());

        assert!(trie.exists(&"a".to_owned()));
        assert!(!trie.exists(&"A".to_owned()));
    }

    #[test]
    fn match_string_case_sensitive() {
        {
            let mut trie = Trie::new(Match::Exact, Case::Sensitive);
            trie.add(&"abcde".to_owned());
            trie.add(&"abc".to_owned());

            assert!(trie.exists(&"abcde".to_owned()));
            assert!(trie.exists(&"abc".to_owned()));
            assert!(!trie.exists(&"ab".to_owned()));
            assert!(!trie.exists(&"ABCDE".to_owned()));
        }

        {
            let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
            trie.add(&"abcde".to_owned());
            trie.add(&"abc".to_owned());

            assert!(trie.exists(&"abcde".to_owned()));
            assert!(trie.exists(&"abc".to_owned()));
            assert!(trie.exists(&"ab".to_owned()));
            assert!(!trie.exists(&"ABCDE".to_owned()));
        }

        {
            let mut trie = Trie::new(Match::PrefixPostfix, Case::Sensitive);
            trie.add(&"abcde".to_owned());

            assert!(trie.exists(&"abcde".to_owned()));
            assert!(trie.exists(&"abc".to_owned()));
            assert!(trie.exists(&"ab".to_owned()));
            assert!(trie.exists(&"bcde".to_owned()));
            assert!(trie.exists(&"cd".to_owned()));
            assert!(!trie.exists(&"ABCDE".to_owned()));
        }
    }

    #[test]
    fn no_match_string() {
        let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
        trie.add(&"abc".to_owned());

        assert!(!trie.exists(&"bc".to_owned()));
        assert!(!trie.exists(&"AB".to_owned())); //partial complete match
    }
}
