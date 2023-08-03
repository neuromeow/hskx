use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[arg(short, long, global = true, value_delimiter = ',', use_value_delimiter = true, value_parser = clap::value_parser!(u8).range(1..7))]
    pub levels: Option<Vec<u8>>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Train {
        #[arg(short, long)]
        no_hieroglyph: bool,
        #[arg(short, long)]
        pinyin: bool,
        #[arg(short, long)]
        english: bool,
        #[arg(short, long)]
        answer: bool,
        #[arg(short, long)]
        shuffle: bool,
        #[arg(short, long)]
        delay: Option<u64>,
    },
    Wordlist {
        #[arg(short, long)]
        numbers: bool,
    },
}
