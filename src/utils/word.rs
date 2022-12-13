use crate::utils::morpheme::{WordData, PrefixData, Root, Loan};
use crate::utils::str::is_similar;

pub trait Check {
    fn is_non_native(&self, word: String) -> bool;
}

impl Check for Loan {
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

trait Cyrillic {
    fn is_cyrillic(&self) -> bool;
}

impl Cyrillic for char {
    fn is_cyrillic(&self) -> bool {
        matches!(self, 'а'..='я' | 'А'..='Я' | 'ё' | 'Ё')
    }
}

pub fn filter_native_words<'data>(words: &'data Vec<Loan>, prefixes: &'data Vec<PrefixData>, to_check: String) -> Vec<WordData<'data>> {
    let to_check_only_cyrillic_alphabetic = to_check
        .chars()
        .map(|c| if c.is_ascii_punctuation() { ' ' } else { c })
        .filter(|c| c.is_cyrillic() || c.is_whitespace())
        .collect::<String>()
        .to_lowercase();

    let words_to_check = to_check_only_cyrillic_alphabetic
        .split_whitespace()
        .collect::<Vec<&str>>();

    words_to_check
        .iter()
        .filter_map(|bad_word| {
            let bad_word = bad_word.trim_end_matches("ы");
            let loanroot = prefixes
                .iter()
                .find_map(|prefix: &PrefixData| {
                    let w_r: Option<&Loan> = words
                        .iter()
                        .find(|root| -> bool {
                            root.is_non_native(bad_word.trim_start_matches(&prefix.base).to_string())
                        });
                    match w_r {
                        Some(root) => Some(WordData(prefix, Root::Loanword(root))),
                        None => None
                    }
                });
            if loanroot != None {
                loanroot
            } else {
                prefixes
                .iter()
                .filter(|prefix| {
                    bad_word.len() > prefix.base.len() + 2 * 3 && prefix.target.len() > 0
                })
                .find_map(|prefix: &PrefixData| {
                    if &bad_word[..prefix.base.len()] == prefix.base {
                        Some(WordData(prefix, Root::Native(bad_word[prefix.base.len()..].to_string())))
                    } else {
                        None
                    }
                })
            }
            
        })
        .collect::<Vec<WordData>>()
}

#[cfg(test)]
mod word_tests {
    use crate::utils::word::{filter_native_words, Check, PrefixData, Loan, WordData, Root};

    #[test]
    fn test_is_non_native() {
        let word = Loan {
            non_native: "абберация".to_string(),
            native: vec![vec!["отклонение".to_string()]],
            inexact: "".to_string(),
            extra_normal_form: "аберация".to_string(),
            unrecognized_forms: "".to_string(),
        };

        assert!(word.is_non_native("абберация".to_string()));
        assert!(word.is_non_native("аберация".to_string()));
        assert!(!word.is_non_native("отклонение".to_string()));

        let word = Loan {
            non_native: "волонтёр".to_string(),
            native: vec![vec!["доброволец".to_string()]],
            inexact: "".to_string(),
            extra_normal_form: "".to_string(),
            unrecognized_forms: "".to_string(),
        };

        assert!(word.is_non_native("волонтёр".to_string()));
        assert!(!word.is_non_native("доброволец".to_string()));
    }

    #[test]
    fn test_filter_native_words() {
        let vec_words: Vec<Loan> = vec![
            Loan {
                non_native: "абберация".to_string(),
                native: vec![vec!["отклонение".to_string()]],
                inexact: "".to_string(),
                extra_normal_form: "аберация".to_string(),
                unrecognized_forms: "".to_string(),
            },
            Loan {
                non_native: "абрикос".to_string(),
                native: vec![vec!["желтослив".to_string()]],
                inexact: "".to_string(),
                extra_normal_form: "".to_string(),
                unrecognized_forms: "".to_string(),
            },
        ];

        const ABBERATION_INDEX: usize = 0;
        const АБРИКОС_INDEX: usize = 1;

        let vec_pref = vec![
            PrefixData{
                base:"".to_string(),
                target:"".to_string()
            },
            PrefixData{
                base:"прото".to_string(),
                target:"перво".to_string()
            },
            PrefixData{
                base:"мега".to_string(),
                target:"крупно".to_string()
            },
        ];

        let result = vec![
            WordData(&vec_pref[0], Root::Loanword(&vec_words[ABBERATION_INDEX])),
            WordData(&vec_pref[1], Root::Loanword(&vec_words[АБРИКОС_INDEX]))
        ];

        assert_eq!(
            filter_native_words(&vec_words, &vec_pref, "абберация,протоабрикос".to_string()),
            result
        );


        let result = vec![ WordData(&vec_pref[2], Root::Native("мозг".to_string())) ];

        assert_eq!(
            filter_native_words(&vec_words, &vec_pref, "мегамозг".to_string()),
            result
        );
    }

    #[test]
    fn test_fmt() {
        let prefix = PrefixData::default();
        let root = Loan {
            non_native: "абберация".to_string(),
            native: vec![vec!["отклонение".to_string()]],
            inexact: "".to_string(),
            extra_normal_form: "аберация".to_string(),
            unrecognized_forms: "".to_string(),
        };
        let word_one = WordData(&prefix, Root::Loanword(&root));

        assert_eq!(format!("{}", word_one), "Не абберация, а отклонение.");

        let kant_root = Loan {
            non_native: "кант".to_string(),
            native: vec![
                vec![
                    "оторочка".to_string(),
                    "тесьма".to_string(),
                    "выпушка".to_string(),
                    ],
                vec![
                    "края скользяка (у лыж и снегоката)".to_string(),
                ],],
            inexact: "род мыслителя Иммануила Канта".to_string(),
            extra_normal_form: "".to_string(),
            unrecognized_forms: "".to_string(),
        };
        let word_two = WordData(&prefix, Root::Loanword(&kant_root));

        assert_eq!(format!("{}", word_two), "Если вы имели в виду не род мыслителя Иммануила Канта, то будет правильно 1) оторочка, тесьма, выпушка 2) края скользяка (у лыж и снегоката)");
    }
}
