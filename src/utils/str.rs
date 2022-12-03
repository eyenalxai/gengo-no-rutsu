use strsim::jaro_winkler;

pub fn is_same_first_char(first_str: &str, second_str: &str) -> bool {
    let second_str_first_char = match second_str.chars().next() {
        Some(c) => c,
        None => return false,
    };
    first_str.starts_with(second_str_first_char)
}

pub fn is_similar(first_str: &str, second_str: &str) -> bool {
    is_same_first_char(first_str, second_str) && jaro_winkler(first_str, second_str) >= 0.95
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
        assert!(is_similar("абберация", "абберации"));
        assert!(is_similar("абберация", "аберация"));
        assert!(is_similar("идеология", "идеологии"));
        assert!(is_similar("социальный", "социальные"));
        assert!(!is_similar("социальный", "асоциальный"));
        assert!(!is_similar("кек", "кеке"));
        assert!(!is_similar("запостить", "захостить"));
        assert!(!is_similar("запостить", "запустить"));
    }
}
