#!/usr/bin/env python3
"""
Droplet class for Tubular programming language.

Represents a droplet that moves through the grid during program execution.
"""

from typing import Tuple
from .direction import Direction


class Droplet:
    """
    A droplet that carries a value and moves through the grid.

    Droplets are the primary execution mechanism in Tubular, flowing through
    the grid and performing operations based on the characters they encounter.
    """

    def __init__(self, value: int, x: int, y: int, direction: Direction):
        """
        Initialize a new droplet.

        Args:
            value: The integer value carried by the droplet
            x: The x-coordinate (column) position
            y: The y-coordinate (row) position
            direction: The direction the droplet is moving
        """
        self.value = value
        self.x = x
        self.y = y
        self.direction = direction

    @property
    def position(self) -> Tuple[int, int]:
        """
        Get the current position of the droplet.

        Returns:
            A tuple (x, y) representing the current coordinates
        """
        return (self.x, self.y)

    @position.setter
    def position(self, pos: Tuple[int, int]) -> None:
        """
        Set the position of the droplet.

        Args:
            pos: A tuple (x, y) of new coordinates
        """
        self.x, self.y = pos

    def move(self) -> None:
        """
        Move the droplet one step in its current direction.

        This method updates the droplet's position based on its direction.
        It does not perform bounds checking - that should be handled by the caller.
        """
        if self.direction == Direction.UP:
            self.y -= 1
        elif self.direction == Direction.DOWN:
            self.y += 1
        elif self.direction == Direction.LEFT:
            self.x -= 1
        elif self.direction == Direction.RIGHT:
            self.x += 1

    def set_direction(self, direction: Direction) -> None:
        """
        Set the movement direction of the droplet.

        Args:
            direction: The new direction for the droplet
        """
        self.direction = direction

    def clone(self) -> 'Droplet':
        """
        Create a copy of this droplet with the same state.

        Returns:
            A new Droplet instance with identical properties
        """
        return Droplet(self.value, self.x, self.y, self.direction)

    def __str__(self) -> str:
        """Return string representation of the droplet."""
        return f"Droplet(value={self.value}, pos=({self.x}, {self.y}), dir={self.direction.value})"

    def __repr__(self) -> str:
        """Return detailed string representation of the droplet."""
        return f"Droplet(value={self.value}, x={self.x}, y={self.y}, direction={self.direction})"

    def __eq__(self, other) -> bool:
        """Check equality with another droplet."""
        if not isinstance(other, Droplet):
            return False
        return (self.value == other.value and
                self.x == other.x and
                self.y == other.y and
                self.direction == other.direction)

    def __hash__(self) -> int:
        """Return hash for use in sets and dictionaries."""
        return hash((self.value, self.x, self.y, self.direction))