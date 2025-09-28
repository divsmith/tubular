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