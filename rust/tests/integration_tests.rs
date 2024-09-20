use std::process::Command;

#[test]
fn two_users() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "beam_planner",
            "output",
            "../test/01_two_users.txt",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Scenario:"));
    assert!(stdout.contains("Solution:"));
}

#[test]
fn five_users() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "beam_planner",
            "output",
            "../test/02_five_users.txt",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Scenario:"));
    assert!(stdout.contains("Solution:"));
}

#[test]
fn equatorial_band_users() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "beam_planner",
            "output",
            "../test/03_equatorial_band.txt",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Scenario:"));
    assert!(stdout.contains("Solution:"));
}

#[test]
fn five_thousand_users() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "beam_planner",
            "output",
            "../test/04_five_thousand.txt",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Scenario:"));
    assert!(stdout.contains("Solution:"));
}

#[test]
fn fifty_thousand_users_low_coverage() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "beam_planner",
            "output",
            "../test/05_fifty_thousand_low_coverage.txt",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Scenario:"));
    assert!(stdout.contains("Solution:"));
}

#[test]
fn ten_thousand_users() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "beam_planner",
            "output",
            "../test/06_ten_thousand.txt",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Scenario:"));
    assert!(stdout.contains("Solution:"));
}

#[test]
fn one_hundred_thousand_users() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "beam_planner",
            "output",
            "../test/11_one_hundred_thousand_users.txt",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Scenario:"));
    assert!(stdout.contains("Solution:"));
}
