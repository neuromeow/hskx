use assert_cmd::Command;

fn create_hskx_wordlist_command() -> Command {
    let mut hskx_wordlist_command = Command::cargo_bin("hskx").unwrap();
    hskx_wordlist_command.arg("wordlist");
    hskx_wordlist_command
}

fn create_hskx_train_command() -> Command {
    let mut hskx_train_command = Command::cargo_bin("hskx").unwrap();
    hskx_train_command.arg("train");
    hskx_train_command
}

// Simulated successful command:
// `hskx wordlist`
#[test]
fn test_hskx_wordlist() {
    create_hskx_wordlist_command().assert().success();
}

// Simulated successful command:
// `hskx wordlist --levels 1`
#[test]
fn test_hskx_wordlist_with_levels_option_for_one_level() {
    create_hskx_wordlist_command()
        .arg("--levels=1")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx wordlist --levels 1,2,5`
#[test]
fn test_hskx_wordlist_with_levels_option_for_several_levels() {
    create_hskx_wordlist_command()
        .arg("--levels=1,2,5")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx wordlist --numbers`
#[test]
fn test_hskx_wordlist_with_numbers_option() {
    create_hskx_wordlist_command()
        .arg("--numbers")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx wordlist --levels 3 --numbers`
#[test]
fn test_hskx_wordlist_with_levels_for_one_level_and_numbers_options() {
    create_hskx_wordlist_command()
        .arg("--levels=3")
        .arg("--numbers")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx wordlist --levels 4,6 --numbers`
#[test]
fn test_hskx_wordlist_with_levels_for_several_levels_and_numbers_options() {
    create_hskx_wordlist_command()
        .arg("--levels=4,6")
        .arg("--numbers")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train`
#[test]
fn test_hskx_train() {
    create_hskx_train_command().assert().success();
}

// Simulated successful command:
// `hskx train --pinyin`
#[test]
fn test_hskx_train_with_pinyin_option() {
    create_hskx_train_command()
        .arg("--pinyin")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --english`
#[test]
fn test_hskx_train_with_english_option() {
    create_hskx_train_command()
        .arg("--english")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --pinyin --english`
#[test]
fn test_hskx_train_with_pinyin_and_english_options() {
    create_hskx_train_command()
        .arg("--pinyin")
        .arg("--english")
        .assert()
        .success();
}

// Simulated failed command:
// `hskx train --no-chinese`
#[test]
fn test_hskx_train_with_no_chinese_option() {
    create_hskx_train_command()
        .arg("--no-chinese")
        .assert()
        .failure();
}

// Simulated successful command:
// `hskx train --no-chinese --pinyin`
#[test]
fn test_hskx_train_with_no_chinese_and_pinyin_options() {
    create_hskx_train_command()
        .arg("--no-chinese")
        .arg("--pinyin")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --no-chinese --english`
#[test]
fn test_hskx_train_with_no_chinese_and_english_options() {
    create_hskx_train_command()
        .arg("--no-chinese")
        .arg("--english")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --no-chinese --pinyin --english`
#[test]
fn test_hskx_train_with_no_chinese_and_pinyin_and_english_options() {
    create_hskx_train_command()
        .arg("--no-chinese")
        .arg("--pinyin")
        .arg("--english")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --levels 1`
#[test]
fn test_hskx_train_with_levels_option_for_one_level() {
    create_hskx_train_command()
        .arg("--levels=1")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --levels 1,2,5`
#[test]
fn test_hskx_train_with_levels_option_for_several_levels() {
    create_hskx_train_command()
        .arg("--levels=1,2,5")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --answer`
#[test]
fn test_hskx_train_with_answer_option() {
    create_hskx_train_command()
        .arg("--answer")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --shuffle`
#[test]
fn test_hskx_train_with_shuffle_option() {
    create_hskx_train_command()
        .arg("--shuffle")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --delay 1`
#[test]
#[ignore = "Takes a long time to complete"]
fn test_hskx_train_with_delay_option() {
    create_hskx_train_command()
        .arg("--delay=1")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --no-chinese --pinyin --english --levels 3,4,6 --answer --shuffle`
#[test]
fn test_hskx_train_with_all_options_except_delay_option() {
    create_hskx_train_command()
        .arg("--no-chinese")
        .arg("--pinyin")
        .arg("--english")
        .arg("--levels=3,4,6")
        .arg("--answer")
        .arg("--shuffle")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --no-chinese --pinyin --english --levels 3,4,6 --answer --shuffle --delay 1`
#[test]
#[ignore = "Takes a long time to complete"]
fn test_hskx_train_with_all_options() {
    create_hskx_train_command()
        .arg("--no-chinese")
        .arg("--pinyin")
        .arg("--english")
        .arg("--levels=3,4,6")
        .arg("--answer")
        .arg("--shuffle")
        .arg("--delay=1")
        .assert()
        .success();
}
