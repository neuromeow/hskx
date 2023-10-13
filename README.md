# hskx

`hskx` is a command line tool to prepare for the Chinese Proficiency Test, known as [The Hanyu Shuiping Kaoshi (HSK)](https://en.wikipedia.org/wiki/Hanyu_Shuiping_Kaoshi).
It helps repeat the words presented in the vocabulary lists for each test level.

## Installation

### Manual installation from GitHub

Compiled binary versions of `hskx` are uploaded to GitHub when a release is made.
You can install `hskx` manually by [downloading a release](https://github.com/neuromeow/hskx/releases), extracting it, and copying the binary to a directory in your `$PATH`, such as `/usr/local/bin`.

### Cargo

If you already have a Rust environment set up, you can use the `cargo install` command:

```
cargo install hskx
```

Cargo will build the `hskx` binary and place it in `$HOME/.cargo`.

## Usage

```
Command line tool to prepare for the Hanyu Shuiping Kaoshi (HSK)

Usage: hskx <COMMAND>

Commands:
  train  Start training to repeat words
  list   Print words presented in the vocabulary list
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
Start training to repeat words

Usage: hskx train [OPTIONS]

Options:
  -l, --level <LEVEL>  Exam level [default: 1]
  -n, --no-chinese     Chinese characters are not displayed
  -p, --pinyin         Hanyu pinyin transcriptions are displayed
  -e, --english        English translations are displayed
  -a, --answer         Show the word, it's transcription and translation after the question
  -s, --shuffle        Words appear out of order
  -d, --delay <DELAY>  Delay between words
  -h, --help           Print help

```

```
Print words presented in the vocabulary list

Usage: hskx list [OPTIONS]

Options:
  -l, --level <LEVEL>  Exam level [default: 1]
  -n, --numbers        Display the numbers of the words they have in the vocabulary list
  -h, --help           Print help
```

```
Print this message or the help of the given subcommand(s)

Usage: hskx help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)
```

## Limitations

The HSK is an official, standard Chinese proficiency test used in international Chinese language education.
Starting July 2021, the New HSK standard will replace the previous six-level model.
The New HSK 3.0 has a more specific classification system, including both levels and bands.
Comparing the difficulty and quantity of words required for all four-dimension benchmarks (syllables, characters, vocabulary, and grammar), the New HSK is more rigorous than the old one.

`hskx` covers vocabulary lists for HSK 2.0. Updates for the new version of the exam will be added in future releases.

## License

This project is released under the MIT License.
See [LICENSE](https://github.com/neuromeow/hskx/blob/master/LICENSE) for the full licensing condition.
