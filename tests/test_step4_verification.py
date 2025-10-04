#!/usr/bin/env python3
"""
Step 4 Verification Tests for Tubular Language Implementation.

Tests the @ (Program Start) and n (Numeric Output) operators.
"""

import unittest
import tempfile
import os
import subprocess
import sys
from pathlib import Path

# Add src directory to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from src.grid import Grid
from src.droplet import Droplet
from src.direction import Direction
from src.engine import Engine


class TestStep4Verification(unittest.TestCase):
    """Test cases for Step 4: Program Start & Basic Output."""

    def test_program_start_creates_initial_droplet(self):
        """Test that @ creates an initial droplet with value 0, direction DOWN."""
        # Create a simple grid with @ in the middle
        grid = Grid(3, 3)
        grid._grid[1][1] = '@'

        # Create engine - should automatically find @ and create droplet
        engine = Engine(grid)

        # Verify initial droplet was created
        self.assertEqual(len(engine.get_active_droplets()), 1)

        droplet = engine.get_active_droplets()[0]
        self.assertEqual(droplet.value, 0)
        self.assertEqual(droplet.x, 1)
        self.assertEqual(droplet.y, 1)
        self.assertEqual(droplet.direction, Direction.DOWN)

    def test_numeric_output_operator(self):
        """Test that n operator prints droplet value and destroys droplet."""
        # Create a simple grid with @ and n
        grid = Grid(3, 3)
        grid._grid[0][0] = '@'
        grid._grid[1][0] = 'n'

        engine = Engine(grid)

        # Capture stdout to verify output
        import io
        from contextlib import redirect_stdout

        f = io.StringIO()
        with redirect_stdout(f):
            # Run one tick - droplet should move down and hit n
            engine.tick()

        output = f.getvalue().strip()
        self.assertEqual(output, "0")

        # Verify droplet was destroyed
        self.assertEqual(len(engine.get_active_droplets()), 0)

    def test_verification_program_execution(self):
        """Test the exact verification program from the checklist."""
        # Create temporary file with the verification program
        program_content = "@\n|\nn\n"

        with tempfile.NamedTemporaryFile(mode='w', suffix='.tub', delete=False) as f:
            f.write(program_content)
            temp_file_path = f.name

        try:
            # Run the program using the compiler
            result = subprocess.run([
                sys.executable, str(Path(__file__).parent.parent / "src" / "compiler.py"), temp_file_path
            ], capture_output=True, text=True, cwd=Path(__file__).parent.parent)

            # Print debug info if test fails
            if result.returncode != 0:
                print(f"STDOUT: {result.stdout}")
                print(f"STDERR: {result.stderr}")
                print(f"Return code: {result.returncode}")

            # Verify it ran successfully
            self.assertEqual(result.returncode, 0)

            # Verify output is "0" followed by program completion message
            output_lines = result.stdout.strip().split('\n')
            self.assertTrue(any("0" in line for line in output_lines))
            self.assertIn("Program execution completed.", result.stdout)

        finally:
            # Clean up temporary file
            os.unlink(temp_file_path)

    def test_no_program_start_character(self):
        """Test behavior when no @ character is present."""
        # Create grid without @
        grid = Grid(2, 2)
        grid._grid[0][0] = 'n'

        # Engine should initialize without any droplets
        engine = Engine(grid)
        self.assertEqual(len(engine.get_active_droplets()), 0)

    def test_multiple_program_start_characters(self):
        """Test behavior when multiple @ characters are present."""
        # Create grid with multiple @
        grid = Grid(3, 3)
        grid._grid[0][0] = '@'
        grid._grid[2][2] = '@'

        # Engine should only create one droplet at first @
        engine = Engine(grid)
        self.assertEqual(len(engine.get_active_droplets()), 1)

        droplet = engine.get_active_droplets()[0]
        self.assertEqual(droplet.x, 0)
        self.assertEqual(droplet.y, 0)


if __name__ == '__main__':
    unittest.main()