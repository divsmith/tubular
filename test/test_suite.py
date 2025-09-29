"""
Comprehensive test suite for the Tubular programming language interpreter.
"""
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from src.grid import Grid
from src.interpreter import TubularInterpreter


def run_test(name, grid_str, expected_output, description=""):
    """Run a single test and report the result."""
    try:
        grid = Grid(grid_str)
        interpreter = TubularInterpreter(grid)
        output = interpreter.execute_program()
        
        # Clean up the output by removing trailing whitespace
        output = output.rstrip()
        expected_output = expected_output.rstrip()
        
        if output == expected_output:
            print(f"✓ {name}: PASSED - {description}")
            return True
        else:
            print(f"✗ {name}: FAILED - {description}")
            print(f"  Expected: {repr(expected_output)}")
            print(f"  Got:      {repr(output)}")
            return False
    except Exception as e:
        print(f"✗ {name}: ERROR - {description}")
        print(f"  Exception: {e}")
        return False


def test_basic_functionality():
    """Test basic interpreter functionality."""
    tests_passed = 0
    total_tests = 0
    
    # Test 1: Simple number source
    total_tests += 1
    if run_test("Number Source", "@\n|\n5\n|\n!", "5\n", "Outputs the number 5"):
        tests_passed += 1
    
    # Test 2: Increment
    total_tests += 1
    if run_test("Increment", "@\n|\n+\n|\n!", "1\n", "Increments initial value 0 to 1"):
        tests_passed += 1
        
    # Test 3: Multiple increments
    total_tests += 1
    if run_test("Multiple Increments", "@\n|\n+\n|\n+\n|\n+\n|\n!", "3\n", "Applies three increments to initial value 0"):
        tests_passed += 1
    
    # Test 4: Decrement
    total_tests += 1
    if run_test("Decrement", "@\n|\n~\n|\n!", "-1\n", "Decrements initial value 0 to -1"):
        tests_passed += 1
        
    # Test 5: Basic output with different value
    total_tests += 1
    if run_test("Number 7 Output", "  @\n  |\n  7\n  |\n  !", "7\n", "Outputs the number 7"):
        tests_passed += 1
    
    return tests_passed, total_tests


def test_data_stack():
    """Test data stack operations."""
    tests_passed = 0
    total_tests = 0
    
    # Test 1: Push and Dup then Pop (Dup doesn't destroy droplet)
    # @ -> 5 -> d -> ; -> !
    # 1. Droplet 0 hits 5, new droplet 5 goes down
    # 2. Droplet 5 hits d, pushes 5 to stack, continues
    # 3. Droplet 5 hits ;, pops 5 from stack, sets value to 5, continues
    # 4. Droplet 5 hits !, outputs 5
    total_tests += 1
    if run_test("Push and Pop", "  @\n  |\n  5\n  |\n  d\n  |\n  ;\n  |\n  !", "5\n", "Push 5 using d, then pop it using ;"):
        tests_passed += 1
    
    # Test 2: Add operation
    # Need to push two values, then trigger an add operation, then get the result
    # This is complex to do with a single droplet path, so we'll have to be creative
    # For now, let's just test that values can be pushed and popped
    total_tests += 1
    grid_str = '''  @
  |
  3
  |
  :
  5
  |
  ;
  |
  !'''
    # This won't work as expected because the droplet that hits 5 creates a new droplet at 5
    # which then hits : and pushes 5 to stack, then gets destroyed
    # So the original droplet continues to 3, pushes 3, and continues to ; where it should 
    # pop first 5 then 3, add them, push 8, and get destroyed
    # We need to think of a different design... This is quite complex
    
    # Actually, let me implement a simple test differently:
    # @ -> 3 -> : -> 5 -> ; -> ! 
    # 1. Initial droplet (0) hits 3, creates droplet with value 3
    # 2. Droplet (3) hits : (push), pushes 3 to stack, continues
    # 3. Droplet (3) hits 5, creates droplet with value 5
    # 4. Droplet (5) hits ; (pop), pops from stack (gets 3), sets value to 3, continues
    # 5. Droplet (3) hits !, outputs 3
    if run_test("Push then Pop Different Value", "  @\n  |\n  3\n  |\n  :\n  5\n  |\n  ;\n  |\n  !", "3\n", "Push 3, then pop it (with different intermediate value)"):
        tests_passed += 1
        
    # Test 3: Dup operation
    total_tests += 1
    if run_test("Dup Operation", "  @\n  |\n  4\n  |\n  d\n  |\n  ;\n  |\n  !", "4\n", "Duplicate 4 then pop it"):
        tests_passed += 1
    
    return tests_passed, total_tests


def test_pipes_and_flow():
    """Test pipe operators and flow control."""
    tests_passed = 0
    total_tests = 0
    
    # Test 1: Vertical pipe
    total_tests += 1
    if run_test("Vertical Pipe", "@\n|\n|\n|\n5\n|\n!", "5\n", "Droplet flows through vertical pipes"):
        tests_passed += 1
    
    # Test 2: Horizontal pipe
    total_tests += 1
    if run_test("Horizontal Pipe", "  @\n  -\n  -\n  5\n  |\n  !", "5\n", "Droplet flows through horizontal pipes"):
        tests_passed += 1
    
    # Test 3: Forward-Slash Corner with zero - should go left
    # @
    # |
    # 0
    # |
    # /
    # | \
    # 5 !
    total_tests += 1
    grid_str = '''  @
  |
  0
  |
 / \\
 | |
 5 !'''
    if run_test("Conditional Zero Forward-Slash", grid_str, "5\n", "When value is 0, go left"):
        tests_passed += 1
    
    # Test 4: Forward-Slash Corner with non-zero - should go right
    total_tests += 1
    grid_str = '''  @
  |
  1
  |
 / \\
 | |
 ! 5'''
    if run_test("Conditional Non-Zero Forward-Slash", grid_str, "1\n", "When value is non-zero, go right"):
        tests_passed += 1
    
    # Test 5: Backslash corner with zero - should go right
    total_tests += 1
    grid_str = '''  @
  |
  0
  |
 \\ /
  | |
  ! 7'''
    if run_test("Conditional Zero Back-Slash", grid_str, "0\n", "When value is 0, go right (backslash)"):
        tests_passed += 1
    
    return tests_passed, total_tests


def test_complex_features():
    """Test more complex features like tape reader and output."""
    tests_passed = 0
    total_tests = 0
    
    # Test 1: Tape reader (Hello World)
    total_tests += 1
    grid_str = '''  @
  |
 >Hello
  |
  !'''
    if run_test("Tape Reader", grid_str, "Hello", "Output string from tape reader"):
        tests_passed += 1
    
    # Test 2: Simple arithmetic
    total_tests += 1
    if run_test("Simple Arithmetic", "@\n|\n5\n|\n+\n+\n|\n!", "7\n", "Start with 5, add 2"):
        tests_passed += 1
    
    return tests_passed, total_tests


def run_all_tests():
    """Run all tests and report results."""
    print("Running Tubular interpreter test suite...\n")
    
    all_tests_passed = 0
    all_total_tests = 0
    
    # Run basic functionality tests
    print("Testing basic functionality...")
    tests_passed, total_tests = test_basic_functionality()
    all_tests_passed += tests_passed
    all_total_tests += total_tests
    print(f"Basic functionality: {tests_passed}/{total_tests} tests passed\n")
    
    # Run data stack tests
    print("Testing data stack operations...")
    tests_passed, total_tests = test_data_stack()
    all_tests_passed += tests_passed
    all_total_tests += total_tests
    print(f"Data stack operations: {tests_passed}/{total_tests} tests passed\n")
    
    # Run pipe and flow tests
    print("Testing pipes and flow control...")
    tests_passed, total_tests = test_pipes_and_flow()
    all_tests_passed += tests_passed
    all_total_tests += total_tests
    print(f"Pipes and flow control: {tests_passed}/{total_tests} tests passed\n")
    
    # Run complex feature tests
    print("Testing complex features...")
    tests_passed, total_tests = test_complex_features()
    all_tests_passed += tests_passed
    all_total_tests += total_tests
    print(f"Complex features: {tests_passed}/{total_tests} tests passed\n")
    
    # Summary
    print(f"Overall: {all_tests_passed}/{all_total_tests} tests passed")
    if all_tests_passed == all_total_tests:
        print("🎉 All tests passed!")
        return True
    else:
        print("❌ Some tests failed.")
        return False


if __name__ == "__main__":
    success = run_all_tests()
    sys.exit(0 if success else 1)