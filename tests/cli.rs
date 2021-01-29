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
        .stderr(predicate::str::contains("No template file given"));

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

#[test]
fn with_argument_variables() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    // We need to escape the {{, with two more {{.
    writeln!(
        file,
        "Invoice number {{{{ number }}}}\n Total €{{{{ total }}}}"
    )?;

    let mut cmd = Command::cargo_bin("blek")?;
    cmd.arg(file.path());
    cmd.arg("--var");
    cmd.arg("number=42");
    cmd.arg("--var");
    cmd.arg("total=13.37");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Invoice number 42\n Total €13.37"));

    Ok(())
}

#[test]
fn with_argument_variable_that_overrides_basic() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    // We need to escape the {{, with two more {{.
    writeln!(file, "Invoice date {{{{ date }}}}")?;

    let mut cmd = Command::cargo_bin("blek")?;
    cmd.arg(file.path());
    cmd.arg("--var");
    cmd.arg("date=2010");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Invoice date 2010"));

    Ok(())
}
