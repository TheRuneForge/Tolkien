use std::process::Command;

#[test]
fn executable_reports_application_identity() {
    let output = Command::new(env!("CARGO_BIN_EXE_tolkien"))
        .output()
        .expect("Tolkien executable should start");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)
        .expect("Tolkien output should be valid UTF-8");

    assert!(stdout.starts_with("Tolkien v"));
    assert!(stdout.contains("Adaptive, modular document interpretation"));
}
