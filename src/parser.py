#!/usr/bin/env python3
"""
Parser/Loader component for reading .tub files and creating Grid objects.

This module provides a dedicated parser class for handling Tubular program files.
"""

import os
from typing import List, Tuple
try:
    from .grid import Grid
except ImportError:
    from grid import Grid


class TubFileParser:
    """
    A parser for reading .tub files and creating Grid objects from them.

    This class handles the file I/O and parsing logic separately from the Grid class,
    providing better separation of concerns and more flexibility for file processing.
    """

    def __init__(self):
        """Initialize the parser."""
        pass

    def parse_file(self, filepath: str) -> Grid:
        """
        Parse a .tub file and create a Grid object from its contents.

        Args:
            filepath: Path to the .tub file to parse

        Returns:
            A Grid object populated with the file contents

        Raises:
            FileNotFoundError: If the file doesn't exist
            IOError: If there's an error reading the file
            ValueError: If the file format is invalid (empty, etc.)
        """
        # Validate file exists
        if not os.path.exists(filepath):
            raise FileNotFoundError(f"File not found: {filepath}")

        # Validate file extension
        if not filepath.lower().endswith('.tub'):
            raise ValueError(f"Invalid file type. Expected .tub file, got: {filepath}")

        try:
            with open(filepath, 'r', encoding='utf-8') as file:
                lines = file.readlines()
        except IOError as e:
            raise IOError(f"Error reading file {filepath}: {e}")

        # Process the lines
        processed_lines = self._process_lines(lines)

        # Validate we have content
        if not processed_lines:
            raise ValueError(f"File is empty or contains no valid content: {filepath}")

        # Calculate dimensions
        height = len(processed_lines)
        width = max(len(line) for line in processed_lines) if processed_lines else 0

        if width == 0 or height == 0:
            raise ValueError(f"Invalid grid dimensions (width={width}, height={height}) in file: {filepath}")

        # Create and populate grid
        grid = Grid(width, height)
        self._populate_grid(grid, processed_lines)

        return grid

    def _process_lines(self, lines: List[str]) -> List[str]:
        """
        Process raw lines from file, removing line endings and filtering empty lines.

        Args:
            lines: Raw lines read from file

        Returns:
            List of processed lines with normalized endings
        """
        processed_lines = []

        for line in lines:
            # Remove trailing whitespace and normalize line endings
            cleaned_line = line.rstrip('\r\n').rstrip()
            # Only add non-empty lines
            if cleaned_line.strip():
                processed_lines.append(cleaned_line)

        return processed_lines

    def _populate_grid(self, grid: Grid, lines: List[str]) -> None:
        """
        Populate a Grid object with content from parsed lines.

        Args:
            grid: The Grid object to populate
            lines: The processed lines to add to the grid
        """
        width = grid.width

        for y, line in enumerate(lines):
            if y >= grid.height:
                break

            # Pad line to grid width if necessary
            padded_line = line.ljust(width)
            row = list(padded_line)

            # Set each character in the grid
            for x, char in enumerate(row):
                if x < grid.width:
                    grid._grid[y][x] = char

    def get_file_info(self, filepath: str) -> Tuple[int, int, int]:
        """
        Get information about a .tub file without creating a Grid object.

        Args:
            filepath: Path to the .tub file

        Returns:
            A tuple (width, height, line_count) with file dimensions

        Raises:
            Same exceptions as parse_file()
        """
        # Validate file exists
        if not os.path.exists(filepath):
            raise FileNotFoundError(f"File not found: {filepath}")

        try:
            with open(filepath, 'r', encoding='utf-8') as file:
                lines = file.readlines()
        except IOError as e:
            raise IOError(f"Error reading file {filepath}: {e}")

        processed_lines = self._process_lines(lines)
        height = len(processed_lines)
        width = max(len(line) for line in processed_lines) if processed_lines else 0
        line_count = len(lines)  # Original line count including empty lines

        return (width, height, line_count)

    def validate_file(self, filepath: str) -> bool:
        """
        Validate that a file can be successfully parsed.

        Args:
            filepath: Path to the .tub file to validate

        Returns:
            True if file is valid and can be parsed

        Raises:
            Same exceptions as parse_file() if file is invalid
        """
        try:
            self.parse_file(filepath)
            return True
        except (FileNotFoundError, IOError, ValueError):
            return False