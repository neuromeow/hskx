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

#[test]
fn test_hskx_wordlist() {
    create_hskx_wordlist_command().assert().success();
}

#[test]
fn test_hskx_wordlist_with_levels_option_for_one_level() {
    create_hskx_wordlist_command()
        .arg("--levels=1")
        .assert()
        .success();
}

#[test]
fn test_hskx_wordlist_with_levels_option_for_several_levels() {
    create_hskx_wordlist_command()
        .arg("--levels=1,2,5")
        .assert()
        .success();
}

#[test]
fn test_hskx_wordlist_with_numbers_option() {
    create_hskx_wordlist_command()
        .arg("--numbers")
        .assert()
        .success();
}

#[test]
fn test_hskx_wordlist_with_levels_for_one_level_and_numbers_options() {
    create_hskx_wordlist_command()
        .arg("--levels=3")
        .arg("--numbers")
        .assert()
        .success();
}

#[test]
fn test_hskx_wordlist_with_levels_for_several_levels_and_numbers_options() {
    create_hskx_wordlist_command()
        .arg("--levels=4,6")
        .arg("--numbers")
        .assert()
        .success();
}
