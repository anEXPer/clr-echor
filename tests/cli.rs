#![warn(clippy::all, clippy::pedantic)]
use std::{fs, str};

use anyhow;
use assert_cmd::{cargo::CommandCargoExt, Command};
use predicates::prelude::*;

// this set of tests uses naive/unwrap error handling with assertions,
// which is vulnerable to panics.
// Ordindarily, I might refactor these to use one of the later patterns,
// but I'm going to leave them this way because I like the comparison.

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

// Adding tests here using the fixture method from the book
// These tests also use `?` instead of unwrap,
// Passing a result up to the caller of the test functions to handle.

// This version is from the 2024 revision
// It uses anyhow for the returned result.
// We're going to do the rest of the fixture tests from the book this way.
#[test]
fn hello1() -> anyhow::Result<()> {
    run_against_fixture(&["Hello there"], "tests/expected/hello1.txt")
}
#[test]
fn hello2() -> anyhow::Result<()> {
    run_against_fixture(&["Hello", "there"], "tests/expected/hello2.txt")
}
#[test]
fn hello1n() -> anyhow::Result<()> {
    run_against_fixture(&["Hello there", "-n"], "tests/expected/hello1.n.txt")
}
#[test]
fn hello2n() -> anyhow::Result<()> {
    run_against_fixture(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}

fn run_against_fixture(arg_array: &[&str], fixture_path: &str) -> anyhow::Result<()> {
    let expected = fs::read_to_string(fixture_path)?;
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.args(arg_array).assert().success().stdout(expected);
    Ok(())
}

// Let's try the version from the original
// which builds a type alias for itself.

// This type alias holds either a Unit `()`
// or a Box with something in it that implements Error.
// This appears to be functionally equivalent to the anyhow Result type above.
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn hello1o() -> TestResult {
    let expected = fs::read_to_string("tests/expected/hello1.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}
