#!/usr/bin/env python3
"""
Unit tests for Droplet class in Tubular programming language.
"""

import pytest
from src.droplet import Droplet
from src.direction import Direction


class TestDroplet:
    """Test cases for Droplet class functionality."""

    def test_droplet_constructor(self):
        """Test Droplet constructor with valid parameters."""
        droplet = Droplet(42, 5, 10, Direction.UP)

        assert droplet.value == 42
        assert droplet.x == 5
        assert droplet.y == 10
        assert droplet.direction == Direction.UP

    def test_droplet_constructor_zero_value(self):
        """Test Droplet constructor with zero value."""
        droplet = Droplet(0, 0, 0, Direction.LEFT)
        assert droplet.value == 0
        assert droplet.x == 0
        assert droplet.y == 0
        assert droplet.direction == Direction.LEFT

    def test_droplet_constructor_negative_coordinates(self):
        """Test Droplet constructor with negative coordinates."""
        droplet = Droplet(100, -5, -3, Direction.DOWN)
        assert droplet.value == 100
        assert droplet.x == -5
        assert droplet.y == -3
        assert droplet.direction == Direction.DOWN

    def test_droplet_position_property_getter(self):
        """Test position property getter."""
        droplet = Droplet(123, 7, 15, Direction.RIGHT)

        position = droplet.position
        assert position == (7, 15)
        assert droplet.x == 7
        assert droplet.y == 15

    def test_droplet_position_property_setter(self):
        """Test position property setter."""
        droplet = Droplet(456, 0, 0, Direction.UP)

        droplet.position = (10, 20)
        assert droplet.x == 10
        assert droplet.y == 20
        assert droplet.position == (10, 20)

    def test_droplet_position_setter_tuple_unpacking(self):
        """Test position setter with tuple unpacking."""
        droplet = Droplet(789, 1, 1, Direction.LEFT)

        # Test setting position with various tuple formats
        droplet.position = (5, 8)
        assert droplet.position == (5, 8)

        droplet.position = (-1, -1)
        assert droplet.position == (-1, -1)

        droplet.position = (0, 0)
        assert droplet.position == (0, 0)

    def test_droplet_move_up(self):
        """Test moving droplet upward."""
        droplet = Droplet(100, 5, 10, Direction.UP)
        droplet.move()

        assert droplet.x == 5
        assert droplet.y == 9  # Should decrease by 1
        assert droplet.direction == Direction.UP

    def test_droplet_move_down(self):
        """Test moving droplet downward."""
        droplet = Droplet(200, 3, 4, Direction.DOWN)
        droplet.move()

        assert droplet.x == 3
        assert droplet.y == 5  # Should increase by 1
        assert droplet.direction == Direction.DOWN

    def test_droplet_move_left(self):
        """Test moving droplet left."""
        droplet = Droplet(300, 8, 6, Direction.LEFT)
        droplet.move()

        assert droplet.x == 7  # Should decrease by 1
        assert droplet.y == 6
        assert droplet.direction == Direction.LEFT

    def test_droplet_move_right(self):
        """Test moving droplet right."""
        droplet = Droplet(400, 2, 9, Direction.RIGHT)
        droplet.move()

        assert droplet.x == 3  # Should increase by 1
        assert droplet.y == 9
        assert droplet.direction == Direction.RIGHT

    def test_droplet_move_sequence(self):
        """Test moving droplet in sequence."""
        droplet = Droplet(500, 0, 0, Direction.RIGHT)

        # Move right three times
        droplet.move()
        assert droplet.position == (1, 0)

        droplet.move()
        assert droplet.position == (2, 0)

        droplet.move()
        assert droplet.position == (3, 0)

    def test_droplet_move_all_directions(self):
        """Test moving droplet in all directions from center."""
        # Start at center
        droplet = Droplet(999, 0, 0, Direction.UP)

        # Move up
        droplet.move()
        assert droplet.position == (0, -1)

        # Change direction and move down
        droplet.set_direction(Direction.DOWN)
        droplet.move()
        assert droplet.position == (0, 0)

        # Move left
        droplet.set_direction(Direction.LEFT)
        droplet.move()
        assert droplet.position == (-1, 0)

        # Move right
        droplet.set_direction(Direction.RIGHT)
        droplet.move()
        assert droplet.position == (0, 0)

    def test_droplet_set_direction(self):
        """Test setting droplet direction."""
        droplet = Droplet(111, 0, 0, Direction.UP)

        # Change to different direction
        droplet.set_direction(Direction.DOWN)
        assert droplet.direction == Direction.DOWN

        droplet.set_direction(Direction.LEFT)
        assert droplet.direction == Direction.LEFT

        droplet.set_direction(Direction.RIGHT)
        assert droplet.direction == Direction.RIGHT

    def test_droplet_clone(self):
        """Test cloning a droplet."""
        original = Droplet(222, 5, 10, Direction.UP)
        cloned = original.clone()

        # Should have same properties
        assert cloned.value == 222
        assert cloned.x == 5
        assert cloned.y == 10
        assert cloned.direction == Direction.UP

        # Should be equal
        assert cloned == original

        # But different objects
        assert cloned is not original

    def test_droplet_clone_independence(self):
        """Test that cloned droplet is independent of original."""
        original = Droplet(333, 1, 2, Direction.LEFT)
        cloned = original.clone()

        # Modify original
        original.value = 999
        original.x = 10
        original.y = 20
        original.set_direction(Direction.DOWN)

        # Clone should remain unchanged
        assert cloned.value == 333
        assert cloned.x == 1
        assert cloned.y == 2
        assert cloned.direction == Direction.LEFT

        # And original should have new values
        assert original.value == 999
        assert original.x == 10
        assert original.y == 20
        assert original.direction == Direction.DOWN

    def test_droplet_string_representation(self):
        """Test string representation of droplet."""
        droplet = Droplet(42, 5, 10, Direction.UP)

        droplet_str = str(droplet)
        assert "Droplet(value=42, pos=(5, 10), dir=UP)" == droplet_str

    def test_droplet_repr(self):
        """Test repr of droplet."""
        droplet = Droplet(123, 7, 15, Direction.DOWN)

        droplet_repr = repr(droplet)
        expected = "Droplet(value=123, x=7, y=15, direction=DOWN)"
        assert droplet_repr == expected

    def test_droplet_equality_same_properties(self):
        """Test equality with droplets having same properties."""
        droplet1 = Droplet(100, 5, 10, Direction.UP)
        droplet2 = Droplet(100, 5, 10, Direction.UP)

        assert droplet1 == droplet2
        assert droplet2 == droplet1

    def test_droplet_equality_different_value(self):
        """Test equality with different values."""
        droplet1 = Droplet(100, 5, 10, Direction.UP)
        droplet2 = Droplet(200, 5, 10, Direction.UP)

        assert droplet1 != droplet2

    def test_droplet_equality_different_position(self):
        """Test equality with different positions."""
        droplet1 = Droplet(100, 5, 10, Direction.UP)
        droplet2 = Droplet(100, 6, 10, Direction.UP)

        assert droplet1 != droplet2

        droplet3 = Droplet(100, 5, 11, Direction.UP)
        assert droplet1 != droplet3

    def test_droplet_equality_different_direction(self):
        """Test equality with different directions."""
        droplet1 = Droplet(100, 5, 10, Direction.UP)
        droplet2 = Droplet(100, 5, 10, Direction.DOWN)

        assert droplet1 != droplet2

    def test_droplet_equality_with_non_droplet(self):
        """Test equality with non-Droplet objects."""
        droplet = Droplet(100, 5, 10, Direction.UP)

        assert droplet != "not a droplet"
        assert droplet != 42
        assert droplet != None
        assert droplet != []

    def test_droplet_hash_consistency(self):
        """Test that droplet hash is consistent."""
        droplet = Droplet(100, 5, 10, Direction.UP)

        # Hash should be consistent
        hash1 = hash(droplet)
        hash2 = hash(droplet)
        assert hash1 == hash2

    def test_droplet_hash_based_on_properties(self):
        """Test that hash is based on droplet properties."""
        droplet1 = Droplet(100, 5, 10, Direction.UP)
        droplet2 = Droplet(100, 5, 10, Direction.UP)
        droplet3 = Droplet(200, 5, 10, Direction.UP)

        # Equal droplets should have equal hashes
        assert droplet1 == droplet2
        assert hash(droplet1) == hash(droplet2)

        # Different droplets should potentially have different hashes
        # (though collisions are possible)
        assert droplet1 != droplet3

    def test_droplet_hash_with_position_changes(self):
        """Test that hash changes when properties change."""
        droplet = Droplet(100, 5, 10, Direction.UP)

        original_hash = hash(droplet)

        # Change value
        droplet.value = 200
        assert hash(droplet) != original_hash

        # Reset and change position
        droplet = Droplet(100, 5, 10, Direction.UP)
        droplet.position = (6, 11)
        assert hash(droplet) != original_hash

        # Reset and change direction
        droplet = Droplet(100, 5, 10, Direction.UP)
        droplet.set_direction(Direction.DOWN)
        assert hash(droplet) != original_hash

    def test_droplet_with_large_values(self):
        """Test droplet with large integer values."""
        large_value = 999999
        droplet = Droplet(large_value, 1000, 2000, Direction.RIGHT)

        assert droplet.value == large_value
        assert droplet.x == 1000
        assert droplet.y == 2000
        assert droplet.direction == Direction.RIGHT

        # Test movement still works
        droplet.move()
        assert droplet.position == (1001, 2000)

    def test_droplet_move_from_negative_coordinates(self):
        """Test moving droplet from negative coordinates."""
        droplet = Droplet(50, -5, -3, Direction.UP)

        # Move up (y decreases)
        droplet.move()
        assert droplet.position == (-5, -4)

        # Change direction and move down (y increases)
        droplet.set_direction(Direction.DOWN)
        droplet.move()
        assert droplet.position == (-5, -3)

        # Move left (x decreases)
        droplet.set_direction(Direction.LEFT)
        droplet.move()
        assert droplet.position == (-6, -3)

        # Move right (x increases)
        droplet.set_direction(Direction.RIGHT)
        droplet.move()
        assert droplet.position == (-5, -3)