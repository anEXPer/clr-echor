use assert_cmd::Command;
// use predicates::prelude::*;

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
