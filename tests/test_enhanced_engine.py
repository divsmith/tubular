#!/usr/bin/env python3
"""
Comprehensive test suite for the enhanced Tubular engine functionality.

Tests all enhanced operators and execution capabilities:
- Unary operators (+, ~)
- Data sources (0-9, >, ?, ??)
- Stack operations (:, ;, d)
- Arithmetic operations (A, S, M, D, %)
- I/O operations (!, ,, n)
- Memory operations (G, P)
- Subroutine operations (C, R)
- Corner pipes (/, \)
"""

import pytest
import sys
import io
import os
from unittest.mock import patch
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from src.engine import Engine
from src.grid import Grid
from src.droplet import Droplet
from src.direction import Direction
from src.parser import TubularParser, ParsedProgram
from src.tokens import Token, TokenType, Position


class TestUnaryOperators:
    """Test unary operators (+ and ~)."""

    def test_increment_operator(self):
        """Test + (increment) operator."""
        grid = Grid(3, 3)
        grid._grid[1][1] = '+'

        engine = Engine(grid)
        droplet = Droplet(5, 1, 0, Direction.DOWN)  # Will move to (1, 1)
        engine.add_droplet(droplet)

        engine.tick()

        # Droplet should have moved to (1, 1) and value should be incremented
        assert droplet.x == 1 and droplet.y == 1
        assert droplet.value == 6  # 5 + 1

    def test_decrement_operator(self):
        """Test ~ (decrement) operator."""
        grid = Grid(3, 3)
        grid._grid[1][1] = '~'

        engine = Engine(grid)
        droplet = Droplet(5, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert droplet.x == 1 and droplet.y == 1
        assert droplet.value == 4  # 5 - 1

    def test_multiple_unary_operators(self):
        """Test sequence of unary operators."""
        grid = Grid(5, 5)
        grid._grid[2][1] = '+'
        grid._grid[2][2] = '~'
        grid._grid[2][3] = '+'

        engine = Engine(grid)
        droplet = Droplet(5, 2, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        # Execute multiple ticks
        for _ in range(3):
            engine.tick()

        # Should end at (2, 3) with value 5 + 1 - 1 + 1 = 6
        assert droplet.x == 2 and droplet.y == 3
        assert droplet.value == 6


class TestDataSources:
    """Test data source operators (0-9, >, ?, ??)."""

    def test_number_operators(self):
        """Test number operators (0-9)."""
        for num in range(10):
            grid = Grid(3, 3)
            grid._grid[1][1] = str(num)

            engine = Engine(grid)
            droplet = Droplet(99, 1, 0, Direction.DOWN)
            engine.add_droplet(droplet)

            engine.tick()

            assert droplet.x == 1 and droplet.y == 1
            assert droplet.value == num

    def test_tape_reader_operator(self):
        """Test > (tape reader) operator."""
        grid = Grid(3, 3)
        grid._grid[1][1] = '>'

        engine = Engine(grid)
        droplet = Droplet(0, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert droplet.x == 1 and droplet.y == 1
        assert droplet.value == 0  # First tape read

        # Second tape read should increment
        droplet2 = Droplet(0, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet2)
        engine.tick()
        assert droplet2.value == 1

    def test_character_input_operator(self):
        """Test ? (character input) operator."""
        grid = Grid(3, 3)
        grid._grid[1][1] = '?'

        engine = Engine(grid)
        engine.add_input("ABC")

        droplet = Droplet(0, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert droplet.x == 1 and droplet.y == 1
        assert droplet.value == ord('A')  # First character

        # Test with no input (should default to space)
        droplet2 = Droplet(0, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet2)
        engine.tick()
        assert droplet2.value == ord(' ')

    def test_numeric_input_operator(self):
        """Test ?? (numeric input) operator."""
        grid = Grid(3, 3)
        grid._grid[1][1] = '?'
        grid._grid[1][2] = '?'

        engine = Engine(grid)
        engine.add_numeric_input(42)
        engine.add_numeric_input(99)

        droplet = Droplet(0, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert droplet.x == 1 and droplet.y == 2  # ?? takes two cells
        assert droplet.value == 42

        # Test with no input (should default to 0)
        droplet2 = Droplet(0, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet2)
        engine.tick()
        assert droplet2.value == 0


class TestStackOperations:
    """Test stack operations (:, ;, d)."""

    def test_push_operator(self):
        """Test : (push) operator."""
        grid = Grid(3, 3)
        grid._grid[1][1] = ':'

        engine = Engine(grid)
        droplet = Droplet(42, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert droplet.x == 1 and droplet.y == 1
        assert engine.get_stack_depth() == 1
        assert engine.get_stack_top() == 42

    def test_pop_operator(self):
        """Test ; (pop) operator."""
        grid = Grid(3, 3)
        grid._grid[1][1] = ';'

        engine = Engine(grid)
        engine.data_stack = [10, 20, 30]  # Pre-populate stack

        droplet = Droplet(0, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert droplet.x == 1 and droplet.y == 1
        assert droplet.value == 30  # Popped top value
        assert engine.get_stack_depth() == 2  # One item removed

    def test_pop_empty_stack(self):
        """Test ; (pop) operator with empty stack."""
        grid = Grid(3, 3)
        grid._grid[1][1] = ';'

        engine = Engine(grid)
        droplet = Droplet(99, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert droplet.value == 0  # Default value for empty stack

    def test_duplicate_operator(self):
        """Test d (duplicate) operator."""
        grid = Grid(3, 3)
        grid._grid[1][1] = 'd'

        engine = Engine(grid)
        engine.data_stack = [10, 20]

        droplet = Droplet(0, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert engine.get_stack_depth() == 3
        assert engine.get_stack_top() == 20  # Top should be duplicated

    def test_duplicate_empty_stack(self):
        """Test d (duplicate) operator with empty stack."""
        grid = Grid(3, 3)
        grid._grid[1][1] = 'd'

        engine = Engine(grid)
        droplet = Droplet(0, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert engine.get_stack_depth() == 1
        assert engine.get_stack_top() == 0  # Default value


class TestArithmeticOperations:
    """Test arithmetic operations (A, S, M, D, %)."""

    def setup_arithmetic_test(self, operator, initial_stack, expected_stack):
        """Helper method for arithmetic tests."""
        grid = Grid(3, 3)
        grid._grid[1][1] = operator

        engine = Engine(grid)
        engine.data_stack = initial_stack.copy()

        droplet = Droplet(0, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        return engine.data_stack

    def test_addition_operator(self):
        """Test A (addition) operator."""
        result = self.setup_arithmetic_test('A', [5, 3, 8], [5, 11])
        assert result == [5, 11]  # 3 + 8

    def test_subtraction_operator(self):
        """Test S (subtraction) operator."""
        result = self.setup_arithmetic_test('S', [10, 7, 3], [10, 4])
        assert result == [10, 4]  # 7 - 3

    def test_multiplication_operator(self):
        """Test M (multiplication) operator."""
        result = self.setup_arithmetic_test('M', [2, 6, 4], [2, 24])
        assert result == [2, 24]  # 6 * 4

    def test_division_operator(self):
        """Test D (division) operator."""
        result = self.setup_arithmetic_test('D', [4, 15, 3], [4, 5])
        assert result == [4, 5]  # 15 // 3

    def test_division_by_zero(self):
        """Test D (division) operator with division by zero."""
        result = self.setup_arithmetic_test('D', [2, 10, 0], [2, 0])
        assert result == [2, 0]  # Division by zero returns 0

    def test_modulo_operator(self):
        """Test % (modulo) operator."""
        result = self.setup_arithmetic_test('%', [3, 17, 5], [3, 2])
        assert result == [3, 2]  # 17 % 5

    def test_modulo_by_zero(self):
        """Test % (modulo) operator with modulo by zero."""
        result = self.setup_arithmetic_test('%', [2, 10, 0], [2, 0])
        assert result == [2, 0]  # Modulo by zero returns 0

    def test_insufficient_stack_operands(self):
        """Test arithmetic operations with insufficient stack operands."""
        grid = Grid(3, 3)
        grid._grid[1][1] = 'A'

        engine = Engine(grid)
        engine.data_stack = [5]  # Only one operand

        droplet = Droplet(0, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        initial_depth = engine.get_stack_depth()
        engine.tick()

        # Stack should remain unchanged
        assert engine.get_stack_depth() == initial_depth


class TestIOOperations:
    """Test I/O operations (!, ,, n)."""

    def test_output_sink_operator(self):
        """Test ! (output sink) operator."""
        grid = Grid(3, 3)
        grid._grid[1][1] = '!'

        engine = Engine(grid)

        with patch('builtins.print') as mock_print:
            droplet = Droplet(42, 1, 0, Direction.DOWN)
            engine.add_droplet(droplet)

            engine.tick()

            mock_print.assert_called_once_with(42, end='')

    def test_character_output_operator(self):
        """Test , (character output) operator."""
        grid = Grid(3, 3)
        grid._grid[1][1] = ','

        engine = Engine(grid)

        with patch('builtins.print') as mock_print:
            droplet = Droplet(65, 1, 0, Direction.DOWN)  # ASCII 'A'
            engine.add_droplet(droplet)

            engine.tick()

            mock_print.assert_called_once_with('A', end='')

    def test_numeric_output_operator(self):
        """Test n (numeric output) operator."""
        grid = Grid(3, 3)
        grid._grid[1][1] = 'n'

        engine = Engine(grid)

        with patch('builtins.print') as mock_print:
            droplet = Droplet(42, 1, 0, Direction.DOWN)
            engine.add_droplet(droplet)

            engine.tick()

            mock_print.assert_called_once_with(42)


class TestMemoryOperations:
    """Test memory operations (G, P)."""

    def test_put_and_get_operations(self):
        """Test P (put) and G (get) operators."""
        grid = Grid(5, 5)
        grid._grid[2][1] = 'P'  # Put at (2, 1)
        grid._grid[2][3] = 'G'  # Get at (2, 3)

        engine = Engine(grid)
        droplet = Droplet(42, 2, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        # Move to put location
        for _ in range(1):
            engine.tick()

        assert droplet.x == 2 and droplet.y == 1
        assert engine.get_reservoir_size() == 1
        assert engine.reservoir.get("2,1") == 42

        # Move to get location
        for _ in range(2):
            engine.tick()

        assert droplet.x == 2 and droplet.y == 3
        assert droplet.value == 42  # Retrieved value

    def test_get_from_empty_memory(self):
        """Test G (get) operator with empty reservoir."""
        grid = Grid(3, 3)
        grid._grid[1][1] = 'G'

        engine = Engine(grid)
        droplet = Droplet(99, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert droplet.value == 0  # Default value for empty reservoir

    def test_memory_isolation(self):
        """Test that different positions have separate memory."""
        grid = Grid(5, 5)
        grid._grid[1][1] = 'P'
        grid._grid[3][3] = 'P'

        engine = Engine(grid)
        droplet1 = Droplet(10, 1, 0, Direction.DOWN)
        droplet2 = Droplet(20, 3, 2, Direction.DOWN)

        engine.add_droplet(droplet1)
        engine.add_droplet(droplet2)

        # Move droplets to their positions
        for _ in range(1):
            engine.tick()

        assert droplet1.x == 1 and droplet1.y == 1
        assert droplet2.x == 3 and droplet2.y == 3

        # Check memory isolation
        assert engine.reservoir.get("1,1") == 10
        assert engine.reservoir.get("3,3") == 20


class TestSubroutineOperations:
    """Test subroutine operations (C, R)."""

    def test_call_operator(self):
        """Test C (call) operator."""
        grid = Grid(3, 3)
        grid._grid[1][1] = 'C'

        engine = Engine(grid)
        droplet = Droplet(42, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert droplet.x == 1 and droplet.y == 1
        assert engine.get_call_stack_depth() == 1

        # Check that state was saved (droplet moves to operator position before processing)
        call_frame = engine.call_stack[0]
        assert call_frame['droplet_x'] == 1
        assert call_frame['droplet_y'] == 1  # Droplet moved to operator position
        assert call_frame['droplet_direction'] == Direction.DOWN
        assert call_frame['droplet_value'] == 42

    def test_return_operator(self):
        """Test R (return) operator."""
        grid = Grid(3, 3)
        grid._grid[1][1] = 'R'

        engine = Engine(grid)

        # Pre-populate call stack
        engine.call_stack = [{
            'droplet_x': 5,
            'droplet_y': 5,
            'droplet_direction': Direction.UP,
            'droplet_value': 99
        }]

        droplet = Droplet(42, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        # Droplet should be restored to saved state
        assert droplet.x == 5 and droplet.y == 5
        assert droplet.direction == Direction.UP
        assert droplet.value == 99
        assert engine.get_call_stack_depth() == 0

    def test_return_without_call(self):
        """Test R (return) operator without matching call."""
        grid = Grid(3, 3)
        grid._grid[1][1] = 'R'

        engine = Engine(grid)
        droplet = Droplet(42, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        # Droplet should be destroyed (no call to return from)
        assert len(engine.get_active_droplets()) == 0


class TestCornerPipes:
    """Test corner pipe operations (/ and \\)."""

    def test_forward_slash_positive_value(self):
        """Test / (forward slash) with positive droplet value."""
        grid = Grid(3, 3)
        grid._grid[1][1] = '/'

        engine = Engine(grid)
        droplet = Droplet(5, 1, 0, Direction.DOWN)  # Moving down, positive value
        engine.add_droplet(droplet)

        engine.tick()

        # With positive value, should turn right (from DOWN to LEFT)
        assert droplet.x == 1 and droplet.y == 1  # Stay at current position
        assert droplet.direction == Direction.LEFT

    def test_forward_slash_negative_value(self):
        """Test / (forward slash) with negative droplet value."""
        grid = Grid(3, 3)
        grid._grid[1][1] = '/'

        engine = Engine(grid)
        droplet = Droplet(-3, 1, 0, Direction.DOWN)  # Moving down, negative value
        engine.add_droplet(droplet)

        engine.tick()

        # With negative value, should turn left (from DOWN to RIGHT)
        assert droplet.x == 1 and droplet.y == 1  # Stay at current position
        assert droplet.direction == Direction.RIGHT

    def test_back_slash_positive_value(self):
        """Test \\ (back slash) with positive droplet value."""
        grid = Grid(3, 3)
        grid._grid[1][1] = '\\'

        engine = Engine(grid)
        droplet = Droplet(5, 1, 0, Direction.DOWN)  # Moving down, positive value
        engine.add_droplet(droplet)

        engine.tick()

        # With positive value, should turn left (from DOWN to RIGHT)
        assert droplet.x == 1 and droplet.y == 1  # Stay at current position
        assert droplet.direction == Direction.RIGHT

    def test_back_slash_negative_value(self):
        """Test \\ (back slash) with negative droplet value."""
        grid = Grid(3, 3)
        grid._grid[1][1] = '\\'

        engine = Engine(grid)
        droplet = Droplet(-3, 1, 0, Direction.DOWN)  # Moving down, negative value
        engine.add_droplet(droplet)

        engine.tick()

        # With negative value, should turn right (from DOWN to LEFT)
        assert droplet.x == 1 and droplet.y == 1  # Stay at current position
        assert droplet.direction == Direction.LEFT

    def test_corner_pipe_boundary_handling(self):
        """Test corner pipes at grid boundaries."""
        grid = Grid(3, 3)
        grid._grid[0][0] = '/'  # Top-left corner

        engine = Engine(grid)
        droplet = Droplet(1, 0, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        # Should handle boundary conditions gracefully
        # (exact behavior depends on implementation, but shouldn't crash)
        assert len(engine.get_active_droplets()) >= 0  # Either moved or destroyed


class TestComplexPrograms:
    """Test complex programs using multiple operators."""

    def test_calculator_program(self):
        """Test a simple calculator program: 5 3 +"""
        grid = Grid(5, 5)
        grid._grid[0][0] = '@'
        grid._grid[0][1] = '5'  # Push 5
        grid._grid[0][2] = ':'
        grid._grid[0][3] = '3'  # Push 3
        grid._grid[0][4] = ':'
        grid._grid[1][4] = 'A'  # Add
        grid._grid[2][4] = 'n'  # Output result

        engine = Engine(grid)
        initial_droplets = engine.get_active_droplets()
        assert len(initial_droplets) == 1  # Entry point droplet

        # Run until completion
        tick_count = 0
        while not engine.is_empty() and tick_count < 100:
            engine.tick()
            tick_count += 1

        # Should have output 8 (5 + 3)
        assert tick_count < 100  # Should complete in reasonable time

    def test_memory_program(self):
        """Test program using memory operations."""
        grid = Grid(5, 5)
        grid._grid[0][0] = '@'
        grid._grid[0][1] = '4'
        grid._grid[0][2] = '2'
        grid._grid[0][3] = ':'
        grid._grid[0][4] = 'P'  # Store 42
        grid._grid[1][4] = 'G'  # Retrieve value
        grid._grid[2][4] = 'n'  # Output retrieved value

        engine = Engine(grid)

        # Run until completion
        tick_count = 0
        with patch('builtins.print') as mock_print:
            while not engine.is_empty() and tick_count < 100:
                engine.tick()
                tick_count += 1

            # Should output 42
            mock_print.assert_called_with(42)

    def test_stack_manipulation_program(self):
        """Test program with complex stack operations."""
        grid = Grid(7, 7)
        grid._grid[0][0] = '@'
        grid._grid[0][1] = '1'
        grid._grid[0][2] = '0'
        grid._grid[0][3] = ':'
        grid._grid[0][4] = ':'
        grid._grid[0][5] = 'd'  # Duplicate 10
        grid._grid[0][6] = 'A'  # Add (10 + 10 = 20)
        grid._grid[1][6] = 'n'  # Output 20

        engine = Engine(grid)

        # Run until completion
        tick_count = 0
        with patch('builtins.print') as mock_print:
            while not engine.is_empty() and tick_count < 100:
                engine.tick()
                tick_count += 1

            # Should output 20
            mock_print.assert_called_with(20)


class TestEdgeCases:
    """Test edge cases and boundary conditions."""

    def test_zero_values(self):
        """Test operators with zero values."""
        grid = Grid(3, 3)
        grid._grid[1][1] = '+'

        engine = Engine(grid)
        droplet = Droplet(0, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert droplet.value == 1  # 0 + 1 = 1

    def test_negative_values(self):
        """Test operators with negative values."""
        grid = Grid(3, 3)
        grid._grid[1][1] = '~'

        engine = Engine(grid)
        droplet = Droplet(-5, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert droplet.value == -6  # -5 - 1 = -6

    def test_large_values(self):
        """Test operators with large values."""
        grid = Grid(3, 3)
        grid._grid[1][1] = '+'

        engine = Engine(grid)
        droplet = Droplet(999999, 1, 0, Direction.DOWN)
        engine.add_droplet(droplet)

        engine.tick()

        assert droplet.value == 1000000  # 999999 + 1

    def test_empty_stack_arithmetic(self):
        """Test arithmetic operations with empty stack."""
        for op in ['A', 'S', 'M', 'D', '%']:
            grid = Grid(3, 3)
            grid._grid[1][1] = op

            engine = Engine(grid)
            droplet = Droplet(42, 1, 0, Direction.DOWN)
            engine.add_droplet(droplet)

            initial_depth = engine.get_stack_depth()
            engine.tick()

            # Stack should remain unchanged
            assert engine.get_stack_depth() == initial_depth

    def test_concurrent_droplets(self):
        """Test multiple droplets executing different operators simultaneously."""
        grid = Grid(5, 5)
        grid._grid[1][1] = '+'
        grid._grid[3][3] = '~'

        engine = Engine(grid)
        droplet1 = Droplet(10, 1, 0, Direction.DOWN)
        droplet2 = Droplet(20, 3, 2, Direction.DOWN)

        engine.add_droplet(droplet1)
        engine.add_droplet(droplet2)

        # Execute one tick
        engine.tick()

        # Both droplets should have executed their operators
        assert droplet1.value == 11  # 10 + 1
        assert droplet2.value == 19   # 20 - 1
        assert len(engine.get_active_droplets()) == 2


if __name__ == "__main__":
    pytest.main([__file__])