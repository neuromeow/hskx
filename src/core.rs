use std::error::Error;
use std::{io, thread, time};

use clap::Parser;
use csv;
use serde::Deserialize;

use crate::cli::{Cli, Commands};

#[derive(Deserialize)]
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
            delay,
        } => {
            if let Some(delay) = delay {
                let delay_duration = time::Duration::from_secs(*delay);
                for word in words {
                    let mut printing_string = String::from(&word.chinese);
                    printing_string.push(' ');
                    if *pinyin == true {
                        printing_string.push_str(&word.pinyin);
                    }
                    if *english == true {
                        printing_string.push(' ');
                        printing_string.push_str(&word.english);
                    }
                    println!("{}\n", printing_string);
                    thread::sleep(delay_duration);
                }
            } else {
                for word in words {
                    let mut printing_string = String::from(&word.chinese);
                    printing_string.push(' ');
                    if *pinyin == true {
                        printing_string.push_str(&word.pinyin);
                    }
                    if *english == true {
                        printing_string.push(' ');
                        printing_string.push_str(&word.english);
                    }
                    println!("{}", printing_string);
                    // As a way to wait for user input
                    let mut buffer = String::new();
                    io::stdin().read_line(&mut buffer)?;
                }
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
