#!/usr/bin/env python3
"""
Grid class for Tubular programming language.

Represents a 2D grid of characters loaded from a source file.
"""

from typing import List, Tuple, Optional


class Grid:
    """
    A 2D grid of characters representing a Tubular program.

    The grid is loaded from a source file and provides methods to access
    characters at specific coordinates with proper bounds checking.
    """

    def __init__(self, width: int, height: int):
        """
        Initialize an empty grid with given dimensions.

        Args:
            width: The width of the grid (number of columns)
            height: The height of the grid (number of rows)
        """
        self.width = width
        self.height = height
        self._grid: List[List[str]] = []

    @classmethod
    def from_file(cls, filepath: str) -> 'Grid':
        """
        Load a grid from a source file.

        Args:
            filepath: Path to the source file containing the grid data

        Returns:
            A new Grid instance loaded from the file

        Raises:
            FileNotFoundError: If the file doesn't exist
            ValueError: If the file format is invalid
        """
        with open(filepath, 'r', encoding='utf-8') as file:
            lines = file.readlines()

        # Remove trailing whitespace and normalize line endings
        lines = [line.rstrip('\r\n') for line in lines]

        if not lines:
            raise ValueError("File is empty")

        # Find the maximum width
        height = len(lines)
        width = max(len(line) for line in lines) if lines else 0

        # Create grid instance
        grid = cls(width, height)

        # Fill the grid, padding shorter lines with spaces
        for y, line in enumerate(lines):
            if y >= height:
                break
            row = list(line) + [' '] * (width - len(line))
            grid._grid.append(row)

        return grid

    def get(self, x: int, y: int) -> str:
        """
        Get the character at the specified coordinates.

        Args:
            x: The x-coordinate (column)
            y: The y-coordinate (row)

        Returns:
            The character at (x, y), or ' ' if out of bounds
        """
        if not self._is_valid_position(x, y):
            return ' '
        return self._grid[y][x]

    def _is_valid_position(self, x: int, y: int) -> bool:
        """
        Check if the given coordinates are within grid bounds.

        Args:
            x: The x-coordinate (column)
            y: The y-coordinate (row)

        Returns:
            True if coordinates are valid, False otherwise
        """
        return 0 <= x < self.width and 0 <= y < self.height

    def get_dimensions(self) -> Tuple[int, int]:
        """
        Get the dimensions of the grid.

        Returns:
            A tuple (width, height) of the grid dimensions
        """
        return (self.width, self.height)

    def is_empty(self) -> bool:
        """
        Check if the grid is empty.

        Returns:
            True if the grid has no content, False otherwise
        """
        return self.width == 0 or self.height == 0

    def __str__(self) -> str:
        """Return string representation of the grid."""
        if not self._grid:
            return "Empty grid"

        lines = []
        for row in self._grid:
            lines.append(''.join(row))
        return '\n'.join(lines)

    def __repr__(self) -> str:
        """Return detailed string representation of the grid."""
        return f"Grid(width={self.width}, height={self.height})"