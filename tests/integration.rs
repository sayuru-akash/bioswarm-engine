use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn status_command_runs() {
    let temp = tempdir().unwrap();
    let db_path = temp.path().join("test.db");
    Command::cargo_bin("bioswarm")
        .unwrap()
        .env("FIREWORKS_API_KEY", "test-fireworks")
        .env("DATABASE_PATH", &db_path)
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("checkpoint"));
}

#[test]
fn run_command_writes_exports() {
    let temp = tempdir().unwrap();
    let out = temp.path().join("out");
    let db_path = temp.path().join("test.db");
    Command::cargo_bin("bioswarm")
        .unwrap()
        .env("FIREWORKS_API_KEY", "test-fireworks")
        .arg("run")
        .arg("--query")
        .arg("fintech intelligence")
        .arg("--output-dir")
        .arg(&out)
        .arg("--database-path")
        .arg(&db_path)
        .assert()
        .success();

    let files = fs::read_dir(&out).unwrap().count();
    assert!(files >= 4);
}
