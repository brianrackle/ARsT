use arr_macro::arr;
use std::borrow::Borrow;

pub struct Trie {
    matching: Match,
    case: Case,
    index: Node, //change this to Link for consistency
}

pub enum Match {
    Exact,
    Prefix,
    PrefixPostfix,
}

pub enum Case {
    Sensitive,
    Insensitve,
}

type Link = Option<Box<Node>>;

//optimize leaf so it doesnt store array
struct Node {
    children: [Link; 257]
}

enum LinkId {
    End,
    Char(char),
}

impl Trie {
    pub fn new(matching: Match, case: Case) -> Self {
        Trie { matching: matching, case: case, index: Node::new() }
    }

    pub fn add(&mut self, value: &String) {
        value.chars().next().map(|_| self.index.add_some(value, self.matching.borrow()));
    }

    pub fn exists(&self, value: &String) -> bool {
        let mut cur = &self.index;

        for c in self.set_case(value).chars() {
            match cur.char_exists(c) {
                Some(t) => {
                    cur = &t;
                }
                None => return false
            }
        }

        //look for terminal character if exact match
        match self.matching {
            Match::Exact => cur.children[256].is_some(),
            _ => true
        }
    }

    fn set_case(&self, value: &String) -> String {
        match self.case {
            Case::Insensitve => value.to_lowercase(),
            Case::Sensitive => value.clone()
        }
    }
}

impl Node {
    //TODO: index json and store path
    //TODO: make variable length based off settings
    fn new() -> Self {
        Node { children: arr![None; 257] }
        //4   -> Keys [char; 4] / Links [None; 4]   -> Unsorted Linear search
        //16  -> Keys [char; 16] / Links [None; 16] -> Sorted Binary search
        //48  -> Keys [u8 (actually only need 6 bits; 256] / Links [None; 48]  -> Radix lookup
        //256 -> Links [None; 256]                  -> Radix lookup
    }

    fn add_some(&mut self, value: &String, match_type: &Match) {
        match match_type {
            Match::Exact | Match::Prefix => {
                let mut cur = self;
                for c in value.chars() {
                    cur = match Node::char_to_usize(c) {
                        None => cur, //ignore char and move to next one
                        Some(i) => cur.children[i].get_or_insert(Box::new(Node::new()))
                    }
                }
                //add terminal char when match is exact
                match match_type {
                    Match::Exact => cur.children[257 - 1] = Some(Box::new(Node::new())),
                    _ => ()
                }
            }
            Match::PrefixPostfix => {
                //takes 0+n first characters off string
                let mut cur: &mut Node;
                for j in 0..value.len() {
                    cur = self;
                    for c in value[j..].chars() {
                        cur = match Node::char_to_usize(c) {
                            None => cur,
                            Some(i) => cur.children[i].get_or_insert(Box::new(Node::new()))
                        }
                    }
                }
            }
        }
    }

    fn char_exists(&self, c: char) -> Option<&Node> {
        Node::char_to_usize(c).and_then(|i| {
            self.children[i].as_ref().map(|c| c.as_ref())
        })
    }

    fn char_to_usize(c: char) -> Option<usize> {
        let int = c as usize;
        match int {
            x if x < 257 => Some(x),
            _ => Option::None
        }
    }
}

#[cfg(test)] //module should only be compiled for testing
mod test {
    use super::{Trie, Node, Match, Case};

    fn ctu(c: char) -> usize {
        Node::char_to_usize(c).expect("ERR")
    }

    //doesnt check terminal char
    fn only_has_chars(n: &Node, s: String) -> bool {
        let mut result = true;
        for i in 0_u8..255_u8 {
            let mut schar = String::new();
            schar.push(i as char);
            let contain = s.contains(&schar);
            let should_contain = match &n.children[i as usize] {
                None => false,
                Some(c) => true
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
        { //EXACT
            let mut trie = Trie::new(Match::Exact, Case::Sensitive);
            trie.add(&"abc".to_owned());

            let char_1 = trie.index.children[ctu('a')].as_ref();
            assert!(only_has_chars(&trie.index, "a".to_owned()));
            let char_2 = char_1.expect("ERR").children[ctu('b')].as_ref();
            assert!(only_has_chars(&char_1.expect("ERR"), "b".to_owned()));
            let char_3 = char_2.expect("ERR").children[ctu('c')].as_ref();
            assert!(only_has_chars(&char_2.expect("ERR"), "c".to_owned()));
            let char_4 = char_3.expect("ERR").children[256].as_ref();
            assert!(char_4.is_some());
        }

        { //PREFIX
            let mut trie = Trie::new(Match::Prefix, Case::Sensitive);
            trie.add(&"abc".to_owned());

            let char_1 = trie.index.children[ctu('a')].as_ref();
            assert!(only_has_chars(&trie.index, "a".to_owned()));
            let char_2 = char_1.expect("ERR").children[ctu('b')].as_ref();
            assert!(only_has_chars(&char_1.expect("ERR"), "b".to_owned()));
            let char_3 = char_2.expect("ERR").children[ctu('c')].as_ref();
            assert!(only_has_chars(&char_2.expect("ERR"), "c".to_owned()));
            let char_4 = char_3.expect("ERR").children[256].as_ref();
            assert!(!char_4.is_some());
        }

        { //PREFIXPOSTFIX
            let mut trie = Trie::new(Match::PrefixPostfix, Case::Sensitive);
            trie.add(&"abcd".to_owned());

            let char_1 = trie.index.children[ctu('a')].as_ref();
            assert!(only_has_chars(&trie.index, "abcd".to_owned()));
            let char_2 = char_1.expect("ERR").children[ctu('b')].as_ref();
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