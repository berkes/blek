use assert_cmd::prelude::*; // Adds methods on commands
use chrono::Local; // Allows to fetch local dates to match against
use predicates::prelude::*; // Allows easier asssertions
use std::io::Write; // Writes into files
use std::process::Command; // Runs programs
use tempfile::NamedTempFile; // Creates tempfiles

#[test]
fn missing_template_name_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("blek")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No template given"));

    Ok(())
}

#[test]
fn file_doesnt_exist_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("blek")?;
    cmd.arg("notatemplate");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn with_predefined_variables() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    // We need to escape the {{, with two more {{.
    writeln!(file, "Today is {{{{ date }}}}\n Now is {{{{ time }}}}")?;

    let mut cmd = Command::cargo_bin("blek")?;
    cmd.arg(file.path());

    let today = Local::now().format("%X");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("Now is {}", today)));

    Ok(())
}
