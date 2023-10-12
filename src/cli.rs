use clap::{Parser, Subcommand};

/// Console application to prepare for the Chinese Proficiency Test, known as The Hanyu Shuiping Kaoshi (HSK)
#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print words for practice
    Train {
        /// Exam level
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..7))]
        level: u8,
        /// Chinese characters are not displayed
        #[arg(short, long)]
        no_chinese: bool,
        /// Hanyu pinyin are displayed
        #[arg(short, long)]
        pinyin: bool,
        /// English translations of words are displayed
        #[arg(short, long)]
        translations: bool,
        /// Show the word and it's translation after the question
        #[arg(short, long)]
        answer: bool,
        /// Words appear out of order
        #[arg(short, long)]
        shuffle: bool,
        /// Delay between words
        #[arg(short, long, value_parser = clap::value_parser!(u64).range(1..))]
        delay: Option<u64>,
    },
    /// Print a list of words for exam preparation
    Wordlist {
        /// Exam level
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..7))]
        level: u8,
        /// Display the numbers of the words they have in the wordlist
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
