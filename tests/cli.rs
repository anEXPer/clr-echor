use assert_cmd::Command;
use predicates::prelude::{self, *};

// This type alias holds either a Unit `()`
// or a Box with something in it that implements Error.
// This does not yet totally make sense to me.
// type TestResult = Result<(), Box<dyn std::error::Error>>;

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
fn does_not_print_flags() {
    Command::cargo_bin("echor")
        .unwrap()
        .args(["first", "--flag", "second"])
        .assert()
        .success()
        .stdout(predicates::str::contains("first second"));
}
