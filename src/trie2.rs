pub struct Trie {
    match_type: MatchType,
    index: Node,
}

pub enum MatchType {
    Full,
    Prefix,
    Postfix,
    PrefixPostfix,
}


type Link = Option<Box<Node>>;

struct Node {
    children: [Link; Node::CHILD_COUNT]
}

enum LinkId {
    End,
    Char(char),
}

impl Trie {
    pub fn new(match_type: MatchType) -> Self {
        Trie { match_type: match_type, index: Node { children: Default::default() } }
    }

    pub fn add(&mut self, value: &String, full: bool) {
        if full {
            value.chars().next().map(|_| self.index.add_some(value, full));
        } else {
            unimplemented!()
        }
    }

    pub fn exists(&self, value: &String) -> bool {
        let mut cur = &self.index;

        for c in value.to_lowercase().chars() {
            match cur.char_exists(c) {
                Some(t) => {
                    cur = &t;
                }
                None => return false
            }
        }
        true
    }
}


impl Node {
    //case insensitive alphabet: 26
    //case sensitive alphabet: 52
    //digits: 10 (48 - 57)
    const CHILD_COUNT: usize = 27; //27 is terminal char signifying end of sequence

    fn new() -> Self {
        Node { children: Default::default() }
    }

    //TODO: make iterative
    fn add_some(&mut self, value: &String, full: bool) {
        if full { //allow full matching
            match value.chars().next() {
                None => self.children[Node::CHILD_COUNT - 1] = Some(Box::new(Node::new())),
                Some(c) => self.add_char(c, value, full)
            };
        } else { //allow partial matching
            unimplemented!()
        }
    }

    fn add_char(&mut self, c: char, value: &String, full: bool) {
        match Node::char_to_usize(c) {
            None => (),
            Some(i) => self.children[i].get_or_insert(Box::new(Node::new())).add_some(&value[1..].to_owned(), full)
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
            x if x >= 97 && x < 97 + 26 => Some(x - 97),
            _ => Option::None
        }
    }
}

#[cfg(test)] //module should only be compiled for testing
mod test {
    use super::Trie;
    use super::MatchType;
    // use rand::{thread_rng, Rng};


    #[test]
    fn add_char_exist() {
        let mut trie = Trie::new(MatchType::Prefix);
        trie.add(&"a".to_owned(), true);
        assert!(trie.index.children[0].is_some());
    }

    #[test]
    fn add_string_chars_exist_full() {
        let mut trie = Trie::new(MatchType::Prefix);
        trie.add(&"aabc".to_owned(), true);
        {
            let char_1 = trie.index.children[0].as_ref();
            let char_2 = char_1.expect("ERR").children[0].as_ref();
            let char_3 = char_2.expect("ERR").children[1].as_ref();
            let char_4 = &char_3.expect("ERR").children[2].as_ref();
            assert!(char_1.is_some());
            assert!(char_2.is_some());
            assert!(char_3.is_some());
            assert!(char_4.is_some());
        }
    }

    #[test]
    fn add_string_chars_dont_exist_full() {
        let mut trie = Trie::new(MatchType::Prefix);
        trie.add(&"aabc".to_owned(), true);
        let valid_char_1 = trie.index.children[0].as_ref();
        let valid_char_2 = valid_char_1.expect("ERR").children[0].as_ref();
        let valid_char_3 = valid_char_2.expect("ERR").children[1].as_ref();

        let invalid_char_1 = &trie.index.children[1];
        let invalid_char_2 = &valid_char_1.expect("ERR").children[1];
        let invalid_char_3 = &valid_char_2.expect("ERR").children[2];
        let invalid_char_4 = &valid_char_3.expect("ERR").children[3];

        assert!(invalid_char_1.is_none());
        assert!(invalid_char_2.is_none());
        assert!(invalid_char_3.is_none());
        assert!(invalid_char_4.is_none());
    }

    #[test]
    fn match_empty() {
        let mut trie = Trie::new(MatchType::Prefix);
        trie.add(&"".to_owned(), true);
        assert!(trie.exists(&"".to_owned()))
    }

    #[test]
    fn match_no_empty() {
        let trie = Trie::new(MatchType::Prefix);
        assert!(trie.exists(&"".to_owned()))
    }

    #[test]
    fn match_char() {
        let mut trie = Trie::new(MatchType::Prefix);
        trie.add(&"a".to_owned(), true);

        assert!(trie.exists(&"a".to_owned()));
        assert!(trie.exists(&"A".to_owned()));
    }

    #[test]
    fn match_string() {
        let mut trie = Trie::new(MatchType::Prefix);
        trie.add(&"abc".to_owned(), true);

        assert!(trie.exists(&"abc".to_owned()));
        assert!(trie.exists(&"ABC".to_owned()));
    }

    #[test]
    fn no_match_string() {
        let mut trie = Trie::new(MatchType::Prefix);
        trie.add(&"abc".to_owned(), true);

        assert!(!trie.exists(&"bc".to_owned()));
        assert!(trie.exists(&"AB".to_owned())); //partial complete match
    }

    #[test]
    fn large_data() {
        // let mut trie = Trie::new(MatchType::Prefix);
        // for _ in 0..100000 {
        //     let s: String = thread_rng().gen_ascii_chars().take(10).collect();
        // }
    }
}