use clap::Parser;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::error::Error;

use crate::cli::{Cli, Commands};

const ERROR_HELP_MESSAGE: &str = "For more information, try '--help'.";

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

fn read_records_from_cvs_file_and_deserialize(path: &str) -> Result<Vec<Word>, Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_path(path)?;
    let mut deserialized_records_from_file = Vec::new();
    for record in csv_reader.deserialize() {
        let deserialized_record: Word = record?;
        deserialized_records_from_file.push(deserialized_record)
    }
    Ok(deserialized_records_from_file)
}

fn render_question_string(word: Word, no_chinese: &bool, pinyin: &bool, english: &bool) -> String {
    let mut question_words = Vec::new();
    if !(*no_chinese) {
        question_words.push(word.chinese);
    }
    if *pinyin {
        question_words.push(word.pinyin);
    }
    if *english {
        question_words.push(word.english);
    }
    question_words.join(" ")
}

fn print_question_string_with_delay(
    words: Vec<Word>,
    no_chinese: &bool,
    pinyin: &bool,
    english: &bool,
    answer: &bool,
    delay: &u64,
) {
    let delay_duration = std::time::Duration::from_secs(*delay);
    for word in words {
        let question_string = render_question_string(word.clone(), no_chinese, pinyin, english);
        println!("{}\n", question_string);
        std::thread::sleep(delay_duration);
        if *answer {
            println!("{}\n", word);
        }
    }
}

fn print_question_string_waiting_input(
    words: Vec<Word>,
    no_chinese: &bool,
    pinyin: &bool,
    english: &bool,
    answer: &bool,
) -> Result<(), Box<dyn Error>> {
    for word in words {
        let question_string = render_question_string(word.clone(), no_chinese, pinyin, english);
        println!("{}", question_string);
        // As a way to wait for user input
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        if *answer {
            println!("{}\n", word);
        }
    }
    Ok(())
}

fn print_wordlist(words: Vec<Word>, numbers: &bool) {
    if *numbers {
        for word in words {
            println!("{} {}", word.word_number, word);
        }
    } else {
        for word in words {
            println!("{}", word);
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut words = read_records_from_cvs_file_and_deserialize("./src/data/wordlist.csv")?;
    match &cli.command {
        Commands::Train {
            levels,
            no_chinese,
            pinyin,
            english,
            answer,
            shuffle,
            delay,
        } => {
            if let Some(level_numbers) = levels {
                words.retain(|word| level_numbers.contains(&word.hsk_level))
            }
            // Perhaps this scenario can be handled using 'clap' features
            if (no_chinese, pinyin, english) == (&true, &false, &false) {
                eprintln!(
                    "error: it is not possible to use the 'no-chinese' option without using \
                    the 'pinyin' or 'english' options or both.\n\n{}",
                    ERROR_HELP_MESSAGE
                );
                std::process::exit(1);
            }
            if *shuffle {
                let mut rng = rand::thread_rng();
                words.shuffle(&mut rng);
            }
            if *delay > 0u64 {
                print_question_string_with_delay(words, no_chinese, pinyin, english, answer, delay);
            } else {
                print_question_string_waiting_input(words, no_chinese, pinyin, english, answer)?;
            }
        }
        Commands::Wordlist { levels, numbers } => {
            if let Some(level_numbers) = levels {
                words.retain(|word| level_numbers.contains(&word.hsk_level))
            }
            print_wordlist(words, numbers);
        }
    }
    Ok(())
}
