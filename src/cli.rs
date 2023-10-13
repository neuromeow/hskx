use clap::{Parser, Subcommand};

/// Command line tool to prepare for the Hanyu Shuiping Kaoshi (HSK)
#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start training to repeat words
    Train {
        /// Exam level
        #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..7))]
        level: u8,
        /// Chinese characters are not displayed
        #[arg(short, long)]
        no_chinese: bool,
        /// Hanyu pinyin transcriptions are displayed
        #[arg(short, long)]
        pinyin: bool,
        /// English translations are displayed
        #[arg(short, long)]
        english: bool,
        /// Show the word, it's transcription and translation after the question
        #[arg(short, long)]
        answer: bool,
        /// Words appear out of order
        #[arg(short, long)]
        shuffle: bool,
        /// Delay between words
        #[arg(short, long, value_parser = clap::value_parser!(u64).range(1..))]
        delay: Option<u64>,
    },
    /// Print words presented in the vocabulary list
    List {
        /// Exam level
        #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..7))]
        level: u8,
        /// Display the numbers of the words they have in the vocabulary list
        #[arg(short, long)]
        numbers: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
