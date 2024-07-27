use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("main").arg("text.txt");
    cmd.assert()
    .failure()
    .stderr(predicates::str::contains("No such file or directory"));

    Ok(())
}