use std::fs;

use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn passes_when_all_key_presents() {
    let dir = tempdir().unwrap();

    fs::write(dir.path().join(".env"), "FOO=bar\n\nBAZ=qux\n").unwrap();
    fs::write(dir.path().join(".env.example"), "FOO=\nBAZ=qux\n").unwrap();

    Command::cargo_bin("envck")
        .unwrap()
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout("all required env vars are present\n");
}
