# hskx

`hskx` is a command-line tool to prepare for the Chinese Proficiency Test, known as [The Hanyu Shuiping Kaoshi (HSK)](https://en.wikipedia.org/wiki/Hanyu_Shuiping_Kaoshi).
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

`hskx` provides interactive training sessions and vocabulary lists for different HSK levels.

### Listing Vocabulary

To view the vocabulary list for a specific HSK level, use the `list` command. 
This is helpful for reviewing the words before starting your practice session:

```
hskx list [OPTIONS]
```

**Options:**

- `-l, --level <LEVEL>`: Select the HSK level whose vocabulary list you want to display. Default is 1
- `-n, --numbers`: Show word numbers in the vocabulary list
- `-h, --help`: Displays help information for the `list` command

**Example:**

```
hskx list --level 2 --numbers
```

This command displays the level 2 vocabulary list with word numbers.

### Training Session

Use the `train` command to start a practice session with words from the HSK vocabulary list. 
You can customize the session using various options:

```
hskx train [OPTIONS]
```

**Options:**

- `-l, --level <LEVEL>`: Specify the HSK level for training (1 to 6). Default is 1
- `-n, --no-chinese`: Hide Chinese characters during the session
- `-p, --pinyin`: Display Hanyu Pinyin transcriptions
- `-e, --english`: Show English translations of the words
- `-a, --answer`: Reveal the word, its transcription, and translation after each question
- `-s, --shuffle`: Randomize the order of the words
- `-d, --delay <DELAY>`: Set a delay (in seconds) between words to control pacing
- `-h, --help`: Displays help information for the `train` command

**Example:**

```
hskx train --level 3 --pinyin --english --shuffle --delay 5
```

This command starts a level 3 training session showing both Pinyin and English translations, shuffling the words, with a 5-second delay between them.

### Getting Help

To learn more about `hskx` and its commands, use the `help` command. 
It provides detailed information about how to use the tool and its features:

```
hskx help [COMMAND]
```

Replace `[COMMAND]` with the name of the command you need help with.

**Example:**

```
hskx help train
```

This command displays help information specifically for the `train` command.

## Limitations

The HSK is an official, standard Chinese proficiency test used in international Chinese language education.
Starting July 2021, the New HSK standard will replace the previous six-level model.
The New HSK 3.0 has a more specific classification system, including both levels and bands.
Comparing the difficulty and quantity of words required for all four-dimension benchmarks (syllables, characters, vocabulary, and grammar), the New HSK is more rigorous than the old one.

`hskx` covers vocabulary lists for HSK 2.0. Updates for the new version of the exam will be added in future releases.

## License

This project is released under the MIT License.
See [LICENSE](https://github.com/neuromeow/hskx/blob/master/LICENSE) for the full licensing condition.
