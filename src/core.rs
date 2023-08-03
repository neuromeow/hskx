use std::error::Error;
use std::{io, thread, time};

use clap::Parser;
use csv;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;

use crate::cli::{Cli, Commands};

#[derive(Clone, Deserialize)]
struct Word {
    word_number: u32,
    chinese: String,
    pinyin: String,
    english: String,
    hsk_level: u8,
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.chinese, self.pinyin, self.english)
    }
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

fn render_question_string(
    word: Word,
    no_hieroglyph: &bool,
    english: &bool,
    pinyin: &bool,
) -> String {
    let mut question_words = Vec::new();
    if *no_hieroglyph == false {
        question_words.push(word.chinese);
    }
    if *pinyin == true {
        question_words.push(word.pinyin);
    }
    if *english == true {
        question_words.push(word.english);
    }
    let question_string = question_words.join(" ");
    question_string
}

fn print_question_string_with_delay(
    words: Vec<Word>,
    delay: u64,
    no_hieroglyph: &bool,
    pinyin: &bool,
    english: &bool,
    answer: &bool,
) {
    let delay_duration = time::Duration::from_secs(delay);
    for word in words {
        let question_string = render_question_string(word.clone(), no_hieroglyph, pinyin, english);
        println!("{}\n", question_string);
        thread::sleep(delay_duration);
        if *answer == true {
            println!("{}\n", word);
        }
    }
}

fn print_question_string_waiting_input(
    words: Vec<Word>,
    no_hieroglyph: &bool,
    pinyin: &bool,
    english: &bool,
    answer: &bool,
) -> Result<(), Box<dyn Error>> {
    for word in words {
        let question_string = render_question_string(word.clone(), no_hieroglyph, pinyin, english);
        println!("{}", question_string);
        // As a way to wait for user input
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if *answer == true {
            println!("{}\n", word);
        }
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut words = read_from_file_and_deserialize("./src/data/wordlist.csv")?;
    if let Some(levels) = cli.levels {
        words.retain(|word| levels.contains(&word.hsk_level))
    }
    match &cli.command {
        Commands::Train {
            no_hieroglyph,
            pinyin,
            english,
            answer,
            shuffle,
            delay,
        } => {
            if *shuffle == true {
                let mut rng = thread_rng();
                words.shuffle(&mut rng);
            }
            if let Some(delay) = delay {
                print_question_string_with_delay(
                    words,
                    *delay,
                    no_hieroglyph,
                    pinyin,
                    english,
                    answer,
                );
            } else {
                print_question_string_waiting_input(words, no_hieroglyph, pinyin, english, answer)?;
            }
        }
        Commands::Wordlist { numbers } => {
            if *numbers == true {
                for word in words {
                    println!("{} {}", word.word_number, word);
                }
            } else {
                for word in words {
                    println!("{}", word);
                }
            }
        }
    }
    Ok(())
}
