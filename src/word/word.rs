use crate::str::is_similar;
use crate::word::types::{Check, Word};
use std::fmt::{Display, Formatter};

impl Check for Word {
    fn is_non_native(&self, non_native_word: String) -> bool {
        let non_native_word_str = non_native_word.as_str();
        let unrecognized_forms = self
            .unrecognized_forms
            .split(',')
            .filter(|x| !x.is_empty())
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        return match is_similar(self.non_native.as_str(), non_native_word_str) {
            true => true,
            false => {
                match !self.extra_normal_form.is_empty()
                    && is_similar(self.extra_normal_form.as_str(), non_native_word_str)
                {
                    true => true,
                    false => unrecognized_forms.iter().any(|unrecognized_form| {
                        is_similar(unrecognized_form, non_native_word_str)
                    }),
                }
            }
        };
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
