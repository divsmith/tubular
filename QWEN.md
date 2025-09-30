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

## Temporary Files
- Whenever creating a temporary utility file, test for debugging purposes, or any other file that needs to be removed later, add its name to TEMP.md
- Before finishing your current implementation step, delete all the files listed in TEMP.md
- All tests should still pass, and the build should still succeed after removing all the files listed in TEMP.md
- Finally, clear out the contents all of TEMP.md

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
- Clean up all temporary files immediately after they are no longer needed
- Remove backup files, temporary code files, debug scripts, and any other temporary artifacts
- Maintain a clean workspace with only permanent project files
- Examples of temporary files to clean up: `*.py` files created for testing/debugging, `*_backup.py`, `*_fixed.py`, `*.log`, and other temporary artifacts
- Before completing any phase, run a cleanup to remove temporary files that are no longer needed