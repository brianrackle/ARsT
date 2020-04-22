//Allow search of attribute metadata and quick response with matching results
mod trie4;

// use std::borrow::Borrow;


fn main() {
    // datastructure for indexing documents
    // define which fields are searchable
    // perform different types of search
    //      -begins with, ends with, contains, regex, wildcard
    //supports AND, OR, NOT
    //supports nesting
    //typo tolerance
    //see: https://github.com/typesense/typesense

    //hashmap<Token, [] indices>
    //dont tokenize '?

    //tries? or something else
    //partial match can use trie of all suffixes
    // {
    //     let init_now = Instant::now();
    //     let test_data = input_data10k.iter().cloned().collect::<BTreeSet<&str>>();
    //     let init_time = init_now.elapsed().as_nanos();
    //
    //     let now = Instant::now();
    //     let result = test_data.contains(&"3747776229");
    //     let eval_time = now.elapsed().as_nanos();
    //     println!("BTR   Result:{} Total:{:10} Init:{:10} Execute:{:10}", result, init_time + eval_time, init_time, eval_time);
    // }
    // let data = ["aA", "Brian", "Anjuli", "Jessica", "Jon", "Jonathan", "Sam", "Rackle", "Rachael", "Rolando"];
    // let mut trie = trie1::Trie::new();
    // for d in &data {
    //     trie.add(&String::from(*d));
    // }
}