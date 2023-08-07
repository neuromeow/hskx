use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Train {
        #[arg(short, long, value_delimiter = ',', use_value_delimiter = true, value_parser = clap::value_parser!(u8).range(1..7))]
        levels: Option<Vec<u8>>,
        #[arg(short, long)]
        no_chinese: bool,
        #[arg(short, long)]
        pinyin: bool,
        #[arg(short, long)]
        english: bool,
        #[arg(short, long)]
        answer: bool,
        #[arg(short, long)]
        shuffle: bool,
        #[arg(short, long, default_value_t = 0)]
        delay: u64,
    },
    Wordlist {
        #[arg(short, long, value_delimiter = ',', use_value_delimiter = true, value_parser = clap::value_parser!(u8).range(1..7))]
        levels: Option<Vec<u8>>,
        #[arg(short, long)]
        numbers: bool,
    },
}
