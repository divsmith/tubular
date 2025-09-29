"""
Grid data structure for the Tubular programming language.
The grid represents the 2D pipe system through which droplets flow.
"""
from typing import List, Tuple, Optional


class Grid:
    """
    A 2D grid representing the Tubular program.
    Each cell contains a character that defines the pipe behavior.
    """
    
    def __init__(self, grid_str: str):
        """
        Initialize the grid from a string representation.
        
        Args:
            grid_str: A multi-line string representing the grid
        """
        lines = grid_str.split('\n')
        # Remove potentially empty last line if the string ends with a newline
        if lines and lines[-1] == '':
            lines = lines[:-1]
        self.width = max(len(line) for line in lines) if lines else 0
        self.height = len(lines)
        
        # Pad lines to ensure consistent width
        self.grid: List[List[str]] = []
        for line in lines:
            padded_line = list(line.ljust(self.width))
            self.grid.append(padded_line)
        
        # Initialize the reservoir (2D memory grid)
        # Using a dictionary to store values for sparse memory representation
        self.reservoir: dict = {}
        
        # Find the starting position of the program ('@')
        self.start_pos: Optional[Tuple[int, int]] = None
        for y in range(self.height):
            for x in range(self.width):
                if self.grid[y][x] == '@':
                    self.start_pos = (x, y)
                    break
            if self.start_pos:
                break
        
        if not self.start_pos:
            raise ValueError("No starting position '@' found in the grid")
    
    def get_cell(self, x: int, y: int) -> str:
        """
        Get the character at position (x, y) in the grid.
        
        Args:
            x: X coordinate (column)
            y: Y coordinate (row)
            
        Returns:
            The character at the specified position, or ' ' (space) if out of bounds
        """
        if 0 <= y < self.height and 0 <= x < self.width:
            return self.grid[y][x]
        return ' '  # Treat out-of-bounds as empty space
    
    def set_cell(self, x: int, y: int, char: str):
        """
        Set the character at position (x, y) in the grid.
        
        Args:
            x: X coordinate (column)
            y: Y coordinate (row)
            char: Character to set
        """
        if 0 <= y < self.height and 0 <= x < self.width:
            self.grid[y][x] = char[0] if char else ' '
    
    def get_reservoir_value(self, x: int, y: int) -> int:
        """
        Get the value at position (x, y) in the reservoir.
        
        Args:
            x: X coordinate in the reservoir
            y: Y coordinate in the reservoir
            
        Returns:
            The value at the specified position, or 0 if not set
        """
        return self.reservoir.get((x, y), 0)
    
    def set_reservoir_value(self, x: int, y: int, value: int):
        """
        Set the value at position (x, y) in the reservoir.
        
        Args:
            x: X coordinate in the reservoir
            y: Y coordinate in the reservoir
            value: Value to set
        """
        self.reservoir[(x, y)] = value
    
    def __str__(self):
        """
        String representation of the grid.
        
        Returns:
            A string showing the grid layout
        """
        return '\n'.join(''.join(row) for row in self.grid)