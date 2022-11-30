use crate::types::Words;
use std::fs::File;
use std::io::BufReader;
use strsim::normalized_damerau_levenshtein;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::{Requester, ResponseResult};
use teloxide::types::Message;
use teloxide::{respond, Bot};

pub fn get_words_from_json() -> Vec<Words> {
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

pub fn filter_native_words(words: Vec<Words>, to_check: String) -> Vec<Words> {
    let bad_words = to_check.split_whitespace().collect::<Vec<&str>>();
    words
        .iter()
        .cloned()
        .filter(|word| {
            bad_words.iter().any(|bad_word| {
                // bad_word to lowercase
                let bad_word_lower = bad_word.to_lowercase().as_str().to_owned();
                normalized_damerau_levenshtein(word.non_native.as_str(), &bad_word_lower) >= 0.9
                    || normalized_damerau_levenshtein(
                        word.extra_normal_form.as_str(),
                        &bad_word_lower,
                    ) >= 0.9
            })
        })
        .collect::<Vec<Words>>()
}

pub async fn words_answer(bot: Bot, msg: Message, words: Vec<Words>) -> ResponseResult<()> {
    let msg_text = match msg.text() {
        Some(text) => text,
        None => {
            log::error!("Failed to get text from message");
            panic!("Failed to get text from message");
        }
    };

    let non_native_words = filter_native_words(words, msg_text.to_string());

    match !non_native_words.is_empty() {
        true => {
            let answer = non_native_words.iter().fold("".to_string(), |acc, word| {
                acc + &format!("Не {}, а {}.\n", word.non_native, word.native)
            });
            bot.send_message(
                msg.chat.id,
                format!("{}\nБерегите чистоту русского языка!", answer),
            )
            .reply_to_message_id(msg.id)
            .await?;
            respond(())
        }

        false => respond(()),
    }
}
