use std::env;
use teloxide::{dptree, Bot};

use teloxide::dispatching::update_listeners::webhooks;
use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::error_handlers::LoggingErrorHandler;
use teloxide::requests::Requester;
use teloxide::types::Update;

mod utils {
    pub mod constant;
    pub mod listener;
    pub mod parse;
    pub mod str;
    pub mod word;
}

mod answer {
    pub mod sorry;
    pub mod words;
}

use crate::answer::words::words_answer;

use crate::answer::sorry::{sorry_answer, sorry_filter};
use crate::utils::listener::axum_server;
use crate::utils::parse::get_words_from_json;
use crate::utils::word::Word;
use url::Url;

#[derive(Debug, Clone, Copy)]
pub enum PollingMode {
    Polling,
    Webhook,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();

    let bot = Bot::from_env();
    let bot_id = bot.get_me().await.expect("Failed to get bot info").id.0;

    let polling_mode = match env::var("POLLING_MODE") {
        Ok(mode) => match mode.as_str() {
            "POLLING" => PollingMode::Polling,
            "WEBHOOK" => PollingMode::Webhook,
            _ => panic!("Unknown polling mode: {}", mode),
        },
        Err(_) => panic!("POLLING_MODE env var is not set, probably..."),
    };

    let words: Vec<Word> = get_words_from_json("./words.json");

    let words_handler = Update::filter_message()
        .branch(dptree::filter(sorry_filter).endpoint(sorry_answer))
        .branch(dptree::endpoint(words_answer));

    match polling_mode {
        PollingMode::Polling => {
            log::info!("Polling!");

            Dispatcher::builder(bot, words_handler)
                .dependencies(dptree::deps![words, bot_id])
                .enable_ctrlc_handler()
                .build()
                .dispatch()
                .await
        }

        PollingMode::Webhook => {
            let port: u16 = env::var("PORT")
                .expect("PORT env variable is not set")
                .parse()
                .expect("PORT env variable value is not an integer");

            let domain = env::var("DOMAIN").expect("DOMAIN env variable is not set");
            let url: Url = match format!("https://{domain}/webhook/main").parse() {
                Ok(url) => url,
                Err(err) => panic!("Failed to parse URL: {}", err),
            };

            log::info!("Webhook!");
            log::info!("Port: {}", port.clone().to_string());
            log::info!("URL: {}", url.clone().to_string());

            let addr = ([0, 0, 0, 0], port).into();
            let listener = axum_server(bot.clone(), webhooks::Options::new(addr, url))
                .await
                .expect("Couldn't setup webhook");

            Dispatcher::builder(bot, words_handler)
                .dependencies(dptree::deps![words, bot_id])
                .enable_ctrlc_handler()
                .build()
                .dispatch_with_listener(listener, LoggingErrorHandler::new())
                .await
        }
    }
}
