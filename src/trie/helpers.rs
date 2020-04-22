pub fn char_to_usize(c: char) -> Option<usize> {
    let int = c as usize;
    match int {
        x if x < 257 => Some(x),
        _ => Option::None,
    }
}