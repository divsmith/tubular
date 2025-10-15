//! Basic functionality integration tests
//! These tests verify that the tubular interpreter works end-to-end

use std::process::Command;
use std::fs;
use std::path::Path;

/// Test that the tubular binary can execute simple programs
#[test]
fn test_tubular_binary_execution() {
    // Build the project first
    let build_output = Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .expect("Failed to build tubular");

    assert!(build_output.status.success(), "Build failed: {}", String::from_utf8_lossy(&build_output.stderr));

    let tubular_binary = "target/release/tubular";
    assert!(Path::new(tubular_binary).exists(), "Binary not found at {}", tubular_binary);

    // Test simple.tb program
    let simple_program = r#"
@
|
7
2
-
n
!
"#;

    fs::write("test_simple.tb", simple_program).expect("Failed to write test program");

    let output = Command::new(tubular_binary)
        .arg("test_simple.tb")
        .output()
        .expect("Failed to run tubular");

    assert!(output.status.success(), "Tubular execution failed: {}", String::from_utf8_lossy(&output.stderr));

    let result = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    assert_eq!(result.trim(), "5", "Expected output '5', got '{}'", result.trim());

    // Cleanup
    fs::remove_file("test_simple.tb").ok();
}

/// Test that output formatting works correctly (no duplicates)
#[test]
fn test_no_duplicate_output() {
    let tubular_binary = "target/release/tubular";

    // Create a simple program that outputs a number
    let program = r#"
@
|
5
n
!
"#;

    fs::write("test_output.tb", program).expect("Failed to write test program");

    let output = Command::new(tubular_binary)
        .arg("test_output.tb")
        .output()
        .expect("Failed to run tubular");

    assert!(output.status.success(), "Tubular execution failed: {}", String::from_utf8_lossy(&output.stderr));

    let result = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");

    // Should only have one "5", not "55" (no duplicate output)
    assert_eq!(result.trim(), "5", "Expected single output '5', got '{}'", result.trim());
    assert_eq!(result.trim().len(), 1, "Output should be exactly 1 character, got {} characters", result.trim().len());

    // Cleanup
    fs::remove_file("test_output.tb").ok();
}

/// Test hello world program
#[test]
fn test_hello_world() {
    let tubular_binary = "target/release/tubular";

    // Check if hello_world.tb exists
    let hello_world_path = "examples/hello_world.tb";
    if !Path::new(hello_world_path).exists() {
        // Skip test if file doesn't exist
        return;
    }

    let output = Command::new(tubular_binary)
        .arg(hello_world_path)
        .output()
        .expect("Failed to run tubular");

    assert!(output.status.success(), "Hello world execution failed: {}", String::from_utf8_lossy(&output.stderr));

    let result = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");

    // Should have some output (not empty)
    assert!(!result.trim().is_empty(), "Hello world should produce output");

    // Should not have duplicate output
    let lines: Vec<&str> = result.trim().lines().collect();
    assert_eq!(lines.len(), 1, "Hello world should produce exactly one line of output");
}

/// Test countdown program
#[test]
fn test_countdown() {
    let tubular_binary = "target/release/tubular";

    // Check if countdown.tb exists
    let countdown_path = "examples/countdown.tb";
    if !Path::new(countdown_path).exists() {
        // Skip test if file doesn't exist
        return;
    }

    let output = Command::new(tubular_binary)
        .arg(countdown_path)
        .output()
        .expect("Failed to run tubular");

    assert!(output.status.success(), "Countdown execution failed: {}", String::from_utf8_lossy(&output.stderr));

    let result = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");

    // Should have some output (not empty)
    assert!(!result.trim().is_empty(), "Countdown should produce output");

    // Should not have duplicate output pattern (like "5151")
    let trimmed = result.trim();
    assert!(!trimmed.chars().all(|c| trimmed.matches(c).count() > 1), "Countdown should not have duplicate characters");
}

/// Test arithmetic operations
#[test]
fn test_arithmetic_operations() {
    let tubular_binary = "target/release/tubular";

    // Test addition program
    let add_program = r#"
@
|
5
:
3
S
+
n
!
"#;

    fs::write("test_add.tb", add_program).expect("Failed to write test program");

    let output = Command::new(tubular_binary)
        .arg("test_add.tb")
        .output()
        .expect("Failed to run tubular");

    assert!(output.status.success(), "Addition execution failed: {}", String::from_utf8_lossy(&output.stderr));

    let result = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    assert_eq!(result.trim(), "8", "Expected addition result '8', got '{}'", result.trim());

    // Cleanup
    fs::remove_file("test_add.tb").ok();
}

/// Test that error handling works (invalid programs should fail gracefully)
#[test]
fn test_error_handling() {
    let tubular_binary = "target/release/tubular";

    // Create an invalid program (no start symbol)
    let invalid_program = r#"
5
2
-
n
!
"#;

    fs::write("test_invalid.tb", invalid_program).expect("Failed to write test program");

    let output = Command::new(tubular_binary)
        .arg("test_invalid.tb")
        .output()
        .expect("Failed to run tubular");

    // Should fail (non-zero exit code)
    assert!(!output.status.success(), "Invalid program should fail");

    // Should have error message
    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8 stderr");
    assert!(stderr.contains("error") || stderr.contains("Error") || stderr.contains("ERROR"),
            "Should have error message, got: {}", stderr);

    // Cleanup
    fs::remove_file("test_invalid.tb").ok();
}

/// Test that help command works
#[test]
fn test_help_command() {
    let tubular_binary = "target/release/tubular";

    let output = Command::new(tubular_binary)
        .arg("--help")
        .output()
        .expect("Failed to run tubular --help");

    assert!(output.status.success(), "Help command failed: {}", String::from_utf8_lossy(&output.stderr));

    let result = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");

    // Should contain help information
    assert!(result.contains("tubular") || result.contains("Tubular"), "Help should contain program name");
    assert!(result.contains("help") || result.contains("--help"), "Help should mention help option");
}

/// Test version command
#[test]
fn test_version_command() {
    let tubular_binary = "target/release/tubular";

    let output = Command::new(tubular_binary)
        .arg("--version")
        .output()
        .expect("Failed to run tubular --version");

    assert!(output.status.success(), "Version command failed: {}", String::from_utf8_lossy(&output.stderr));

    let result = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");

    // Should contain version information
    assert!(result.contains("0.1.0") || result.contains("version") || result.contains("Version"),
            "Version output should contain version info, got: {}", result);
}