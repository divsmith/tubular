#!/usr/bin/env python3
"""
Unit tests for Grid class in Tubular programming language.
"""

import os
import tempfile
import pytest
from src.grid import Grid


class TestGrid:
    """Test cases for Grid class functionality."""

    def test_grid_constructor(self):
        """Test Grid constructor with valid dimensions."""
        grid = Grid(5, 3)
        assert grid.width == 5
        assert grid.height == 3
        assert grid.get_dimensions() == (5, 3)
        assert not grid.is_empty()

    def test_grid_constructor_zero_dimensions(self):
        """Test Grid constructor with zero dimensions."""
        grid = Grid(0, 0)
        assert grid.width == 0
        assert grid.height == 0
        assert grid.get_dimensions() == (0, 0)
        assert grid.is_empty()

    def test_grid_constructor_negative_dimensions(self):
        """Test Grid constructor with negative dimensions."""
        grid = Grid(-1, -1)
        assert grid.width == -1
        assert grid.height == -1
        # Note: This creates an invalid grid, but tests the constructor behavior

    def test_grid_string_representation(self):
        """Test string representation of grid."""
        grid = Grid(3, 2)
        grid_str = str(grid)
        assert "Empty grid" in grid_str

        # Test repr
        grid_repr = repr(grid)
        assert "Grid(width=3, height=2)" == grid_repr

    def test_from_file_empty_file(self):
        """Test loading from an empty file raises ValueError."""
        with tempfile.NamedTemporaryFile(mode='w', delete=False) as f:
            f.write("")
            temp_path = f.name

        try:
            with pytest.raises(ValueError, match="File is empty"):
                Grid.from_file(temp_path)
        finally:
            os.unlink(temp_path)

    def test_from_file_nonexistent(self):
        """Test loading from a nonexistent file raises FileNotFoundError."""
        with pytest.raises(FileNotFoundError):
            Grid.from_file("nonexistent_file.txt")

    def test_from_file_simple_content(self):
        """Test loading a simple text file into Grid."""
        content = "ABC\nDEF"

        with tempfile.NamedTemporaryFile(mode='w', delete=False) as f:
            f.write(content)
            temp_path = f.name

        try:
            grid = Grid.from_file(temp_path)
            assert grid.width == 3
            assert grid.height == 2
            assert grid.get_dimensions() == (3, 2)

            # Test content
            assert grid.get(0, 0) == 'A'
            assert grid.get(1, 0) == 'B'
            assert grid.get(2, 0) == 'C'
            assert grid.get(0, 1) == 'D'
            assert grid.get(1, 1) == 'E'
            assert grid.get(2, 1) == 'F'
        finally:
            os.unlink(temp_path)

    def test_from_file_padded_content(self):
        """Test loading file with uneven line lengths (padding with spaces)."""
        content = "AB\nCDE\nF"

        with tempfile.NamedTemporaryFile(mode='w', delete=False) as f:
            f.write(content)
            temp_path = f.name

        try:
            grid = Grid.from_file(temp_path)
            assert grid.width == 3
            assert grid.height == 3

            # Test content with padding
            assert grid.get(0, 0) == 'A'
            assert grid.get(1, 0) == 'B'
            assert grid.get(2, 0) == ' '

            assert grid.get(0, 1) == 'C'
            assert grid.get(1, 1) == 'D'
            assert grid.get(2, 1) == 'E'

            assert grid.get(0, 2) == 'F'
            assert grid.get(1, 2) == ' '
            assert grid.get(2, 2) == ' '
        finally:
            os.unlink(temp_path)

    def test_from_file_with_line_endings(self):
        """Test loading file with different line endings."""
        content = "ABC\r\nDEF\r\nGHI\n"

        with tempfile.NamedTemporaryFile(mode='w', delete=False) as f:
            f.write(content)
            temp_path = f.name

        try:
            grid = Grid.from_file(temp_path)
            assert grid.width == 3
            assert grid.height == 3

            assert grid.get(0, 0) == 'A'
            assert grid.get(1, 0) == 'B'
            assert grid.get(2, 0) == 'C'
            assert grid.get(0, 1) == 'D'
            assert grid.get(1, 1) == 'E'
            assert grid.get(2, 1) == 'F'
            assert grid.get(0, 2) == 'G'
            assert grid.get(1, 2) == 'H'
            assert grid.get(2, 2) == 'I'
        finally:
            os.unlink(temp_path)

    def test_get_valid_coordinates(self):
        """Test get() method with valid coordinates."""
        content = "ABC\nDEF"

        with tempfile.NamedTemporaryFile(mode='w', delete=False) as f:
            f.write(content)
            temp_path = f.name

        try:
            grid = Grid.from_file(temp_path)

            # Test all valid positions
            assert grid.get(0, 0) == 'A'
            assert grid.get(1, 0) == 'B'
            assert grid.get(2, 0) == 'C'
            assert grid.get(0, 1) == 'D'
            assert grid.get(1, 1) == 'E'
            assert grid.get(2, 1) == 'F'
        finally:
            os.unlink(temp_path)

    def test_get_out_of_bounds_coordinates(self):
        """Test get() method with out-of-bounds coordinates returns space."""
        content = "ABC\nDEF"

        with tempfile.NamedTemporaryFile(mode='w', delete=False) as f:
            f.write(content)
            temp_path = f.name

        try:
            grid = Grid.from_file(temp_path)

            # Test out of bounds positions
            assert grid.get(-1, 0) == ' '  # Left of grid
            assert grid.get(0, -1) == ' '  # Above grid
            assert grid.get(3, 0) == ' '   # Right of grid
            assert grid.get(0, 2) == ' '   # Below grid
            assert grid.get(-1, -1) == ' ' # Diagonal out of bounds
            assert grid.get(10, 10) == ' ' # Far out of bounds
        finally:
            os.unlink(temp_path)

    def test_get_edge_coordinates(self):
        """Test get() method at grid edges."""
        content = "ABC\nDEF"

        with tempfile.NamedTemporaryFile(mode='w', delete=False) as f:
            f.write(content)
            temp_path = f.name

        try:
            grid = Grid.from_file(temp_path)

            # Test edge positions (should be valid)
            assert grid.get(0, 0) == 'A'  # Top-left corner
            assert grid.get(2, 0) == 'C'  # Top-right corner
            assert grid.get(0, 1) == 'D'  # Bottom-left corner
            assert grid.get(2, 1) == 'F'  # Bottom-right corner
        finally:
            os.unlink(temp_path)

    def test_is_valid_position_internal_method(self):
        """Test the internal _is_valid_position method behavior."""
        grid = Grid(3, 2)

        # Valid positions
        assert grid._is_valid_position(0, 0) == True
        assert grid._is_valid_position(2, 1) == True
        assert grid._is_valid_position(1, 1) == True

        # Invalid positions
        assert grid._is_valid_position(-1, 0) == False
        assert grid._is_valid_position(0, -1) == False
        assert grid._is_valid_position(3, 0) == False
        assert grid._is_valid_position(0, 2) == False
        assert grid._is_valid_position(-1, -1) == False

    def test_is_empty_method(self):
        """Test the is_empty method."""
        # Empty grid
        empty_grid = Grid(0, 0)
        assert empty_grid.is_empty() == True

        # Grid with zero width
        zero_width = Grid(0, 5)
        assert zero_width.is_empty() == True

        # Grid with zero height
        zero_height = Grid(5, 0)
        assert zero_height.is_empty() == True

        # Non-empty grid
        normal_grid = Grid(3, 2)
        assert normal_grid.is_empty() == False

    def test_grid_with_special_characters(self):
        """Test grid with special characters and whitespace."""
        content = "!@#\n$%^"

        with tempfile.NamedTemporaryFile(mode='w', delete=False) as f:
            f.write(content)
            temp_path = f.name

        try:
            grid = Grid.from_file(temp_path)

            assert grid.get(0, 0) == '!'
            assert grid.get(1, 0) == '@'
            assert grid.get(2, 0) == '#'
            assert grid.get(0, 1) == '$'
            assert grid.get(1, 1) == '%'
            assert grid.get(2, 1) == '^'
        finally:
            os.unlink(temp_path)

    def test_grid_single_character(self):
        """Test grid with single character."""
        content = "X"

        with tempfile.NamedTemporaryFile(mode='w', delete=False) as f:
            f.write(content)
            temp_path = f.name

        try:
            grid = Grid.from_file(temp_path)

            assert grid.width == 1
            assert grid.height == 1
            assert grid.get(0, 0) == 'X'
            assert grid.get(1, 0) == ' '  # Out of bounds
            assert grid.get(0, 1) == ' '  # Out of bounds
        finally:
            os.unlink(temp_path)

    def test_grid_with_empty_lines(self):
        """Test grid with empty lines."""
        content = "ABC\n\nDEF"

        with tempfile.NamedTemporaryFile(mode='w', delete=False) as f:
            f.write(content)
            temp_path = f.name

        try:
            grid = Grid.from_file(temp_path)

            assert grid.width == 3
            assert grid.height == 3

            # Empty line should be treated as line with spaces
            assert grid.get(0, 0) == 'A'
            assert grid.get(1, 0) == 'B'
            assert grid.get(2, 0) == 'C'

            assert grid.get(0, 1) == ' '
            assert grid.get(1, 1) == ' '
            assert grid.get(2, 1) == ' '

            assert grid.get(0, 2) == 'D'
            assert grid.get(1, 2) == 'E'
            assert grid.get(2, 2) == 'F'
        finally:
            os.unlink(temp_path)