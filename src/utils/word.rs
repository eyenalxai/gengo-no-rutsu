use crate::utils::str::is_similar;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Word {
    pub non_native: String,
    pub native: String,
    pub inexact: String,
    pub extra_normal_form: String,
    pub unrecognized_forms: String,
}

pub trait Check {
    fn is_non_native(&self, word: String) -> bool;
}

impl Check for Word {
    fn is_non_native(&self, word_to_check: String) -> bool {
        let non_native_word_str = word_to_check.as_str();
        let unrecognized_forms = self
            .unrecognized_forms
            .split(',')
            .filter(|x| !x.is_empty())
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        if is_similar(self.non_native.as_str(), non_native_word_str) {
            return true;
        }

        if !self.extra_normal_form.is_empty()
            && is_similar(self.extra_normal_form.as_str(), non_native_word_str)
        {
            return true;
        }

        unrecognized_forms
            .iter()
            .any(|unrecognized_form| is_similar(unrecognized_form, non_native_word_str))
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.inexact.is_empty() {
            return write!(
                f,
                "Если вы имели в виду не {}, то будет правильно {}",
                self.inexact, self.native
            );
        }
        write!(f, "Не {}, а {}.", self.non_native, self.native)
    }
}

pub fn filter_native_words(words: Vec<Word>, to_check: String) -> Vec<Word> {
    let to_check_only_alphabetic = to_check
        .chars()
        .filter(|c| !r#"!@#№$%:%^,.&*;()_-–—+=[]{}:"'|\?/<>~"#.contains(*c))
        .collect::<String>();

    let words_to_check = to_check_only_alphabetic
        .split_whitespace()
        .collect::<Vec<&str>>();

    words
        .iter()
        .cloned()
        .filter(|word: &Word| -> bool {
            words_to_check.iter().any(|bad_word| {
                let bad_word_lower = bad_word.to_lowercase().as_str().to_owned();
                word.is_non_native(bad_word_lower)
            })
        })
        .collect::<Vec<Word>>()
}

#[cfg(test)]
mod word_tests {
    use super::*;

    #[test]
    fn test_is_non_native() {
        let word = Word {
            non_native: "абберация".to_string(),
            native: "отклонение".to_string(),
            inexact: "".to_string(),
            extra_normal_form: "аберация".to_string(),
            unrecognized_forms: "".to_string(),
        };

        assert!(word.is_non_native("абберация".to_string()));
        assert!(word.is_non_native("аберация".to_string()));
        assert!(!word.is_non_native("отклонение".to_string()));
    }

    #[test]
    fn test_fmt() {
        let word_one = Word {
            non_native: "абберация".to_string(),
            native: "отклонение".to_string(),
            inexact: "".to_string(),
            extra_normal_form: "аберация".to_string(),
            unrecognized_forms: "".to_string(),
        };

        assert_eq!(format!("{}", word_one), "Не абберация, а отклонение.");

        let word_two = Word {
            non_native: "кант".to_string(),
            native: "1) оторочка, тесьма, выпушка 2) края скользяка (у лыж и снегоката)"
                .to_string(),
            inexact: "род мыслителя Иммануила Канта".to_string(),
            extra_normal_form: "".to_string(),
            unrecognized_forms: "".to_string(),
        };

        assert_eq!(format!("{}", word_two), "Если вы имели в виду не род мыслителя Иммануила Канта, то будет правильно 1) оторочка, тесьма, выпушка 2) края скользяка (у лыж и снегоката)");
    }
}
