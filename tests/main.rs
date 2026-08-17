use std::fs;

use assert_cmd::{Command, assert::Assert};
use predicates::str::contains;
use tempfile::tempdir;

fn run(dotenv: Option<&str>, example: Option<&str>) -> Assert {
    let dir = tempdir().unwrap();

    if let Some(content) = dotenv {
        fs::write(dir.path().join(".env"), content).unwrap();
    }
    if let Some(content) = example {
        fs::write(dir.path().join(".env.example"), content).unwrap();
    }

    Command::cargo_bin("envck")
        .unwrap()
        .current_dir(dir.path())
        .assert()
}

#[test]
fn passes_when_all_keys_present() {
    run(Some("FOO=bar\n\nBAZ=qux\n"), Some("FOO=\nBAZ=qux\n")).success();
}

#[test]
fn fails_when_dotenv_has_extra_keys() {
    run(Some("FOO=bar\nBAZ=qux\nEXTRA=1\n"), Some("FOO=\nBAZ=\n")).failure();
}

#[test]
fn fails_when_dotenv_has_key_not_in_example() {
    run(Some("FOO=bar\nEXTRA=1\n"), Some("FOO=\n")).failure();
}

#[test]
fn passes_when_required_value_is_empty() {
    run(Some("FOO=\n"), Some("FOO=\n")).success();
}

#[test]
fn passes_when_example_file_is_empty() {
    run(Some("FOO=bar\n"), Some("")).success();
}

#[test]
fn passes_when_value_contains_equals_sign() {
    run(Some("FOO=bar=baz\n"), Some("FOO=\n")).success();
}

#[test]
fn fails_when_required_key_missing_from_dotenv() {
    run(Some("BAZ=qux\n"), Some("FOO=\nBAZ=\n")).failure();
}

#[test]
fn fails_when_dotenv_is_empty_but_example_requires_keys() {
    run(Some(""), Some("FOO=\n")).failure();
}

#[test]
fn fails_when_dotenv_file_is_missing() {
    run(None, Some("FOO=\n")).failure();
}

#[test]
fn fails_when_example_file_is_missing() {
    run(Some("FOO=bar\n"), None).failure();
}
