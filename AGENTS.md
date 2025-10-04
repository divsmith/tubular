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

### Recent Refactoring

The following test files have been refactored to comply with this pytest standardization:
- [`tests/test_engine.py`](tests/test_engine.py)
- [`tests/test_step3_verification.py`](tests/test_step3_verification.py)

These files now use pytest fixtures, assertions, and conventions for improved maintainability and consistency with the project's testing standards.