use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("rechor").unwrap();
    cmd.assert()
      .failure()
      .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rechor").unwrap();
    cmd.arg("hello").assert().success();
}

#[test]
fn hello1() {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("rechor").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}

// test with -f flag and check if there is a frog emoji
#[test]
fn frog() {
    let mut cmd = Command::cargo_bin("rechor").unwrap();
    cmd.arg("Hello there").arg("-f").assert().success().stdout(predicate::str::contains("🐸"));
}
