#!/usr/bin/env python3
"""
Step 6 Verification Tests for Tubular Language Implementation.

Tests the complete end-to-end execution of Tubular programs including:
- File loading and parsing
- Grid creation and initialization
- Program execution with tick-based simulation
- Output capture and verification
- Execution completion verification
"""

import pytest
import sys
import os
import io
from contextlib import redirect_stdout
from pathlib import Path

# Add src directory to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from parser import TubFileParser
from grid import Grid
from engine import Engine


def test_step6_verification_complete_execution():
    """
    Test complete end-to-end execution of the step 6 verification program.

    This test verifies that the Tubular program in test.tub executes correctly,
    producing the expected output and completing within a reasonable number of ticks.
    """
    # Get the path to the test program file
    project_root = Path(__file__).parent.parent
    test_program_path = project_root / "tests" / "resources" / "test.tub"

    # Verify file exists
    assert test_program_path.exists(), f"test.tub not found at {test_program_path}"
    assert test_program_path.stat().st_size > 0, "test.tub is empty"

    # Load and parse the file
    parser = TubFileParser()
    grid = parser.parse_file(str(test_program_path))

    # Verify grid dimensions and content
    assert grid.width == 1, f"Expected width 1, got {grid.width}"
    assert grid.height == 9, f"Expected height 9, got {grid.height}"

    # Verify key characters are present
    assert grid.get(0, 0) == '@', "Program should start with '@' at (0, 0)"
    assert grid.get(0, 8) == 'n', "Program should end with 'n' at (0, 8)"

    # Execute program and capture output
    f = io.StringIO()
    with redirect_stdout(f):
        engine = Engine(grid)
        tick_count = 0
        max_ticks = 100  # Reasonable upper limit

        while not engine.is_empty() and tick_count < max_ticks:
            engine.tick()
            tick_count += 1

    # Get the captured output
    output = f.getvalue().strip()

    # Verify output is correct
    assert output == "1", f"Expected output '1', got '{output}'"

    # Verify execution completed within reasonable time
    assert tick_count < max_ticks, f"Program took too many ticks ({tick_count} >= {max_ticks})"

    # Verify all droplets completed (no infinite loops)
    assert engine.is_empty(), "Engine should be empty after program completion"


def test_step6_verification_program_structure():
    """
    Test that the verification program has the expected structure.

    Verifies the grid layout and character placement for the step 6 program.
    """
    project_root = Path(__file__).parent.parent
    test_program_path = project_root / "tests" / "resources" / "test.tub"

    parser = TubFileParser()
    grid = parser.parse_file(str(test_program_path))

    # Verify specific character positions
    expected_positions = [
        (0, 0, '@'),  # Program start
        (0, 1, '|'),  # Vertical pipe
        (0, 2, '+'),  # Increment operator
        (0, 3, '|'),  # Vertical pipe
        (0, 4, '+'),  # Increment operator
        (0, 5, '|'),  # Vertical pipe
        (0, 6, '~'),  # Decrement operator
        (0, 7, '|'),  # Vertical pipe
        (0, 8, 'n'),  # Numeric output
    ]

    for x, y, expected_char in expected_positions:
        assert grid.get(x, y) == expected_char, f"Expected '{expected_char}' at ({x}, {y}), got '{grid.get(x, y)}'"


def test_step6_verification_execution_mechanics():
    """
    Test the execution mechanics of the step 6 verification program.

    Verifies that the program correctly:
    1. Starts with a droplet at '@' with value 0 moving DOWN
    2. Increments the value at '+' operators
    3. Decrements the value at '~' operator
    4. Outputs the final value at 'n'
    5. Completes execution properly
    """
    project_root = Path(__file__).parent.parent
    test_program_path = project_root / "tests" / "resources" / "test.tub"

    parser = TubFileParser()
    grid = parser.parse_file(str(test_program_path))

    # Execute with output capture
    f = io.StringIO()
    with redirect_stdout(f):
        engine = Engine(grid)

        # Verify initial state
        initial_droplets = engine.get_active_droplets()
        assert len(initial_droplets) == 1, "Should start with exactly one droplet"

        droplet = initial_droplets[0]
        assert droplet.value == 0, f"Initial droplet value should be 0, got {droplet.value}"
        assert droplet.x == 0 and droplet.y == 0, f"Initial droplet should be at (0, 0), got ({droplet.x}, {droplet.y})"

        # Run until completion
        tick_count = 0
        while not engine.is_empty() and tick_count < 50:
            engine.tick()
            tick_count += 1

    output = f.getvalue().strip()
    assert output == "1", f"Program should output '1', got '{output}'"
    assert tick_count < 50, f"Program should complete in less than 50 ticks, took {tick_count}"
    assert engine.is_empty(), "All droplets should be consumed by the end"


def test_step6_verification_no_infinite_loops():
    """
    Test that the verification program doesn't run into infinite loops.

    Ensures the program terminates properly and doesn't exceed reasonable tick limits.
    """
    project_root = Path(__file__).parent.parent
    test_program_path = project_root / "tests" / "resources" / "test.tub"

    parser = TubFileParser()
    grid = parser.parse_file(str(test_program_path))

    engine = Engine(grid)

    # Run with a reasonable tick limit
    max_ticks = 200
    tick_count = 0

    while not engine.is_empty() and tick_count < max_ticks:
        engine.tick()
        tick_count += 1

    # Verify completion
    assert engine.is_empty(), "Program should complete (no active droplets remaining)"
    assert tick_count < max_ticks, f"Program should complete in less than {max_ticks} ticks, took {tick_count}"

    # Verify we actually executed some ticks (program did run)
    assert tick_count > 0, "Program should execute at least one tick"


def test_step6_verification_file_validation():
    """
    Test file validation and error handling for the verification program.

    Ensures the test file is valid and can be processed correctly.
    """
    project_root = Path(__file__).parent.parent
    test_program_path = project_root / "tests" / "resources" / "test.tub"

    parser = TubFileParser()

    # Test file validation
    assert parser.validate_file(str(test_program_path)), "test.tub should be a valid file"

    # Test file info retrieval
    width, height, line_count = parser.get_file_info(str(test_program_path))
    assert width == 1, f"Expected width 1, got {width}"
    assert height == 9, f"Expected height 9, got {height}"  # Non-empty lines after processing
    assert line_count == 9, f"Expected 9 total lines, got {line_count}"  # Original line count

    # Test that parsing works correctly
    grid = parser.parse_file(str(test_program_path))
    assert isinstance(grid, Grid), "parse_file should return a Grid instance"
    assert grid.width == width, "Grid width should match file info"
    assert grid.height == height, "Grid height should match file info"