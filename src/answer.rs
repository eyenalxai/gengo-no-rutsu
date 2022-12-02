use crate::word::types::Word;
use crate::word::word::filter_native_words;
use rand::{thread_rng, Rng};
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::{Requester, ResponseResult};
use teloxide::types::Message;
use teloxide::{respond, Bot};

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
