use std::error::Error;

use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Word {
    word_number: u32,
    chinese: String,
    pinyin: String,
    english: String,
    hsk_level: u8,
}

#[derive(Deserialize)]
struct WordRecord {
    word_number: u32,
    chinese: String,
    pinyin: String,
    english: String,
    hsk_level: u8,
}

#[warn(dead_code)]
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?;
    println!("{:?}", headers);
    for record in reader.records() {
        let unwrapped_record = record?;
        println!("{:?}", unwrapped_record);
    }
    Ok(())
}

#[warn(dead_code)]
fn read_from_file_and_deserialize(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?;
    println!("{:?}", headers);
    for record in reader.deserialize() {
        let result: Word = record?;
        println!("{:?}", result);
    }
    Ok(())
}

fn read_from_file_deserialize_and_print(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    for record in reader.deserialize() {
        let word_record: WordRecord = record?;
        println!(
            "{} {} {} {} {}",
            word_record.word_number,
            word_record.chinese,
            word_record.pinyin,
            word_record.english,
            word_record.hsk_level
        )
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // read_from_file("./src/data/wordlist.csv")?;
    // read_from_file_and_deserialize("./src/data/wordlist.csv")?;
    read_from_file_deserialize_and_print("./src/data/wordlist.csv")?;
    Ok(())
}
