"""Droplet data structure for the Tubular programming language.
A droplet represents a unit of data with a value and direction.
"""
from enum import Enum
from typing import Tuple


class Direction(Enum):
    """
    Direction in which a droplet is traveling.
    """
    UP = (-1, 0)
    DOWN = (1, 0)
    LEFT = (0, -1)
    RIGHT = (0, 1)


class Droplet:
    """
    A droplet in the Tubular programming language.
    Each droplet has a value and a direction of travel.
    """
    
    def __init__(self, value: int = 0, direction: Direction = Direction.DOWN, is_ascii_output: bool = False):
        """
        Initialize a droplet.
        
        Args:
            value: Initial value of the droplet (default 0)
            direction: Initial direction of travel (default DOWN)
            is_ascii_output: Whether this droplet should be output as ASCII (default False)
        """
        self.value = value
        self.direction = direction
        self.is_ascii_output = is_ascii_output
    
    def move(self) -> Tuple[int, int]:
        """
        Move the droplet one cell in its current direction.
        
        Returns:
            A tuple (dy, dx) representing the change in position
        """
        dy, dx = self.direction.value
        return dy, dx
    
    def change_direction(self, new_direction: Direction):
        """
        Change the direction of the droplet.
        
        Args:
            new_direction: The new direction for the droplet
        """
        self.direction = new_direction
    
    def __repr__(self):
        """
        String representation of the droplet.
        
        Returns:
            A string showing the value and direction of the droplet
        """
        return f"Droplet(value={self.value}, direction={self.direction.name})"
