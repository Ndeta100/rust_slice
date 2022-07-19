pub fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

pub fn second_word(s: &String) -> (usize, usize) {
    let bytes = s.as_bytes();
    let mut first_index = 0;
    let mut found_first = false;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && !found_first {
            first_index = i + 1;
            found_first = true;
        } else if item == b' ' && found_first {
            return (first_index, (i - 1));
        }
    }
    (s.len(), s.len())
}
