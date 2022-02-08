use crate::trie::enums::{Case, Match};
use crate::trie::nodes::node::{Node, NodeOption};
use crate::trie::nodes::node0::Node0;

pub struct Tree {
    matching: Match,
    case: Case,
    root: NodeOption,
}

//TODO: add fn options() for discovering autocomplete options
impl Tree {
    pub fn new(matching: Match, case: Case) -> Self {
        Tree {
            matching,
            case,
            root: NodeOption::default(),
        }
    }

    pub fn add(&mut self, value: &str) {
        if !value.is_empty() {
            let case_corrected = match self.case {
                Case::Insensitve => value.to_lowercase(),
                Case::Sensitive => String::from(value),
            };

            let upgraded_node = self.root.as_mut().map_or_else(
                || Box::new(Node0::new()).add(case_corrected.as_bytes()),
                |v| v.add(case_corrected.as_bytes()),
            );
            if upgraded_node.is_some() {
                self.root = upgraded_node;
            }
        }
    }

    pub fn exists(&self, value: &str) -> bool {
        let case_corrected = match self.case {
            Case::Insensitve => value.to_lowercase(),
            Case::Sensitive => String::from(value),
        };

        if let Some(node) = self.root.as_ref() {
            node.exists(case_corrected.as_bytes())
        } else {
            false
        }
    }
}

#[cfg(test)] //module should only be compiled for testing
mod test {
    use super::{Case, Match, Tree};
    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader, Lines};
    use std::path::PathBuf;

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

    pub fn english_dict() -> Lines<BufReader<File>> {
        let file = File::open(PathBuf::from("src/test/dictionary.txt")).unwrap();
        io::BufReader::new(file).lines()
    }

    #[test]
    fn test_building_english_dictionary() {
        let mut tree = Tree::new(Match::Exact, Case::Insensitve);

        let dict = english_dict().map(|l| l.unwrap()).collect::<Vec<_>>();
        for word in &dict {
            tree.add(word);
        }

        for word in &dict {
            assert!(tree.exists(word));
        }
    }

    // #[bench]
    // fn bench_building_english_dictionary(b :&mut Bencher) {
    //     let mut root = NodeOption::Some(Box::new(Node0::new()));
    //     let dict = english_dict().map(|l| l.unwrap()).collect::<Vec<_>>();
    //     b.iter(|| {
    //         for word in dict {
    //             let upgrade = root.as_mut().unwrap().add(word.as_bytes());
    //             if upgrade.is_some() {
    //                 root = upgrade;
    //             }
    //         }
    //         root
    //     });
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
