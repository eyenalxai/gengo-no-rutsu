use crate::utils::word::WordData;
use std::fs::File;
use std::io::BufReader;

pub fn get_words_from_json(path: &str) -> Vec<WordData> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("Error opening file words_data.json: {}", e),
    };

    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(words) => words,
        Err(e) => panic!("Error reading file words_data.json: {}", e),
    }
}
