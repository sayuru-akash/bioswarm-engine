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

#[test]
fn run_command_supports_ollama_backend_flags() {
    let temp = tempdir().unwrap();
    let out = temp.path().join("out");
    let db_path = temp.path().join("test.db");
    Command::cargo_bin("bioswarm")
        .unwrap()
        .env("OLLAMA_API_KEY", "test-ollama")
        .arg("run")
        .arg("--query")
        .arg("ollama backend check")
        .arg("--backend")
        .arg("ollama")
        .arg("--model")
        .arg("kimi-k2.5:cloud")
        .arg("--api-base-url")
        .arg("http://127.0.0.1:11434/v1")
        .arg("--api-key-env")
        .arg("OLLAMA_API_KEY")
        .arg("--output-dir")
        .arg(&out)
        .arg("--database-path")
        .arg(&db_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("BioSwarm v3.5 report"))
        .stdout(predicate::str::contains("ollama"));
}

#[test]
fn run_command_supports_openai_compatible_backend_flags() {
    let temp = tempdir().unwrap();
    let out = temp.path().join("out-openai");
    let db_path = temp.path().join("test-openai.db");
    Command::cargo_bin("bioswarm")
        .unwrap()
        .env("OPENAI_API_KEY", "test-openai")
        .arg("run")
        .arg("--query")
        .arg("openai compatible backend check")
        .arg("--backend")
        .arg("openai-compatible")
        .arg("--model")
        .arg("gpt-4.1-mini")
        .arg("--api-base-url")
        .arg("https://api.openai.com/v1")
        .arg("--api-key-env")
        .arg("OPENAI_API_KEY")
        .arg("--output-dir")
        .arg(&out)
        .arg("--database-path")
        .arg(&db_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("BioSwarm v3.5 report"))
        .stdout(predicate::str::contains("openai-compatible"));
}
