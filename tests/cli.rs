use assert_cmd::Command;
use predicates::prelude::*;

// This type alias holds either a Unit `()`
// or a Box with something in it that implements Error.
// This does not yet totally make sense to me.
// type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn when_no_args_it_prints_usage_dies() {
    Command::cargo_bin("echor")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));
}

#[test]
fn dash_h_gives_help() {
    Command::cargo_bin("echor")
        .unwrap()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "A Rust version of `echo` written as practice",
        ));
}

#[test]
fn echos_one_arg() {
    Command::cargo_bin("echor")
        .unwrap()
        .args(["onearg"])
        .assert()
        .success()
        .stdout("onearg\n");
}

#[test]
fn echos_three_args() {
    Command::cargo_bin("echor")
        .unwrap()
        .args(["first arg", "second!", "3"])
        .assert()
        .success()
        .stdout("first arg second! 3\n");
}

#[test]
fn dash_n_does_not_print_newline() {
    Command::cargo_bin("echor")
        .unwrap()
        .args(["first", "-n", "second"])
        .assert()
        .success()
        .stdout(predicates::str::contains("first second"))
        .stdout(predicates::str::contains("second\n").not());
}
