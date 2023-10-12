use assert_cmd::Command;

fn create_hskx_list_command() -> Command {
    let mut hskx_list_command = Command::cargo_bin("hskx").unwrap();
    hskx_list_command.arg("list");
    hskx_list_command
}

fn create_hskx_train_command() -> Command {
    let mut hskx_train_command = Command::cargo_bin("hskx").unwrap();
    hskx_train_command.arg("train");
    hskx_train_command
}

// Simulated successful command:
// `hskx list`
#[test]
fn test_hskx_list() {
    create_hskx_list_command().assert().success();
}

// Simulated successful command:
// `hskx list --level 2`
#[test]
fn test_hskx_list_with_level_option() {
    create_hskx_list_command()
        .arg("--level=2")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx list --numbers`
#[test]
fn test_hskx_list_with_numbers_option() {
    create_hskx_list_command()
        .arg("--numbers")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx list --level 3 --numbers`
#[test]
fn test_hskx_list_with_level_and_numbers_options() {
    create_hskx_list_command()
        .arg("--level=3")
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
// `hskx train --translations`
#[test]
fn test_hskx_train_with_translations_option() {
    create_hskx_train_command()
        .arg("--translations")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --pinyin --translations`
#[test]
fn test_hskx_train_with_pinyin_and_translations_options() {
    create_hskx_train_command()
        .arg("--pinyin")
        .arg("--translations")
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
// `hskx train --no-chinese --translations`
#[test]
fn test_hskx_train_with_no_chinese_and_translations_options() {
    create_hskx_train_command()
        .arg("--no-chinese")
        .arg("--translations")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --no-chinese --pinyin --translations`
#[test]
fn test_hskx_train_with_no_chinese_and_pinyin_and_translations_options() {
    create_hskx_train_command()
        .arg("--no-chinese")
        .arg("--pinyin")
        .arg("--translations")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --level 4`
#[test]
fn test_hskx_train_with_level_option() {
    create_hskx_train_command()
        .arg("--level=4")
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
// `hskx train --no-chinese --pinyin --translations --level 5 --answer --shuffle`
#[test]
fn test_hskx_train_with_all_options_except_delay_option() {
    create_hskx_train_command()
        .arg("--no-chinese")
        .arg("--pinyin")
        .arg("--translations")
        .arg("--level=5")
        .arg("--answer")
        .arg("--shuffle")
        .assert()
        .success();
}

// Simulated successful command:
// `hskx train --no-chinese --pinyin --translations --level 5 --answer --shuffle --delay 1`
#[test]
#[ignore = "Takes a long time to complete"]
fn test_hskx_train_with_all_options() {
    create_hskx_train_command()
        .arg("--no-chinese")
        .arg("--pinyin")
        .arg("--translations")
        .arg("--level=5")
        .arg("--answer")
        .arg("--shuffle")
        .arg("--delay=1")
        .assert()
        .success();
}
