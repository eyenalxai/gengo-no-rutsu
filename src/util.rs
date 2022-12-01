use crate::types::{Check, Word};
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::BufReader;
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
    let bad_words = to_check.split_whitespace().collect::<Vec<&str>>();

    words
        .iter()
        .cloned()
        .filter(|word: &Word| {
            bad_words.iter().any(|bad_word| {
                let bad_word_lower = bad_word.to_lowercase().as_str().to_owned();
                word.is_non_native(bad_word_lower)
            })
        })
        .collect::<Vec<Word>>()
}

pub async fn words_answer(bot: Bot, msg: Message, words: Vec<Word>) -> ResponseResult<()> {
    let msg_text = match msg.text() {
        Some(b) => b,
        None => return respond(()),
    };

    let non_native_words = filter_native_words(words, msg_text.to_string());

    let answer_text = match non_native_words.is_empty() {
        true => "Английщины не обнаружено!".to_string(),
        false => {
            format!(
                "{}\nБерегите чистоту русского языка!",
                non_native_words.iter().fold("".to_string(), |acc, word| acc
                    + format!("{}\n", word).as_str())
            )
        }
    };

    match (non_native_words.is_empty(), msg.chat.is_private()) {
        (true, true) => {
            bot.send_message(msg.chat.id, answer_text)
                .reply_to_message_id(msg.id)
                .await?;
            respond(())
        }
        (true, false) => respond(()),
        (false, true) => {
            bot.send_message(msg.chat.id, answer_text)
                .reply_to_message_id(msg.id)
                .await?;
            respond(())
        }
        (false, false) => {
            if thread_rng().gen_range(0.0..1.0) <= 0.95 {
                return respond(());
            }
            bot.send_message(msg.chat.id, answer_text).await?;
            respond(())
        }
    }
}
