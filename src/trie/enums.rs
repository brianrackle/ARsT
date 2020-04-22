pub enum Match {
    Exact,
    Prefix,
    PrefixPostfix,
}

pub enum Case {
    Sensitive,
    Insensitve,
}

enum LinkId {
    End,
    Char(char),
}
