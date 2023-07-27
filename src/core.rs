use std::error::Error;

use csv;
use serde::Deserialize;

#[derive(Deserialize)]
struct Word {
    word_number: u32,
    chinese: String,
    pinyin: String,
    english: String,
    hsk_level: u8,
}

fn read_from_file_and_deserialize(path: &str) -> Result<Vec<Word>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut result = Vec::new();
    for record in reader.deserialize() {
        let word_record: Word = record?;
        result.push(word_record)
    }
    Ok(result)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let words = read_from_file_and_deserialize("./src/data/wordlist.csv")?;
    for word in words {
        println!(
            "{} {} {} {} {}",
            word.word_number, word.chinese, word.pinyin, word.english, word.hsk_level
        )
    }
    Ok(())
}
