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
    # @ (col2)
    # | (col2)
    # 0 (col2) - creates new droplet with value 0 going down
    # | (col2)
    # | (col2)
    #  / (col2) - new droplet from 0 hits this, value 0, goes LEFT
    # -| (row: - at col1, | at col2) 
    #  | (col2, continues original path down)
    #  5 (col2)
    #  |
    #  !
    # Actually the above is still wrong. The original path and the / need to be the same:
    # @ (col2)
    # | (col2)
    # 0 (col2) - original droplet destroyed, new one created with value 0
    # | (col2, the new droplet continues)
    # | (col2, the new droplet continues)
    # / (col2, the new droplet hits / with value 0, goes LEFT to col 1)
    # Now we need a path in col 1 continuing to 5:
    # @ (col2)
    # | (col2) 
    # 0 (col2)
    # | (col2)
    # | (col2)
    #-/ (col1,2 - - at col1, / at col2)
    # | (col2, continues down the main path)
    # 5 (col2)
    # | (col2) 
    # ! (col2)
    # 
    # When the droplet from above hits / at (2,5) with value 0, it goes LEFT to (1,5)
    # (1,5) is '-', so it continues in current direction (LEFT) to (0,5)
    # This still doesn't connect to the 5 at (2,6)...
    # 
    # Let's create a layout where:
    # - Main path brings droplet with value 0 to the '/'
    # - '/' in col 1, row 5: droplet (x,4) → (x,5)/ → LEFT to (x-1,5) 
    # - (x-1,5) has a '-', leading to a downward path
    # - From @ at (2,0) we go (2,1)→(2,2)0→new droplet→(2,3)→(2,4)→(2,5)/
    # - From (2,5) with value 0, go LEFT to (1,5)
    # - Need (1,5) to be '-' and then continue down
    # Row layout:
    # Row 0:  @  → (2,0)@
    # Row 1:  |  → (2,1)|
    # Row 2:  0  → (2,2)0, creates new droplet
    # Row 3:  |  → (2,3)| for new droplet 
    # Row 4:  |  → (2,4)| for new droplet
    # Row 5: " / → (1,5) , (2,5)/ - the new droplet hits / at (2,5)
    # This doesn't work since (1,5) would be space
    #
    # Let me arrange:
    #  @  (2,0)@
    #  |  (2,1)|  
    #  0  (2,2)0
    #  |  (2,3)|  
    # -/  (1,4)-, (2,4)/ - droplet goes (2,3)→(2,4)/
    #  |  (2,5)| - continues original path
    #  5  (2,6)5
    #  |  (2,7)|
    #  !  (2,8)!
    #
    # When droplet at (2,4) with value 0 hits / at (2,4), it goes LEFT to (1,4) which has '-'.
    # From (1,4) with '-', if coming from above, it passes through to (1,5) - but (1,5) is space.
    # 
    # For a turn from horizontal pipe to vertical, I need a corner piece:
    #  @
    #  |
    #  0
    #  | 
    # -/
    #  \\
    #  |
    #  5
    #  |
    #  !
    # 
    # Row 4: "-/" → (0,4)-, (1,4)/
    # Row 5: " \\" → (0,5)space, (1,5)\\ but this is too short
    # Row 5: " \\" → (0,5)' ', (1,5)'\\' (padded to width 2+)
    # After padding to match max width: (0,5)' ', (1,5)'\\', (2,5)' '
    # 
    # So the turn is: droplet from (1,3) to (1,4)/ with value 0 → go LEFT to (0,4)- → continue LEFT?
    # No: entering / from TOP with value 0 → direction becomes LEFT → next tick: move LEFT from (1,4) to (0,4)
    # At (0,4) is '-' → entered from RIGHT (since previous position was (1,4)) → continues LEFT
    # Go from (0,4) to (0,5) which should have a path down
    # 
    # Let's try:
    #  @
    #  |
    #  0  (droplet goes down to row 4 col 2 which is space if / is in row 4 col 1...)
    # This is getting complex. Let me be precise with the final layout:
    # 
    # We want: A droplet with value 0 arrives at a '/' and goes left, then that path leads to '5' and '!'
    #
    # Layout:
    #  @
    #  |  (vertical path from @ in col 2)
    #  0
    # -/  (row: - in col 1, / in col 2. Col 0 is space if needed for padding)
    #  |  (continuing original path in col 2)
    #  5  (in col 2)
    #  |
    #  !
    # 
    # This will have the droplet go: (2,0)@(2,1)|(2,2)0 → new droplet (2,2) → (2,3) → (2,4) which is space!
    # 
    # The / is at (2,3) in this layout. Let me fix:
    # 
    #  @
    #  0  (original droplet hits 0, creates new droplet with value 0)
    #  |  (new droplet continues)
    # -/  (new droplet hits / at (2,3), value 0, goes LEFT to (1,3) which is -)
    #  |  (in col 2 continuing down)
    #  5  (in col 2)
    #  |
    #  !
    # 
    # String:
    # Row 0: "  @" → (0,0) , (1,0) , (2,0)@
    # Row 1: "  0" → (0,1) , (1,1) , (2,1)0
    # Row 2: "  |" → (0,2) , (1,2) , (2,2)|
    # Row 3: " -/" → (0,3) , (1,3)-, (2,3)/ - new droplet hits this
    # Row 4: "  |" → (0,4) , (1,4) , (2,4)|
    # Row 5: "  5" → (0,5) , (1,5) , (2,5)5  
    # Row 6: "  |" → (0,6) , (1,6) , (2,6)|
    # Row 7: "  !" → (0,7) , (1,7) , (2,7)!
    # 
    # New droplet path: (2,1) → (2,2) → (2,3)/[value 0] → go LEFT to (1,3) which is '-'.
    # At '-' entered from top, what happens? Horizontal pipe spec says it continues in current direction,
    # which is DOWN. So from (1,3) it goes to (1,4) which is space.
    # 
    # I think I need to make the turn happen in a different way. 
    # Let me use the backslash for a different kind of turn:
    # 
    #  @
    #  |
    #  0
    # -/  (row: - in col1, / in col2)
    #  \\  (row: \\ in col1 - this will redirect droplet going left to go down)
    #  |  (row: | in col1 - continues down)
    #  5  (row: 5 in col1)
    #  |
    #  !
    # 
    # String representation:
    total_tests += 1
    grid_str = '''  @
  |
  0
 -/
  \\
  |
  5
  |
  !'''
    if run_test("Conditional Zero Forward-Slash", grid_str, "5\\n", "When value is 0, go left"):
        tests_passed += 1
    
    # Test 4: Forward-Slash Corner with non-zero - should go right\n    # @ (col2)\n    # | (col2)\n    # 1 (col2) - creates new droplet with value 1\n    # | (col2) - new droplet continues\n    # / (col2) - new droplet hits / with value 1, goes RIGHT to col 3\n    # For this to work properly, I need:\n    #  @\n    #  |\n    #  1\n    # -/\n    #  \\\n    #  |\n    #  5\n    #  |\n    #  !\n    # When value 1 droplet hits / at (2,3), it goes RIGHT to (3,3)\n    # (3,3) needs to have a path continuing down to 5\n    # After padding, row 3 is: (0,3) (1,3)- (2,3)/ (3,3) \n    # So (3,3) is space. Instead I need:\n    #  @\n    #  | \n    #  1\n    # -/ (row: - at col1, / at col2)  \n    #  | (continuing in col2)\n    #  5 (in col2)\n    #  | \n    #  !\n    # When droplet with value 1 hits / at (2,3), goes RIGHT to (3,3) - but that's space in padded version\n    #\n    # Actually, let me think differently. For the droplet to go right from (x,y) to (x+1,y),\n    # (x+1,y) needs to be a valid path.\n    # \n    # So I need:\n    #  @\n    #  |\n    #  1\n    # /+ (where + is to the right of /, but + isn't a valid char)\n    # Instead use:\n    #  @\n    #  |\n    #  1\n    # /- (where - is to the right of /)\n    # |  (continuing the original path)\n    # 5\n    # |\n    # !\n    # \n    # Row 3: \"/-\" → (0,3)/, (1,3)-  Wait, no - that puts / in col 0\n    # \n    # Row 2: \"  1\" → (2,2)1\n    # Row 3: \" /-\" → (1,3) , (2,3)/, (3,3)-\n    # When droplet hits (2,3)/ with value 1, goes RIGHT to (3,3)-\n    # At (3,3)- entered from TOP, continues in current direction (DOWN) to (3,4)\n    # (3,4) needs to be valid path to 5...\n    #\n    # Let me do:\n    #  @\n    #  |\n    #  1  \n    # -/\n    #  |\n    #  |\n    #  - (right path: after going right from /)\n    #  |\n    #  5 (in same col as - in row 6)\n    #  |\n    #  !\n    # \n    # Actually, this is getting too complex. Here's a simpler approach:\n    #  @\n    # 1| (row 1: 1 col1, | col2)\n    # /| (row 2: / col1, | col2) - droplet with val 1 hits /, goes RIGHT to (2,2) which is |\n    # || (row 3: | col1, | col2) - the moved droplet continues down at col2\n    # 5| (row 4: 5 col1, | col2) - Wait, if droplet went right to col2, it's already there\n    # \n    # So: (1,0)@ → (1,1)1 → new droplet val 1 at (1,1) → (1,2) → (1,3)/[val 1] → RIGHT to (2,3)| → (2,4)| → (2,5)5\n    # String:\n    total_tests += 1\n    grid_str = ''' @|\n 1|\n /|\n ||\n 5|\n ||\n !|\'''    if run_test(\"Conditional Non-Zero Forward-Slash\", grid_str, \"5\\n\", \"When value is non-zero, go right\"):\n        tests_passed += 1\n
    
    # Test 5: Backslash corner with zero - should go right
    # Backslash behavior when entering from TOP:
    # If droplet value is 0, direction becomes Right.
    # If non-zero, direction becomes Left.
    # 
    # To create a droplet with value 0 that enters the backslash from TOP:
    #  @
    #  |
    #  0 (new droplet with value 0 created)
    #  | (value 0 droplet continues)
    #  \ (value 0 droplet enters \ from top, goes RIGHT)
    # For the right path to go down to output:
    #  @
    #  |
    #  0
    #  |
    #  \- (backslash at col2, - at col3)
    #  |  (| in col3 - continuing the rightward path down)
    #  5  (5 in col3)
    #  |
    #  !
    # 
    # Path: (2,0)@ → (2,1)| → (2,2)0 → new droplet (2,2)val=0 → (2,3)| → (2,4) → (2,5)\[val 0] → RIGHT to (3,5)- → (3,6)| → (3,7)5
    # String:
    total_tests += 1
    grid_str = '''  @
  |
  0
  |
  \\\\
 --
  |
  5
  |
  !'''
    # Test 5: Backslash corner with zero - should go right
    total_tests += 1
    grid_str = """  @
  |
  0
  |
 \
 --
  |
  5
  |
  !
"""
    if run_test("Conditional Zero Back-Slash", grid_str, "5\n", "When value is 0, go right (backslash)"):
        tests_passed += 1

    return tests_passed, total_tests
