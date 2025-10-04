# Testing Standards and Guidelines

## Pytest Standardization Rule

**All tests in this project must use pytest as the testing framework.**

### Reasoning

This standardization ensures:
- **Consistency**: Uniform testing patterns across all test files
- **Better tooling support**: Enhanced IDE integration and debugging capabilities
- **Easier maintenance**: Simplified test discovery and execution
- **Improved developer experience**: Rich assertion syntax and comprehensive test reporting

### Policy on Testing Frameworks

Custom testing frameworks are no longer allowed in this project. All new and existing tests must be converted to use pytest.

## Test Location
- All test files must be located in the test/ directory.
- Regression tests must be run after every change. A task is not complete until regression tests pass successfully.

## Workspace Cleanup Requirements

**All coding agents must clean up temporary and debug files created during their work.**

### Cleanup Directive

Coding agents must:
- **Clean up temporary files**: Remove any temporary files, cache files, or intermediate artifacts created during development or debugging
- **Clean up debug files**: Remove debug logs, temporary output files, and debugging artifacts that are no longer needed
- **Ensure clean workspace**: Verify that no unnecessary files are left in the workspace after task completion
- **Remove test artifacts**: Clean up any test files, log files, or temporary outputs that were created for debugging or testing purposes

### Enforcement

A task is not considered complete until the workspace has been cleaned of all temporary and debug files created during the work. This ensures a clean, maintainable codebase without accumulation of unnecessary artifacts.