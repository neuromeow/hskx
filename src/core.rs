use clap::Parser;
use colored::Colorize;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::error::Error;

use crate::cli::{Cli, Commands};

const HSK_VOCABULARY_LIST_LEVEL_1: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/hsk_vocabulary_lists/hsk_level_1.csv"
));
const HSK_VOCABULARY_LIST_LEVEL_2: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/hsk_vocabulary_lists/hsk_level_2.csv"
));
const HSK_VOCABULARY_LIST_LEVEL_3: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/hsk_vocabulary_lists/hsk_level_3.csv"
));
const HSK_VOCABULARY_LIST_LEVEL_4: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/hsk_vocabulary_lists/hsk_level_4.csv"
));
const HSK_VOCABULARY_LIST_LEVEL_5: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/hsk_vocabulary_lists/hsk_level_5.csv"
));
const HSK_VOCABULARY_LIST_LEVEL_6: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/hsk_vocabulary_lists/hsk_level_6.csv"
));

#[derive(Clone, Deserialize)]
struct Word {
    number: u32,
    chinese: String,
    pinyin: String,
    translations: String,
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.chinese, self.pinyin, self.translations)
    }
}

fn read_records_from_hsk_vocabulary_list_and_deserialize(
    level: &u8,
) -> Result<Vec<Word>, Box<dyn Error>> {
    let hsk_vocabulary_list = match level {
        1 => HSK_VOCABULARY_LIST_LEVEL_1,
        2 => HSK_VOCABULARY_LIST_LEVEL_2,
        3 => HSK_VOCABULARY_LIST_LEVEL_3,
        4 => HSK_VOCABULARY_LIST_LEVEL_4,
        5 => HSK_VOCABULARY_LIST_LEVEL_5,
        _ => HSK_VOCABULARY_LIST_LEVEL_6,
    };
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\\')
        .from_reader(hsk_vocabulary_list);
    let mut deserialized_records_from_hsk_vocabulary_list = Vec::new();
    for record in reader.deserialize() {
        let deserialized_record: Word = record?;
        deserialized_records_from_hsk_vocabulary_list.push(deserialized_record)
    }
    Ok(deserialized_records_from_hsk_vocabulary_list)
}

fn render_question_string(
    word: Word,
    no_chinese: &bool,
    pinyin: &bool,
    translations: &bool,
) -> String {
    let mut question_string = Vec::new();
    if !(*no_chinese) {
        question_string.push(word.chinese);
    }
    if *pinyin {
        question_string.push(word.pinyin);
    }
    if *translations {
        question_string.push(word.translations);
    }
    question_string.join(" ")
}

fn print_question_strings_with_delay(
    words: Vec<Word>,
    no_chinese: &bool,
    pinyin: &bool,
    translations: &bool,
    answer: &bool,
    delay: &u64,
) {
    let delay_duration = std::time::Duration::from_secs(*delay);
    for word in words {
        let question_string =
            render_question_string(word.clone(), no_chinese, pinyin, translations);
        println!("{}\n", question_string);
        std::thread::sleep(delay_duration);
        if *answer {
            println!("{}\n", word);
        }
    }
}

fn print_question_strings_with_waiting_for_input(
    words: Vec<Word>,
    no_chinese: &bool,
    pinyin: &bool,
    translations: &bool,
    answer: &bool,
) -> Result<(), Box<dyn Error>> {
    for word in words {
        let question_string =
            render_question_string(word.clone(), no_chinese, pinyin, translations);
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
            println!("{} {}", word.number, word);
        }
    } else {
        for word in words {
            println!("{}", word);
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Train {
            level,
            no_chinese,
            pinyin,
            translations,
            answer,
            shuffle,
            delay,
        } => {
            // Perhaps this scenario can be handled using 'clap' features
            if (no_chinese, pinyin, translations) == (&true, &false, &false) {
                let options_mismatch_error = format!(
                    "{}: it is not possible to use '{}' without using '{}' or '{}' or both\n\nFor more information, try '{}'.",
                    "error".red(),
                    "--no-chinese".bold(),
                    "--pinyin".bold(),
                    "--translations".bold(),
                    "--help".bold()
                );
                eprintln!("{}", options_mismatch_error);
                std::process::exit(1);
            }
            let mut words = read_records_from_hsk_vocabulary_list_and_deserialize(level)?;
            if *shuffle {
                let mut rng = rand::thread_rng();
                words.shuffle(&mut rng);
            }
            if let Some(delay_value) = delay {
                print_question_strings_with_delay(
                    words,
                    no_chinese,
                    pinyin,
                    translations,
                    answer,
                    delay_value,
                );
            } else {
                print_question_strings_with_waiting_for_input(
                    words,
                    no_chinese,
                    pinyin,
                    translations,
                    answer,
                )?;
            }
        }
        Commands::Wordlist { level, numbers } => {
            let words = read_records_from_hsk_vocabulary_list_and_deserialize(level)?;
            print_wordlist(words, numbers);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_question_string() {
        let test_word = Word {
            number: 1,
            chinese: String::from("考试"),
            pinyin: String::from("kǎoshì"),
            english: String::from("exam"),
            level: 1,
        };
        assert_eq!(
            render_question_string(test_word.clone(), &false, &false, &false),
            String::from("考试")
        );
        assert_eq!(
            render_question_string(test_word.clone(), &false, &true, &false),
            String::from("考试 kǎoshì")
        );
        assert_eq!(
            render_question_string(test_word.clone(), &false, &false, &true),
            String::from("考试 exam")
        );
        assert_eq!(
            render_question_string(test_word.clone(), &false, &true, &true),
            String::from("考试 kǎoshì exam")
        );
        // Real execution with such options will cause the expected error
        assert_eq!(
            render_question_string(test_word.clone(), &true, &false, &false),
            String::from("")
        );
        assert_eq!(
            render_question_string(test_word.clone(), &true, &true, &false),
            String::from("kǎoshì")
        );
        assert_eq!(
            render_question_string(test_word.clone(), &true, &false, &true),
            String::from("exam")
        );
        assert_eq!(
            render_question_string(test_word.clone(), &true, &true, &true),
            String::from("kǎoshì exam")
        );
    }
}
