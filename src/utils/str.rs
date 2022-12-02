use strsim::normalized_damerau_levenshtein;

pub fn is_same_first_char(first_str: &str, second_str: &str) -> bool {
    let second_str_first_char = match second_str.chars().next() {
        Some(c) => c,
        None => return false,
    };
    first_str.starts_with(second_str_first_char)
}

pub fn is_similar(first_str: &str, second_str: &str) -> bool {
    is_same_first_char(first_str, second_str)
        && normalized_damerau_levenshtein(first_str, second_str) >= 0.9
}
