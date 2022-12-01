use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use strsim::normalized_damerau_levenshtein;

#[derive(Debug, Clone, Copy)]
pub enum PollingMode {
    Polling,
    Webhook,
}

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

impl Check for Word {
    fn is_non_native(&self, non_native_word: String) -> bool {
        let non_native_word_str = non_native_word.as_str();
        let unrecognized_forms = self
            .unrecognized_forms
            .split(',')
            .filter(|x| !x.is_empty())
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        return match normalized_damerau_levenshtein(self.non_native.as_str(), non_native_word_str)
            >= 0.9
        {
            true => true,
            false => {
                match !self.extra_normal_form.is_empty()
                    && normalized_damerau_levenshtein(
                        self.extra_normal_form.as_str(),
                        non_native_word_str,
                    ) >= 0.9
                {
                    true => true,
                    false => unrecognized_forms.iter().any(|unrecognized_form| {
                        normalized_damerau_levenshtein(unrecognized_form, non_native_word_str)
                            >= 0.9
                    }),
                }
            }
        };
    }
}
