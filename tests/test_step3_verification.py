#!/usr/bin/env python3
"""
Step 3 Verification Tests for Tubular Engine Implementation.

Tests the basic flow control pipes and collision detection as specified in Step 3:
- Test `|` pipe: droplet entering from top continues Down, from bottom continues Up
- Test `-` pipe: droplet entering from left continues Right, from right continues Left
- Test `^` pipe: droplet entering from any direction changes to Up direction
- Test `#` wall: droplet moving into wall cell is destroyed
- Test collision detection: two droplets aimed at same cell both get destroyed
"""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from src.engine import Engine
from src.grid import Grid
from src.droplet import Droplet
from src.direction import Direction

def test_vertical_pipe_from_top():
    """Test | pipe: droplet entering from top continues Down."""

    grid = Grid(5, 5)
    engine = Engine(grid)

    # Place vertical pipe at (2, 2)
    grid._grid[2][2] = '|'

    # Create droplet at (2, 1) moving DOWN (entering from top)
    droplet = Droplet(42, 2, 1, Direction.DOWN)
    engine.add_droplet(droplet)

    # Execute tick
    engine.tick()

    # Verify droplet moved down through the pipe
    active_droplets = engine.get_active_droplets()
    assert len(active_droplets) == 1, f"Expected 1 active droplet, got {len(active_droplets)}"

    droplet = active_droplets[0]
    expected_pos = (2, 2)  # Should be at the pipe position
    actual_pos = (droplet.x, droplet.y)
    assert droplet.x == 2 and droplet.y == 2, f"Droplet should move down through | pipe, expected position ({expected_pos[0]}, {expected_pos[1]}), got position ({actual_pos[0]}, {actual_pos[1]})"

def test_vertical_pipe_from_bottom():
    """Test | pipe: droplet entering from bottom continues Up."""

    grid = Grid(5, 5)
    engine = Engine(grid)

    # Place vertical pipe at (2, 2)
    grid._grid[2][2] = '|'

    # Create droplet at (2, 3) moving UP (entering from bottom)
    droplet = Droplet(42, 2, 3, Direction.UP)
    engine.add_droplet(droplet)

    # Execute tick
    engine.tick()

    # Verify droplet moved up through the pipe
    active_droplets = engine.get_active_droplets()
    assert len(active_droplets) == 1, f"Expected 1 active droplet, got {len(active_droplets)}"

    droplet = active_droplets[0]
    expected_pos = (2, 2)  # Should be at the pipe position
    actual_pos = (droplet.x, droplet.y)
    assert droplet.x == 2 and droplet.y == 2, f"Droplet should move up through | pipe, expected position ({expected_pos[0]}, {expected_pos[1]}), got position ({actual_pos[0]}, {actual_pos[1]})"


def test_vertical_pipe_wrong_direction():
    """Test | pipe: droplet entering horizontally should be destroyed."""

    grid = Grid(5, 5)
    engine = Engine(grid)

    # Place vertical pipe at (2, 2)
    grid._grid[2][2] = '|'

    # Create droplet at (1, 2) moving RIGHT (wrong direction for vertical pipe)
    droplet = Droplet(42, 1, 2, Direction.RIGHT)
    engine.add_droplet(droplet)

    # Execute tick
    engine.tick()

    # Verify droplet was destroyed
    active_droplets = engine.get_active_droplets()
    assert len(active_droplets) == 0, "Droplet should be destroyed when hitting | pipe from side"

def test_horizontal_pipe_from_left():
    """Test - pipe: droplet entering from left continues Right."""

    grid = Grid(5, 5)
    engine = Engine(grid)

    # Place horizontal pipe at (2, 2)
    grid._grid[2][2] = '-'

    # Create droplet at (1, 2) moving RIGHT (entering from left)
    droplet = Droplet(42, 1, 2, Direction.RIGHT)
    engine.add_droplet(droplet)

    # Execute tick
    engine.tick()

    # Verify droplet moved right through the pipe
    active_droplets = engine.get_active_droplets()
    assert len(active_droplets) == 1, f"Expected 1 active droplet, got {len(active_droplets)}"

    droplet = active_droplets[0]
    expected_pos = (2, 2)  # Should be at the pipe position
    actual_pos = (droplet.x, droplet.y)
    assert droplet.x == 2 and droplet.y == 2, f"Droplet should move right through - pipe, expected position ({expected_pos[0]}, {expected_pos[1]}), got position ({actual_pos[0]}, {actual_pos[1]})"


def test_horizontal_pipe_from_right():
    """Test - pipe: droplet entering from right continues Left."""

    grid = Grid(5, 5)
    engine = Engine(grid)

    # Place horizontal pipe at (2, 2)
    grid._grid[2][2] = '-'

    # Create droplet at (3, 2) moving LEFT (entering from right)
    droplet = Droplet(42, 3, 2, Direction.LEFT)
    engine.add_droplet(droplet)

    # Execute tick
    engine.tick()

    # Verify droplet moved left through the pipe
    active_droplets = engine.get_active_droplets()
    assert len(active_droplets) == 1, f"Expected 1 active droplet, got {len(active_droplets)}"

    droplet = active_droplets[0]
    expected_pos = (2, 2)  # Should be at the pipe position
    actual_pos = (droplet.x, droplet.y)
    assert droplet.x == 2 and droplet.y == 2, f"Droplet should move left through - pipe, expected position ({expected_pos[0]}, {expected_pos[1]}), got position ({actual_pos[0]}, {actual_pos[1]})"

def test_go_up_pipe_any_direction():
    """Test ^ pipe: droplet entering from any direction changes to Up."""

    # Test from each direction
    directions_to_test = [
        (Direction.UP, "from above"),
        (Direction.DOWN, "from below"),
        (Direction.LEFT, "from left"),
        (Direction.RIGHT, "from right")
    ]

    for direction, desc in directions_to_test:
        grid = Grid(5, 5)
        engine = Engine(grid)

        # Place go-up pipe at (2, 2)
        grid._grid[2][2] = '^'

        # Create droplet approaching the pipe from different position based on direction
        if direction == Direction.UP:
            droplet = Droplet(42, 2, 3, Direction.UP)  # From below moving up
        elif direction == Direction.DOWN:
            droplet = Droplet(42, 2, 1, Direction.DOWN)  # From above moving down
        elif direction == Direction.LEFT:
            droplet = Droplet(42, 3, 2, Direction.LEFT)  # From right moving left
        elif direction == Direction.RIGHT:
            droplet = Droplet(42, 1, 2, Direction.RIGHT)  # From left moving right

        engine.add_droplet(droplet)

        # Execute tick
        engine.tick()

        # Verify droplet moved to pipe position and direction changed to UP
        active_droplets = engine.get_active_droplets()
        assert len(active_droplets) == 1, f"Expected 1 active droplet, got {len(active_droplets)}"

        droplet = active_droplets[0]
        pos_correct = (droplet.x == 2 and droplet.y == 2)
        dir_correct = (droplet.direction == Direction.UP)
        assert pos_correct and dir_correct, f"Droplet should move to ^ pipe and change direction to UP, expected position (2, 2) and direction UP, got position ({droplet.x}, {droplet.y}) and direction {droplet.direction}"

def test_wall_destruction():
    """Test # wall: droplet moving into wall cell is destroyed."""

    # Test wall from each direction
    directions_to_test = [Direction.UP, Direction.DOWN, Direction.LEFT, Direction.RIGHT]

    for direction in directions_to_test:
        grid = Grid(5, 5)
        engine = Engine(grid)

        # Place wall at (2, 2)
        grid._grid[2][2] = '#'

        # Create droplet that will hit the wall
        if direction == Direction.UP:
            droplet = Droplet(42, 2, 3, Direction.UP)  # From below moving up
        elif direction == Direction.DOWN:
            droplet = Droplet(42, 2, 1, Direction.DOWN)  # From above moving down
        elif direction == Direction.LEFT:
            droplet = Droplet(42, 3, 2, Direction.LEFT)  # From right moving left
        elif direction == Direction.RIGHT:
            droplet = Droplet(42, 1, 2, Direction.RIGHT)  # From left moving right

        engine.add_droplet(droplet)

        # Execute tick
        engine.tick()

        # Verify droplet was destroyed
        active_droplets = engine.get_active_droplets()
        assert len(active_droplets) == 0, f"Droplet should be destroyed when hitting # wall from {direction.value}"


def test_collision_detection():
    """Test collision detection: two droplets aimed at same cell both get destroyed."""

    grid = Grid(5, 5)
    engine = Engine(grid)

    # Create two droplets that will collide at (2, 2)
    droplet1 = Droplet(42, 1, 2, Direction.RIGHT)  # Moving right towards (2, 2)
    droplet2 = Droplet(99, 3, 2, Direction.LEFT)   # Moving left towards (2, 2)

    engine.add_droplet(droplet1)
    engine.add_droplet(droplet2)

    # Verify both droplets are initially active
    active_droplets = engine.get_active_droplets()
    assert len(active_droplets) == 2, "Both droplets should be initially active"

    # Execute tick - both should collide and be destroyed
    engine.tick()

    # Verify both droplets were destroyed
    active_droplets = engine.get_active_droplets()
    assert len(active_droplets) == 0, "Both droplets should be destroyed in collision"

def test_no_collision_when_not_colliding():
    """Test that droplets don't collide when not aiming at same cell."""

    grid = Grid(5, 5)
    engine = Engine(grid)

    # Create two droplets that won't collide
    droplet1 = Droplet(42, 1, 1, Direction.RIGHT)  # Will go to (2, 1)
    droplet2 = Droplet(99, 1, 2, Direction.RIGHT)  # Will go to (2, 2)

    engine.add_droplet(droplet1)
    engine.add_droplet(droplet2)

    # Execute tick
    engine.tick()

    # Verify both droplets survived
    active_droplets = engine.get_active_droplets()
    assert len(active_droplets) == 2, "Both droplets should survive when not colliding"