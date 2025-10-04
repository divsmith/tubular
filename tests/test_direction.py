#!/usr/bin/env python3
"""
Unit tests for Direction class in Tubular programming language.
"""

import pytest
from src.direction import Direction


class TestDirection:
    """Test cases for Direction enum functionality."""

    def test_direction_enum_values(self):
        """Test that all direction enum values are correct."""
        assert Direction.UP.value == "UP"
        assert Direction.DOWN.value == "DOWN"
        assert Direction.LEFT.value == "LEFT"
        assert Direction.RIGHT.value == "RIGHT"

    def test_direction_string_representation(self):
        """Test string representation of directions."""
        assert str(Direction.UP) == "UP"
        assert str(Direction.DOWN) == "DOWN"
        assert str(Direction.LEFT) == "LEFT"
        assert str(Direction.RIGHT) == "RIGHT"

    def test_from_string_valid_inputs(self):
        """Test creating Direction from valid string inputs."""
        assert Direction.from_string("up") == Direction.UP
        assert Direction.from_string("UP") == Direction.UP
        assert Direction.from_string("Down") == Direction.DOWN
        assert Direction.from_string("DOWN") == Direction.DOWN
        assert Direction.from_string("left") == Direction.LEFT
        assert Direction.from_string("Left") == Direction.LEFT
        assert Direction.from_string("right") == Direction.RIGHT
        assert Direction.from_string("RIGHT") == Direction.RIGHT

    def test_from_string_invalid_inputs(self):
        """Test that invalid string inputs raise ValueError."""
        with pytest.raises(ValueError, match="Invalid direction: invalid"):
            Direction.from_string("invalid")

        with pytest.raises(ValueError, match="Invalid direction: diagonal"):
            Direction.from_string("diagonal")

        with pytest.raises(ValueError, match="Invalid direction: north"):
            Direction.from_string("north")

    def test_all_directions_are_unique(self):
        """Test that all direction values are unique."""
        directions = [Direction.UP, Direction.DOWN, Direction.LEFT, Direction.RIGHT]
        assert len(directions) == len(set(directions))

    def test_direction_enum_iteration(self):
        """Test that we can iterate over all directions."""
        directions = list(Direction)
        assert len(directions) == 4
        assert Direction.UP in directions
        assert Direction.DOWN in directions
        assert Direction.LEFT in directions
        assert Direction.RIGHT in directions

    def test_direction_equality(self):
        """Test direction equality comparisons."""
        assert Direction.UP == Direction.UP
        assert Direction.UP != Direction.DOWN
        assert Direction.LEFT == Direction.LEFT
        assert Direction.RIGHT != Direction.LEFT

    def test_direction_hash_consistency(self):
        """Test that direction hashing is consistent."""
        # Same direction should have same hash
        assert hash(Direction.UP) == hash(Direction.UP)

        # Different directions should potentially have different hashes
        # (though hash collisions are possible, they should be consistent)
        up_hash = hash(Direction.UP)
        down_hash = hash(Direction.DOWN)
        # We can't guarantee different hashes, but they should be consistent
        assert hash(Direction.UP) == up_hash
        assert hash(Direction.DOWN) == down_hash