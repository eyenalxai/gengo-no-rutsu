use trigram::similarity;

fn is_same_first_char(first_str: &str, second_str: &str) -> bool {
    let second_str_first_char = match second_str.chars().next() {
        Some(c) => c,
        None => return false,
    };
    first_str.starts_with(second_str_first_char)
}

fn is_same_last_char(first_str: &str, second_str: &str) -> bool {
    let second_str_last_char = match second_str.chars().last() {
        Some(c) => c,
        None => return false,
    };
    first_str.ends_with(second_str_last_char)
}

fn starts_ends_same(first_str: &str, second_str: &str) -> bool {
    is_same_first_char(first_str, second_str) && is_same_last_char(first_str, second_str)
}

fn is_similar_length(str: &str, str_to_check: &str) -> bool {
    str.chars().count().abs_diff(str_to_check.chars().count()) <= 2
}

pub fn is_similar(str: &str, str_to_check: &str) -> bool {
    if !starts_ends_same(str, str_to_check) || !is_similar_length(str, str_to_check) {
        return false;
    }
    println!(
        "{} - {}, {}",
        similarity(str, str_to_check),
        str,
        str_to_check
    );
    similarity(str, str_to_check) >= 0.55
}

#[cfg(test)]
mod str_tests {
    use crate::utils::str::{is_same_first_char, is_similar};

    #[test]
    fn test_is_same_first_char() {
        assert!(is_same_first_char("hello", "h"));
        assert!(is_same_first_char("hello", "hello"));
        assert!(is_same_first_char("hello", "hello world"));
        assert!(!is_same_first_char("hello", "world"));
        assert!(!is_same_first_char("hello", ""));
        assert!(!is_same_first_char("", "hello"));
        assert!(!is_same_first_char("", ""));
    }

    #[test]
    fn test_is_similar() {
        assert!(is_similar("абберация", "аберация"));

        assert!(!is_similar("социальный", "асоциальный"));
        assert!(!is_similar("кек", "кеке"));
        assert!(!is_similar("запостить", "захостить"));
        assert!(!is_similar("запостить", "запустить"));
        assert!(!is_similar("кант", "канат"));
        assert!(!is_similar("систематик", "систематический"));
        assert!(!is_similar("систематик", "систематика"));
        assert!(!is_similar("систематика", "систематик"));
        assert!(!is_similar("систематик", "синематик"));
        assert!(!is_similar("систематик", "системник"));
    }
}
