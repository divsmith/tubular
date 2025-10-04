#!/usr/bin/env python3
"""
Direction enum for Tubular programming language droplet movement.
"""

from enum import Enum


class Direction(Enum):
    """Enumeration of possible droplet movement directions."""
    UP = "UP"
    DOWN = "DOWN"
    LEFT = "LEFT"
    RIGHT = "RIGHT"

    def __str__(self) -> str:
        """Return string representation of the direction."""
        return self.value

    @classmethod
    def from_string(cls, value: str) -> 'Direction':
        """Create Direction from string value."""
        try:
            return cls(value.upper())
        except ValueError:
            raise ValueError(f"Invalid direction: {value}. Must be one of: {[d.value for d in cls]}")