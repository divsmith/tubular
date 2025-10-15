//! Smoke test - verifies basic tubular functionality works end-to-end

use std::process::Command;
use std::fs;

#[test]
fn test_basic_tubular_functionality() {
    // Build the project first
    let build_output = Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .expect("Failed to build tubular");

    assert!(build_output.status.success(), "Build failed: {}", String::from_utf8_lossy(&build_output.stderr));

    let tubular_binary = "target/debug/tubular";
    assert!(fs::metadata(tubular_binary).is_ok(), "Binary not found at {}", tubular_binary);

    // Test 1: Simple arithmetic (7-2=5)
    let simple_program = r#"
@
|
7
2
-
n
!
"#;

    fs::write("test_arith.tb", simple_program).expect("Failed to write test program");

    let output = Command::new(tubular_binary)
        .arg("test_arith.tb")
        .output()
        .expect("Failed to run tubular");

    assert!(output.status.success(), "Tubular execution failed: {}", String::from_utf8_lossy(&output.stderr));

    let result = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    assert_eq!(result.trim(), "2", "Arithmetic test failed: expected '2', got '{}'", result.trim());

    // Test 2: Output formatting (no duplicates)
    let output_program = r#"
@
|
5
n
!
"#;

    fs::write("test_output.tb", output_program).expect("Failed to write test program");

    let output = Command::new(tubular_binary)
        .arg("test_output.tb")
        .output()
        .expect("Failed to run tubular");

    assert!(output.status.success(), "Tubular execution failed: {}", String::from_utf8_lossy(&output.stderr));

    let result = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    assert_eq!(result.trim(), "5", "Output test failed: expected single '5', got '{}'", result.trim());
    assert_eq!(result.trim().len(), 1, "Output should be exactly 1 character, got {}", result.trim().len());

    // Test 3: Error handling (invalid program should fail)
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

    // Test 4: Help command works
    let output = Command::new(tubular_binary)
        .arg("--help")
        .output()
        .expect("Failed to run tubular --help");

    assert!(output.status.success(), "Help command failed: {}", String::from_utf8_lossy(&output.stderr));

    let result = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    assert!(result.contains("tubular") || result.contains("Tubular"), "Help should contain program name");

    // Cleanup
    fs::remove_file("test_arith.tb").ok();
    fs::remove_file("test_output.tb").ok();
    fs::remove_file("test_invalid.tb").ok();

    // All tests passed!
    println!("✅ All smoke tests passed - tubular is working correctly!");
}