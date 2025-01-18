use assert_cmd::{cargo::CommandCargoExt, Command};

#[test]
fn works() {}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello_world").unwrap();
    cmd.assert().success().stdout("Hello World!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
