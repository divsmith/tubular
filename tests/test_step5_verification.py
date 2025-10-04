#!/usr/bin/env python3
"""
Step 5 Verification Tests for Tubular Language Implementation.

Tests the file loading and parsing functionality.
"""

import unittest
import tempfile
import os
import sys
from pathlib import Path

# Add src directory to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent / "src"))

from parser import TubFileParser
from grid import Grid
from engine import Engine
from droplet import Droplet
from direction import Direction


class TestStep5Verification(unittest.TestCase):
    """Test cases for Step 5: File Loading and Parsing."""

    def setUp(self):
        """Set up test fixtures."""
        self.parser = TubFileParser()

    def test_parser_creation(self):
        """Test that TubFileParser can be instantiated."""
        parser = TubFileParser()
        self.assertIsInstance(parser, TubFileParser)

    def test_parse_basic_program(self):
        """Test parsing the basic verification program from Step 4."""
        # Create temporary file with the basic program
        program_content = "@\n|\nn\n"

        with tempfile.NamedTemporaryFile(mode='w', suffix='.tub', delete=False) as f:
            f.write(program_content)
            temp_file_path = f.name

        try:
            # Parse the file
            grid = self.parser.parse_file(temp_file_path)

            # Verify grid dimensions (3x3 to accommodate the content)
            self.assertEqual(grid.width, 1)  # Max line length is 1
            self.assertEqual(grid.height, 3)  # 3 lines

            # Verify grid contents
            self.assertEqual(grid.get(0, 0), '@')
            self.assertEqual(grid.get(0, 1), '|')
            self.assertEqual(grid.get(0, 2), 'n')

        finally:
            # Clean up temporary file
            os.unlink(temp_file_path)

    def test_parse_wider_program(self):
        """Test parsing a program with wider lines."""
        program_content = " @ \n | \n  n\n"

        with tempfile.NamedTemporaryFile(mode='w', suffix='.tub', delete=False) as f:
            f.write(program_content)
            temp_file_path = f.name

        try:
            grid = self.parser.parse_file(temp_file_path)

            # Should be 3x3 (widest line is 3 characters)
            self.assertEqual(grid.width, 3)
            self.assertEqual(grid.height, 3)

            # Verify content with padding
            self.assertEqual(grid.get(0, 0), ' ')
            self.assertEqual(grid.get(1, 0), '@')
            self.assertEqual(grid.get(2, 0), ' ')
            self.assertEqual(grid.get(0, 1), ' ')
            self.assertEqual(grid.get(1, 1), '|')
            self.assertEqual(grid.get(2, 1), ' ')
            self.assertEqual(grid.get(0, 2), ' ')
            self.assertEqual(grid.get(2, 2), 'n')

        finally:
            os.unlink(temp_file_path)

    def test_parse_program_with_empty_lines(self):
        """Test parsing a program with empty lines (should be filtered out)."""
        program_content = "@\n\n|\n\nn\n"

        with tempfile.NamedTemporaryFile(mode='w', suffix='.tub', delete=False) as f:
            f.write(program_content)
            temp_file_path = f.name

        try:
            grid = self.parser.parse_file(temp_file_path)

            # Empty lines should be filtered, so height should be 3
            self.assertEqual(grid.width, 1)
            self.assertEqual(grid.height, 3)

            self.assertEqual(grid.get(0, 0), '@')
            self.assertEqual(grid.get(0, 1), '|')
            self.assertEqual(grid.get(0, 2), 'n')

        finally:
            os.unlink(temp_file_path)

    def test_parse_program_with_comments(self):
        """Test parsing a program with whitespace-only lines."""
        program_content = "@\n  \n\t\n|\n \nn\n"

        with tempfile.NamedTemporaryFile(mode='w', suffix='.tub', delete=False) as f:
            f.write(program_content)
            temp_file_path = f.name

        try:
            grid = self.parser.parse_file(temp_file_path)

            # Whitespace-only lines should be filtered
            self.assertEqual(grid.width, 1)
            self.assertEqual(grid.height, 3)

            self.assertEqual(grid.get(0, 0), '@')
            self.assertEqual(grid.get(0, 1), '|')
            self.assertEqual(grid.get(0, 2), 'n')

        finally:
            os.unlink(temp_file_path)

    def test_get_file_info(self):
        """Test getting file information without creating a Grid."""
        program_content = "@\n|\nn\n"

        with tempfile.NamedTemporaryFile(mode='w', suffix='.tub', delete=False) as f:
            f.write(program_content)
            temp_file_path = f.name

        try:
            width, height, line_count = self.parser.get_file_info(temp_file_path)

            self.assertEqual(width, 1)
            self.assertEqual(height, 3)  # Non-empty lines after processing
            self.assertEqual(line_count, 3)  # Original line count

        finally:
            os.unlink(temp_file_path)

    def test_validate_file_success(self):
        """Test file validation with a valid file."""
        program_content = "@\n|\nn\n"

        with tempfile.NamedTemporaryFile(mode='w', suffix='.tub', delete=False) as f:
            f.write(program_content)
            temp_file_path = f.name

        try:
            result = self.parser.validate_file(temp_file_path)
            self.assertTrue(result)

        finally:
            os.unlink(temp_file_path)

    def test_validate_file_invalid_extension(self):
        """Test file validation with invalid file extension."""
        program_content = "@\n|\nn\n"

        with tempfile.NamedTemporaryFile(mode='w', suffix='.txt', delete=False) as f:
            f.write(program_content)
            temp_file_path = f.name

        try:
            with self.assertRaises(ValueError):
                self.parser.parse_file(temp_file_path)

        finally:
            os.unlink(temp_file_path)

    def test_parse_nonexistent_file(self):
        """Test parsing a file that doesn't exist."""
        with self.assertRaises(FileNotFoundError):
            self.parser.parse_file("nonexistent.tub")

    def test_parse_empty_file(self):
        """Test parsing an empty file."""
        with tempfile.NamedTemporaryFile(mode='w', suffix='.tub', delete=False) as f:
            temp_file_path = f.name

        try:
            with self.assertRaises(ValueError):
                self.parser.parse_file(temp_file_path)

        finally:
            os.unlink(temp_file_path)

    def test_whitespace_only_file(self):
        """Test parsing a file with only whitespace."""
        program_content = "  \n\t\n  \n"

        with tempfile.NamedTemporaryFile(mode='w', suffix='.tub', delete=False) as f:
            f.write(program_content)
            temp_file_path = f.name

        try:
            with self.assertRaises(ValueError):
                self.parser.parse_file(temp_file_path)

        finally:
            os.unlink(temp_file_path)

    def test_end_to_end_file_loading_and_execution(self):
        """Test complete end-to-end file loading and program execution."""
        program_content = "@\n|\nn\n"

        with tempfile.NamedTemporaryFile(mode='w', suffix='.tub', delete=False) as f:
            f.write(program_content)
            temp_file_path = f.name

        try:
            # Load and parse the file
            grid = self.parser.parse_file(temp_file_path)

            # Create engine and execute
            engine = Engine(grid)

            # Run until completion
            tick_count = 0
            while not engine.is_empty() and tick_count < 100:
                engine.tick()
                tick_count += 1

            # Verify execution completed
            self.assertEqual(len(engine.get_active_droplets()), 0)

        finally:
            os.unlink(temp_file_path)

    def test_test_program_tub_loading_and_execution(self):
        """Test loading and execution of test_program.tub from tests/resources."""
        # Get the path to the test program file
        project_root = Path(__file__).parent.parent
        test_program_path = project_root / "tests" / "resources" / "test_program.tub"

        # Verify file exists
        self.assertTrue(test_program_path.exists(), f"test_program.tub not found at {test_program_path}")

        # Test file parsing and dimensions
        grid = self.parser.parse_file(str(test_program_path))
        self.assertEqual(grid.width, 1)
        self.assertEqual(grid.height, 3)

        # Verify character placement
        self.assertEqual(grid.get(0, 0), '@')
        self.assertEqual(grid.get(0, 1), '|')
        self.assertEqual(grid.get(0, 2), 'n')

        # Test end-to-end execution - should output '0'
        # We need to capture stdout to verify the output
        import io
        from contextlib import redirect_stdout

        f = io.StringIO()
        with redirect_stdout(f):
            engine = Engine(grid)
            tick_count = 0
            while not engine.is_empty() and tick_count < 100:
                engine.tick()
                tick_count += 1

        output = f.getvalue().strip()
        self.assertEqual(output, '0', "test_program.tub should output '0'")

    def test_sample_program_tub_loading_and_execution(self):
        """Test loading and execution of sample_program.tub from tests/resources."""
        # Get the path to the sample program file
        project_root = Path(__file__).parent.parent
        sample_program_path = project_root / "tests" / "resources" / "sample_program.tub"

        # Verify file exists
        self.assertTrue(sample_program_path.exists(), f"sample_program.tub not found at {sample_program_path}")

        # Test file parsing and dimensions
        grid = self.parser.parse_file(str(sample_program_path))
        self.assertEqual(grid.width, 3)  # '@..' is 3 characters wide
        self.assertEqual(grid.height, 5)

        # Verify character placement
        self.assertEqual(grid.get(0, 0), '@')
        self.assertEqual(grid.get(1, 0), '.')
        self.assertEqual(grid.get(2, 0), '.')
        self.assertEqual(grid.get(0, 1), '|')
        self.assertEqual(grid.get(1, 1), '.')
        self.assertEqual(grid.get(2, 1), '.')
        self.assertEqual(grid.get(0, 2), '+')
        self.assertEqual(grid.get(1, 2), '.')
        self.assertEqual(grid.get(2, 2), '.')
        self.assertEqual(grid.get(0, 3), '|')
        self.assertEqual(grid.get(1, 3), '.')
        self.assertEqual(grid.get(2, 3), '.')
        self.assertEqual(grid.get(0, 4), 'n')
        self.assertEqual(grid.get(1, 4), '.')
        self.assertEqual(grid.get(2, 4), '.')

        # Test end-to-end execution - should output '1'
        # We need to capture stdout to verify the output
        import io
        from contextlib import redirect_stdout

        f = io.StringIO()
        with redirect_stdout(f):
            engine = Engine(grid)
            tick_count = 0
            while not engine.is_empty() and tick_count < 100:
                engine.tick()
                tick_count += 1

        output = f.getvalue().strip()
        self.assertEqual(output, '1', "sample_program.tub should output '1'")

    def test_sample_files_end_to_end_pipeline(self):
        """Test that both sample files work correctly through the complete pipeline."""
        project_root = Path(__file__).parent.parent

        # Test both files
        test_files = [
            ("tests/resources/test_program.tub", '0', 1, 3),  # filename, expected_output, width, height
            ("tests/resources/sample_program.tub", '1', 3, 5)
        ]

        for filename, expected_output, expected_width, expected_height in test_files:
            with self.subTest(file=filename):
                filepath = project_root / filename

                # Verify file exists and is readable
                self.assertTrue(filepath.exists(), f"{filename} not found")
                self.assertTrue(filepath.stat().st_size > 0, f"{filename} is empty")

                # Test parsing
                grid = self.parser.parse_file(str(filepath))
                self.assertEqual(grid.width, expected_width)
                self.assertEqual(grid.height, expected_height)

                # Test execution produces expected output
                import io
                from contextlib import redirect_stdout

                f = io.StringIO()
                with redirect_stdout(f):
                    engine = Engine(grid)
                    tick_count = 0
                    while not engine.is_empty() and tick_count < 100:
                        engine.tick()
                        tick_count += 1

                output = f.getvalue().strip()
                self.assertEqual(output, expected_output, f"{filename} should output '{expected_output}'")

                # Verify execution completed (no infinite loops)
                self.assertLess(tick_count, 100, f"{filename} execution took too many ticks")


if __name__ == '__main__':
    unittest.main()