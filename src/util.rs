use crate::types::{Check, Word};
use rand::{thread_rng, Rng};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use strsim::normalized_damerau_levenshtein;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::{Requester, ResponseResult};
use teloxide::types::Message;
use teloxide::{respond, Bot};

pub fn get_words_from_json() -> Vec<Word> {
    let file = match File::open("./words.json") {
        Ok(file) => file,
        Err(e) => panic!("Error opening file words.json: {}", e),
    };

    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(words) => words,
        Err(e) => panic!("Error reading file words.json: {}", e),
    }
}
fn filter_native_words(words: Vec<Word>, to_check: String) -> Vec<Word> {
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
        .filter(|word: &Word| {
            words_to_check.iter().any(|bad_word| {
                let bad_word_lower = bad_word.to_lowercase().as_str().to_owned();
                word.is_non_native(bad_word_lower)
            })
        })
        .collect::<Vec<Word>>()
}

fn build_answer_text(non_native_words: Vec<Word>) -> String {
    if non_native_words.is_empty() {
        return "Английщины не обнаружено!".to_string();
    }

    format!(
        "{}\nБерегите корни русского языка!",
        non_native_words.iter().fold("".to_string(), |acc, word| acc
            + format!("{}\n", word).as_str())
    )
}

pub async fn words_answer(bot: Bot, msg: Message, words: Vec<Word>) -> ResponseResult<()> {
    let msg_text = match msg.text() {
        Some(b) => b,
        None => return respond(()),
    };

    let non_native_words = filter_native_words(words, msg_text.to_string());
    let is_private_chat = !msg.chat.is_group() && !msg.chat.is_supergroup();

    match (non_native_words.is_empty(), is_private_chat) {
        (_, true) => {
            bot.send_message(msg.chat.id, build_answer_text(non_native_words))
                .reply_to_message_id(msg.id)
                .await?;
            respond(())
        }
        (true, false) => respond(()),
        (false, false) => {
            if thread_rng().gen_range(0.0..1.0) <= 0.90 {
                return respond(());
            }

            bot.send_message(msg.chat.id, build_answer_text(non_native_words))
                .reply_to_message_id(msg.id)
                .await?;
            respond(())
        }
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
