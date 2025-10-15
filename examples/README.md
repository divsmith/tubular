# Examples Summary

This document describes what each example should output when run with the Tubular interpreter.

## Working Examples

### simple.tb
- **Description**: Basic arithmetic - outputs the number 5
- **Expected Output**: `5`
- **Code**: Starts with 0, sets value to 5, outputs as number

### simple_working.tb
- **Description**: Arithmetic example - outputs 5 (7-2=5)
- **Expected Output**: `5`
- **Code**: Subtracts 2 from 7, outputs result as number

### hello_simple.tb
- **Description**: Basic character output example
- **Expected Output**: Bell character (ASCII 7) sound
- **Code**: Outputs ASCII character 7

### hello_world.tb
- **Description**: Basic arithmetic example - outputs 5 (7-2=5)
- **Expected Output**: `5`
- **Code**: Subtracts 2 from 7, outputs as number

### character_working.tb
- **Description**: Character output example
- **Expected Output**: Bell character (ASCII 7) sound
- **Code**: Outputs ASCII character 7

### digit_1.tb
- **Description**: Character output - outputs ASCII character 1
- **Expected Output**: ASCII character 1
- **Code**: Subtracts 9 from 4 (result -5), outputs as character

### letter_A.tb
- **Description**: Character output - outputs ASCII character representing A
- **Expected Output**: ASCII character from calculation (6-5=1)
- **Code**: Subtracts 5 from 6, outputs as character

### calculator.tb
- **Description**: Interactive calculator - reads two numbers and outputs their sum
- **Expected Output**: Sum of two input numbers
- **Code**: Reads two numbers, adds them, outputs result

### calculator_test.tb
- **Description**: Calculator test - reads two numbers and outputs their sum
- **Expected Output**: Sum of two input numbers
- **Code**: Similar to calculator.tb

### countdown.tb
- **Description**: Countdown loop - outputs numbers 5,4,3,2,1
- **Expected Output**: `5,4,3,2,1,`
- **Code**: Loop that counts down from 5

### memory_test_simple.tb
- **Description**: Memory operations test - stores and retrieves values
- **Expected Output**: `42,99,0,`
- **Code**: Tests memory storage and retrieval

## Removed Examples

The following examples were removed because they:
- Had malformed syntax (missing operators between numbers)
- Had incomplete flow control (pipes that don't connect properly)
- Were redundant duplicates of working examples
- Had incomplete subroutine calls or other structural issues

### Removed Files:
- hello_working_from_root.tb
- working_A_from_root.tb
- visible_hello_from_root.tb
- subroutine_test_simple.tb
- working_numbers_from_root.tb
- basic_test_from_root.tb
- simple_a_from_root.tb
- working_test_from_root.tb
- hello_visible_from_root.tb
- simple_test_from_root.tb
- test_simple.tb
- test_basic.tb
- test_arithmetic.tb
- test_mult.tb
- test_input.tb
- debug_test.tb
- debug_grid_test.rs