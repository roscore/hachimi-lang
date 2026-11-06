use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_FILE: AtomicUsize = AtomicUsize::new(0);

fn command(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_hachimi"))
        .args(args)
        .output()
        .expect("launch hachimi")
}

fn temp_source(bytes: &[u8]) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "hachimi-test-{}-{}.hcm",
        std::process::id(),
        NEXT_FILE.fetch_add(1, Ordering::Relaxed)
    ));
    fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn eval_prints_byte_output_and_accepts_utf8_input() {
    let output = command(&["eval", "허니 테이오", "--input", "한"]);
    assert!(output.status.success());
    assert_eq!(output.stdout, [0xed]);
}

#[test]
fn run_reads_utf8_source_file() {
    let path = temp_source("🙂 주석\n허니 테이오".as_bytes());
    let output = command(&["run", path.to_str().unwrap(), "--input", "A"]);
    fs::remove_file(path).unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"A");
}

#[test]
fn check_validates_without_running() {
    let path = temp_source("나루".as_bytes());
    let output = command(&["check", path.to_str().unwrap()]);
    fs::remove_file(path).unwrap();
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn parse_error_has_distinct_exit_code_and_position() {
    let path = temp_source("한글\n 맘보".as_bytes());
    let output = command(&["check", path.to_str().unwrap()]);
    fs::remove_file(path).unwrap();
    assert_eq!(output.status.code(), Some(4));
    assert!(String::from_utf8_lossy(&output.stderr).contains("2:2"));
}

#[test]
fn runtime_error_has_distinct_exit_code_and_position() {
    let output = command(&["eval", "나루"]);
    assert_eq!(output.status.code(), Some(5));
    assert!(String::from_utf8_lossy(&output.stderr).contains("1:1"));
}

#[test]
fn limit_options_are_enforced() {
    let steps = command(&["eval", "하찌미 하찌미", "--max-steps", "1"]);
    assert_eq!(steps.status.code(), Some(5));
    let cells = command(&["eval", "하야쿠", "--max-cells", "1"]);
    assert_eq!(cells.status.code(), Some(5));
}

#[test]
fn invalid_arguments_and_utf8_have_useful_exit_codes() {
    assert_eq!(command(&["eval"]).status.code(), Some(2));
    assert_eq!(
        command(&["eval", "", "--max-steps", "nope"]).status.code(),
        Some(2)
    );
    let path = temp_source(&[0xff]);
    let output = command(&["run", path.to_str().unwrap()]);
    fs::remove_file(path).unwrap();
    assert_eq!(output.status.code(), Some(3));
}

#[test]
fn help_and_version_are_available() {
    let help = command(&["--help"]);
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("하찌미랭"));
    let version = command(&["--version"]);
    assert!(version.status.success());
    assert_eq!(version.stdout, b"hachimi 0.1.0\n");
}
