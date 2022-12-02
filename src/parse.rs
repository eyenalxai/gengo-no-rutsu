use crate::word::types::Word;
use std::fs::File;
use std::io::BufReader;

pub fn get_words_from_json(path: &str) -> Vec<Word> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("Error opening file words.json: {}", e),
    };

    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(words) => words,
        Err(e) => panic!("Error reading file words.json: {}", e),
    }
}
