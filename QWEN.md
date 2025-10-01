# Project Guidelines for Tubular Programming Language

## Directory Structure
- All source code should be placed in the `src/` directory
- All tests should be placed in the `test/` directory
- Documentation files should be placed in the root or in a `docs/` directory if there are many

## Implementation Process
- Follow the steps outlined in `bootstrap_checklist.md`
- Ensure all implementations adhere to the specifications in `tubular_spec.md`
- Verify each implementation before moving to the next step
- Run all tests to ensure nothing is broken
- Clean up temporary files after implementation
- All temporary files must be created in and cleaned up from the /app/temp directory

## Temporary Files
- Under NO CIRCUMSTANCES should any temporary, debug, or backup file be created ANYWHERE other than inside the /app/temp directory
- ANY temporary, debug, or backup file should be created in the /app/temp directory
- Before creating any temporary file, first ensure the /app/temp directory exists: `mkdir -p /app/temp`
- After completing a body of work, the entire contents of the /app/temp directory should be emptied
- Nothing should be left in /app/temp after use - all temporary files should be deleted from the /app/temp directory after use
- DO NOT create ANY debug files ANYWHERE other than the /app/temp directory
- If no TEMP.md file exists, you should create it and follow this process
- Examples of files to put in /app/temp: debug_*.py, test_*.py, *_backup.py, *_fixed.py, temporary utility scripts, etc.

## Code Quality
- Write clean, well-documented code
- Follow the behavior described in the specification exactly
- Don't skip any steps in the implementation checklist
- EXTREME CARE must be taken to avoid getting stuck in infinite loops, especially during testing
- Protect against infinite loops by setting reasonable execution limits
- When testing, always set timeouts to prevent infinite execution

## Testing Safety
- ALWAYS implement safeguards against infinite loops when testing
- Set reasonable upper limits for execution time during tests
- If execution exceeds time limits, assume an infinite loop and stop manually
- NEVER ignore potential infinite loops - they must be resolved before continuing

## Regression Testing Requirements
- At the beginning of each phase, run all existing tests
- All tests that are passing at the start of the phase are considered the regression set
- Under NO CIRCUMSTANCES may you disable, remove, skip, or otherwise neuter any test in the regression set to make it pass
- As you implement a phase, new tests may be written and removed as appropriate to cover the current logic being implemented
- At the end of a phase, all tests in the regression set AND any new tests added to the test suite should be passing successfully

## Workspace Cleanup Requirements
- All temporary, debug, and backup files must be created in the /app/temp directory
- Clean up ALL contents of the /app/temp directory immediately after temporary files are no longer needed
- Remove backup files, temporary code files, debug scripts, and any other temporary artifacts from the /app/temp directory
- Maintain a clean workspace with only permanent project files outside of /app/temp
- After completing any phase or task, ensure the /app/temp directory is completely emptied
- Examples of files that should be in /app/temp: `debug_*.py`, `test_*.py`, `*_backup.py`, `*_fixed.py`, `*.log`, and other temporary artifacts
- NEVER leave any temporary files in /app/temp directory after they are no longer needed
- If the /app/temp directory doesn't exist, create it before placing any temporary files: `mkdir -p /app/temp`