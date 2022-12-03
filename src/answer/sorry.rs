use teloxide::payloads::SendMessageSetters;
use teloxide::requests::{Requester, ResponseResult};
use teloxide::types::Message;
use teloxide::{respond, Bot};

pub fn sorry_filter(msg: Message, bot_id: u64) -> bool {
    if !msg.chat.is_group() && !msg.chat.is_supergroup() {
        return false;
    }

    let reply_to_message = match msg.reply_to_message() {
        Some(b) => b,
        None => return false,
    };

    let reply_to_message_from_id = match reply_to_message.from() {
        Some(b) => b.id.0,
        None => return false,
    };

    if reply_to_message_from_id != bot_id {
        return false;
    }

    let msg_text = match msg.text() {
        Some(b) => b,
        None => return false,
    };

    println!("msg_text: {}", msg_text);

    if msg_text.to_lowercase().contains("извинись") {
        return true;
    }

    return false;
}

pub async fn sorry_answer(bot: Bot, msg: Message) -> ResponseResult<()> {
    let msg_text = "Извиняюсь";

    bot.send_message(msg.chat.id, msg_text)
        .reply_to_message_id(msg.id)
        .await?;
    respond(())
}
