use assert_cmd::Command;
use pretty_assertions::assert_eq;
#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("ch1_hello").unwrap();
    let output = cmd.output().expect("failed");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8. How?");
    assert_eq!(stdout, "Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}