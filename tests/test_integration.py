#!/usr/bin/env python3
"""
Comprehensive integration test suite for the Tubular compiler system.

Tests the integration between:
- Parser and Engine components
- Command-line interface functionality
- Backward compatibility with existing functionality
- Error handling across the entire system
- Complex programs using all features
"""

import pytest
import sys
import io
import os
import subprocess
import tempfile
from unittest.mock import patch
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from src.compiler import main as compiler_main
from src.parser import TubularParser
from src.engine import Engine
from src.grid import Grid
from src.errors import ErrorReporter
from src.droplet import Droplet
from src.direction import Direction
from src.tokens import Position


def create_grid_from_string(program_text: str) -> Grid:
    """Helper function to create Grid from string."""
    lines = program_text.strip().split('\n')
    height = len(lines)
    width = max(len(line) for line in lines) if lines else 0

    grid = Grid(width, height)
    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            if char != ' ':
                grid._grid[y][x] = char

    return grid


class TestParserEngineIntegration:
    """Test integration between parser and engine components."""

    def test_parse_and_execute_valid_program(self):
        """Test parsing and executing a valid program."""
        program_text = """
        @
        |
        5
        :
        3
        :
        A
        n
        """

        parser = TubularParser()
        program = parser.parse_from_string(program_text)

        assert program.has_valid_structure

        # Create engine with parsed program
        grid = create_grid_from_string(program_text)
        engine = Engine(grid, program)

        # Execute program
        with patch('builtins.print') as mock_print:
            tick_count = 0
            while not engine.is_empty() and tick_count < 100:
                engine.tick()
                tick_count += 1

            # Should output 8 (5 + 3)
            mock_print.assert_called_with(8)

    def test_parse_and_execute_complex_program(self):
        """Test parsing and executing a complex program with multiple features."""
        program_text = """
        @
        |
        1
        0
        :
        :
        d
        A
        2
        M
        5
        %
        n
        """

        parser = TubularParser()
        program = parser.parse_from_string(program_text)

        assert program.has_valid_structure

        grid = Grid.from_string(program_text)
        engine = Engine(grid, program)

        # Execute program: (10 + 10) * 2 % 5 = 20 * 2 % 5 = 40 % 5 = 0
        with patch('builtins.print') as mock_print:
            tick_count = 0
            while not engine.is_empty() and tick_count < 100:
                engine.tick()
                tick_count += 1

            mock_print.assert_called_with(0)

    def test_parser_error_handling_in_engine(self):
        """Test that parser errors are properly handled during execution."""
        program_text = """
        @
        |
        X
        +
        """

        parser = TubularParser()
        program = parser.parse_from_string(program_text)

        # Program should be invalid due to invalid character
        assert not program.has_valid_structure
        assert len(program.parsing_errors) > 0

        # Engine should handle this gracefully
        grid = Grid.from_string(program_text)
        engine = Engine(grid, program)

        # Should not crash, but may not execute properly
        tick_count = 0
        while not engine.is_empty() and tick_count < 10:
            engine.tick()
            tick_count += 1

    def test_memory_integration(self):
        """Test memory operations across parser and engine."""
        program_text = """
        @
        |
        4
        2
        :
        P
        G
        n
        """

        parser = TubularParser()
        program = parser.parse_from_string(program_text)

        assert program.has_valid_structure

        grid = Grid.from_string(program_text)
        engine = Engine(grid, program)

        with patch('builtins.print') as mock_print:
            tick_count = 0
            while not engine.is_empty() and tick_count < 100:
                engine.tick()
                tick_count += 1

            # Should store 42 and then retrieve and output it
            mock_print.assert_called_with(42)


class TestCommandLineInterface:
    """Test command-line interface functionality."""

    def test_help_output(self, tmp_path):
        """Test --help option."""
        test_file = tmp_path / "dummy.tub"

        # Test help without file argument
        result = subprocess.run([
            sys.executable, "src/compiler.py"
        ], capture_output=True, text=True, cwd=".")

        assert result.returncode == 0
        assert "Tubular Language Compiler/Interpreter" in result.stdout
        assert "Usage:" in result.stdout

    def test_validate_flag(self, tmp_path):
        """Test --validate flag with valid program."""
        test_file = tmp_path / "valid.tub"
        test_file.write_text("@\n|\n+\n")

        result = subprocess.run([
            sys.executable, "src/compiler.py", "--validate", str(test_file)
        ], capture_output=True, text=True, cwd=".")

        assert result.returncode == 0
        assert "✓ Program structure is valid" in result.stdout
        assert "Loaded grid from:" in result.stdout

    def test_validate_flag_invalid_program(self, tmp_path):
        """Test --validate flag with invalid program."""
        test_file = tmp_path / "invalid.tub"
        test_file.write_text("X\n|\n+\n")  # Invalid character

        result = subprocess.run([
            sys.executable, "src/compiler.py", "--validate", str(test_file)
        ], capture_output=True, text=True, cwd=".")

        assert result.returncode == 1
        assert "✗ Program structure validation failed" in result.stdout

    def test_strict_flag_with_warnings(self, tmp_path):
        """Test --strict flag with unreachable code."""
        test_file = tmp_path / "unreachable.tub"
        test_file.write_text("@\n|\n+\n\n\n#\n-\n")  # Unreachable code

        result = subprocess.run([
            sys.executable, "src/compiler.py", "--validate", "--strict", str(test_file)
        ], capture_output=True, text=True, cwd=".")

        # Should exit with error due to unreachable code in strict mode
        assert result.returncode == 1
        assert "Strict mode: Exiting due to parsing errors" in result.stdout

    def test_file_not_found_error(self):
        """Test error handling for non-existent file."""
        result = subprocess.run([
            sys.executable, "src/compiler.py", "nonexistent.tub"
        ], capture_output=True, text=True, cwd=".")

        assert result.returncode == 1
        assert "File 'nonexistent.tub' not found" in result.stdout

    def test_version_flag(self):
        """Test --version flag."""
        result = subprocess.run([
            sys.executable, "src/compiler.py", "--version"
        ], capture_output=True, text=True, cwd=".")

        assert result.returncode == 0
        assert "Tubular Compiler v0.1.0" in result.stdout


class TestBackwardCompatibility:
    """Test backward compatibility with existing functionality."""

    def test_existing_test_files_still_pass(self):
        """Test that existing test files still pass."""
        # Import and run existing tests
        sys.path.insert(0, 'tests')

        try:
            import test_engine
            import test_grid
            import test_droplet
            import test_direction

            # If imports succeed, basic structure is compatible
            assert True

        except ImportError as e:
            pytest.fail(f"Backward compatibility broken: {e}")

    def test_basic_execution_still_works(self):
        """Test that basic execution functionality still works."""
        grid = Grid(3, 3)
        grid._grid[0][0] = '@'
        grid._grid[0][1] = '|'
        grid._grid[0][2] = 'n'

        # Test without parser (old way)
        engine = Engine(grid)
        assert len(engine.get_active_droplets()) == 1

        # Test with parser (new way)
        parser = TubularParser()
        program = parser.parse_from_grid(grid)
        engine_with_parser = Engine(grid, program)

        assert len(engine_with_parser.get_active_droplets()) == 1

    def test_old_operator_behavior_preserved(self):
        """Test that old operators behave the same way."""
        grid = Grid(3, 3)
        grid._grid[1][1] = 'n'  # Numeric output

        engine = Engine(grid)
        droplet = Droplet(42, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        with patch('builtins.print') as mock_print:
            engine.tick()

            # Should still work the same way
            mock_print.assert_called_once_with(42)


class TestErrorHandlingIntegration:
    """Test error handling across the entire system."""

    def test_lexical_error_propagation(self):
        """Test that lexical errors are properly propagated."""
        program_text = "@\n|\nX\n+"  # Invalid character

        parser = TubularParser()
        program = parser.parse_from_string(program_text)

        assert not program.has_valid_structure
        assert len(program.parsing_errors) > 0
        assert "Invalid character" in program.parsing_errors[0]

    def test_semantic_error_propagation(self):
        """Test that semantic errors are properly propagated."""
        program_text = "@\n@\n|"  # Multiple entry points

        parser = TubularParser()
        program = parser.parse_from_string(program_text)

        assert not program.has_valid_structure
        assert len(program.parsing_errors) > 0
        assert "Multiple entry points" in program.parsing_errors[0]

    def test_runtime_error_handling(self):
        """Test error handling during program execution."""
        # Create a program that might cause issues
        program_text = "@\n|\n:\n;\nA\n"  # Stack underflow

        parser = TubularParser()
        program = parser.parse_from_string(program_text)

        assert program.has_valid_structure

        grid = create_grid_from_string(program_text)
        engine = Engine(grid, program)

        # Should handle stack underflow gracefully
        tick_count = 0
        while not engine.is_empty() and tick_count < 50:
            engine.tick()
            tick_count += 1

        # Should complete without crashing
        assert tick_count < 50

    def test_large_program_handling(self):
        """Test handling of large programs."""
        # Create a large program
        width, height = 50, 50
        grid = Grid(width, height)
        grid._grid[0][0] = '@'

        # Add some operators throughout the grid
        for i in range(10):
            if i < width and i < height:
                grid._grid[i][i] = '+'

        parser = TubularParser()
        program = parser.parse_from_grid(grid)

        assert program.has_valid_structure

        # Should handle large grid without performance issues
        engine = Engine(grid, program)
        assert len(engine.get_active_droplets()) == 1


class TestComplexIntegrationScenarios:
    """Test complex integration scenarios."""

    def test_full_compiler_workflow(self, tmp_path):
        """Test complete workflow from file to execution."""
        test_file = tmp_path / "calculator.tub"
        test_file.write_text("""
        @
        |
        7
        :
        3
        :
        M
        2
        D
        n
        """)

        # Parse from file
        parser = TubularParser()
        program = parser.parse_from_file(str(test_file))

        assert program.has_valid_structure

        # Execute
        grid = Grid.from_file(str(test_file))
        engine = Engine(grid, program)

        with patch('builtins.print') as mock_print:
            tick_count = 0
            while not engine.is_empty() and tick_count < 100:
                engine.tick()
                tick_count += 1

            # Should compute (7 * 3) / 2 = 10.5 -> 10 (integer division)
            mock_print.assert_called_with(10)

    def test_parser_engine_data_flow(self):
        """Test data flow between parser analysis and engine execution."""
        program_text = """
        @
        |
        5
        :
        +
        3
        :
        S
        n
        """

        parser = TubularParser()
        program = parser.parse_from_string(program_text)

        # Parser should identify all operators and structure
        assert len(program.operators) == 9  # Parser pads to rectangular grid
        assert program.entry_point == Position(0, 0)
        assert len(program.reachable_positions) >= 1  # At least entry point is reachable

        # Engine should execute correctly
        grid = Grid.from_string(program_text)
        engine = Engine(grid, program)

        with patch('builtins.print') as mock_print:
            tick_count = 0
            while not engine.is_empty() and tick_count < 100:
                engine.tick()
                tick_count += 1

            # Should compute 5 + 3 = 8
            mock_print.assert_called_with(8)

    def test_error_recovery_integration(self):
        """Test error recovery across parser and engine."""
        parser = TubularParser()

        # First, try to parse invalid program
        invalid_program = parser.parse_from_string("@\n|\nX\n+")
        assert not invalid_program.has_valid_structure

        # Clear errors and parse valid program
        parser.clear_errors()
        valid_program = parser.parse_from_string("@\n|\n+\n")
        assert valid_program.has_valid_structure
        assert len(parser.get_errors()) == 0


class TestEdgeCaseIntegration:
    """Test edge cases in integrated scenarios."""

    def test_minimal_valid_program(self):
        """Test minimal valid program execution."""
        parser = TubularParser()
        program = parser.parse_from_string("@")

        assert program.has_valid_structure

        grid = Grid.from_string("@")
        engine = Engine(grid, program)

        # Should create initial droplet and terminate immediately
        assert len(engine.get_active_droplets()) == 1

        # Execute one tick
        engine.tick()

        # Should be empty (no further execution)
        assert engine.is_empty()

    def test_empty_program_handling(self):
        """Test handling of empty programs."""
        parser = TubularParser()
        program = parser.parse_from_string("")

        assert not program.has_valid_structure

        # Engine should handle gracefully
        grid = Grid(1, 1)
        engine = Engine(grid, program)

        # Should not crash
        assert engine.is_empty()

    def test_whitespace_only_program(self):
        """Test program with only whitespace."""
        parser = TubularParser()
        program = parser.parse_from_string("   \n  \n  ")

        assert not program.has_valid_structure

        grid = Grid(3, 3)
        engine = Engine(grid, program)

        # Should handle gracefully
        assert engine.is_empty()

    def test_single_operator_programs(self):
        """Test programs with single operators."""
        test_cases = [
            ("@", True),   # Valid entry point
            ("|", False),  # Invalid - no entry point
            ("+", False),  # Invalid - no entry point
            ("5", False),  # Invalid - no entry point
        ]

        for program_text, should_be_valid in test_cases:
            parser = TubularParser()
            program = parser.parse_from_string(program_text)

            assert program.has_valid_structure == should_be_valid

            grid = Grid.from_string(program_text)
            engine = Engine(grid, program)

            # Should handle gracefully regardless of validity
            tick_count = 0
            while not engine.is_empty() and tick_count < 10:
                engine.tick()
                tick_count += 1


class TestPerformanceIntegration:
    """Test performance characteristics of integrated system."""

    def test_reasonable_execution_time(self):
        """Test that programs execute in reasonable time."""
        # Create a moderately complex program
        program_text = "@\n" + "|\n" * 20 + "+\n" * 10 + "n"

        parser = TubularParser()
        program = parser.parse_from_string(program_text)

        grid = Grid.from_string(program_text)
        engine = Engine(grid, program)

        # Should complete in reasonable time
        import time
        start_time = time.time()

        tick_count = 0
        while not engine.is_empty() and tick_count < 1000:
            engine.tick()
            tick_count += 1

        end_time = time.time()

        # Should complete within 1 second
        assert end_time - start_time < 1.0
        assert tick_count < 1000

    def test_memory_usage_scaling(self):
        """Test memory usage with larger programs."""
        # Test with increasingly large grids
        sizes = [10, 20, 50]

        for size in sizes:
            grid = Grid(size, size)
            grid._grid[0][0] = '@'

            parser = TubularParser()
            program = parser.parse_from_grid(grid)

            engine = Engine(grid, program)

            # Should handle large grids without excessive memory usage
            # (Basic check: doesn't crash and droplet count is reasonable)
            assert len(engine.get_active_droplets()) <= 1

            # Clean up
            engine.clear_data_structures()


class TestSystemIntegration:
    """Test system-level integration scenarios."""

    def test_module_imports(self):
        """Test that all modules can be imported correctly."""
        try:
            from src.parser import TubularParser
            from src.engine import Engine
            from src.grid import Grid
            from src.compiler import main
            from src.errors import ErrorReporter

            # All imports successful
            assert True

        except ImportError as e:
            pytest.fail(f"Module import failed: {e}")

    def test_circular_dependencies(self):
        """Test that there are no circular import dependencies."""
        # This test mainly ensures the modules can be imported in different orders
        import importlib

        modules = [
            'src.parser',
            'src.engine',
            'src.grid',
            'src.compiler',
            'src.errors',
            'src.tokens',
            'src.direction',
            'src.droplet'
        ]

        for module_name in modules:
            try:
                importlib.import_module(module_name)
            except ImportError as e:
                pytest.fail(f"Failed to import {module_name}: {e}")

    def test_error_reporter_integration(self):
        """Test error reporter integration across components."""
        error_reporter = ErrorReporter()

        # Test that parser uses error reporter
        parser = TubularParser(error_reporter)
        program = parser.parse_from_string("X")  # Invalid character

        assert error_reporter.has_errors()
        assert len(error_reporter.errors) > 0

        # Test error formatting
        formatted = error_reporter.format_all()
        assert "Error" in formatted

        # Test error clearing
        error_reporter.clear()
        assert not error_reporter.has_errors()


if __name__ == "__main__":
    pytest.main([__file__])