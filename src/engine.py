#!/usr/bin/env python3
"""
Execution Engine for Tubular programming language.

Manages the execution of droplets on a grid during program execution.
"""

from typing import List
try:
    from .grid import Grid
    from .droplet import Droplet
    from .direction import Direction
except ImportError:
    from grid import Grid
    from droplet import Droplet
    from direction import Direction


class Engine:
    """
    Execution engine for Tubular programs.

    Manages a grid and a collection of active droplets, executing
    movement and collision detection during each tick.
    """

    def __init__(self, grid: Grid):
        """
        Initialize the execution engine with a grid.

        Args:
            grid: The Grid instance representing the program
        """
        self.grid = grid
        self.droplets: List[Droplet] = []
        self._initialize_program()

    def add_droplet(self, droplet: Droplet) -> None:
        """
        Add a droplet to the engine's active droplet list.

        Args:
            droplet: The Droplet instance to add
        """
        self.droplets.append(droplet)

    def remove_droplet(self, droplet: Droplet) -> None:
        """
        Remove a droplet from the engine's active droplet list.

        Args:
            droplet: The Droplet instance to remove
        """
        if droplet in self.droplets:
            self.droplets.remove(droplet)

    def _initialize_program(self) -> None:
        """
        Initialize the program by scanning for '@' (Program Start) character.

        Creates an initial trigger droplet (value 0, direction Down) at the '@' position.
        """
        for y in range(self.grid.height):
            for x in range(self.grid.width):
                if self.grid.get(x, y) == '@':
                    # Create initial droplet with value 0, direction DOWN
                    initial_droplet = Droplet(0, x, y, Direction.DOWN)
                    self.add_droplet(initial_droplet)
                    return  # Only one '@' should exist, so we can return after finding it

    def tick(self) -> None:
        """
        Execute one tick of the simulation.

        For each active droplet:
        - Calculate the new position based on its direction
        - Handle pipe characters according to their behavior:
          * '|' (Vertical Pipe): allows vertical movement (UP/DOWN)
          * '-' (Horizontal Pipe): allows horizontal movement (LEFT/RIGHT)
          * '^' (Go Up Pipe): forces direction to UP
          * '#' (Wall): destroys droplet
        - If movement not allowed or out of bounds, destroy the droplet
        - After calculating all positions, check for collisions
        """
        droplets_to_remove: List[Droplet] = []
        new_positions: List[tuple] = []  # Track (droplet, new_x, new_y) for collision detection

        for droplet in self.droplets:
            # Calculate new position
            new_x = droplet.x
            new_y = droplet.y

            if droplet.direction == Direction.UP:
                new_y -= 1
            elif droplet.direction == Direction.DOWN:
                new_y += 1
            elif droplet.direction == Direction.LEFT:
                new_x -= 1
            elif droplet.direction == Direction.RIGHT:
                new_x += 1

            # Check if new position is valid
            if not (0 <= new_x < self.grid.width and 0 <= new_y < self.grid.height):
                # Out of bounds - destroy droplet
                droplets_to_remove.append(droplet)
                continue

            target_cell = self.grid.get(new_x, new_y)

            # Handle pipe characters
            if target_cell == '|':
                # Vertical pipe - only allow vertical movement
                if droplet.direction in [Direction.UP, Direction.DOWN]:
                    droplet.x = new_x
                    droplet.y = new_y
                    new_positions.append((droplet, new_x, new_y))
                else:
                    # Wrong direction for vertical pipe - destroy droplet
                    droplets_to_remove.append(droplet)
            elif target_cell == '-':
                # Horizontal pipe - only allow horizontal movement
                if droplet.direction in [Direction.LEFT, Direction.RIGHT]:
                    droplet.x = new_x
                    droplet.y = new_y
                    new_positions.append((droplet, new_x, new_y))
                else:
                    # Wrong direction for horizontal pipe - destroy droplet
                    droplets_to_remove.append(droplet)
            elif target_cell == '^':
                # Go up pipe - force direction to UP and move
                droplet.direction = Direction.UP
                droplet.x = new_x
                droplet.y = new_y
                new_positions.append((droplet, new_x, new_y))
            elif target_cell == '#':
                # Wall - destroy droplet
                droplets_to_remove.append(droplet)
            elif target_cell == 'n':
                # Numeric output - print droplet value and destroy droplet
                print(droplet.value)
                droplets_to_remove.append(droplet)
            elif target_cell == '+':
                # Increment operator - add 1 to droplet value and move
                droplet.value += 1
                droplet.x = new_x
                droplet.y = new_y
                new_positions.append((droplet, new_x, new_y))
            elif target_cell == '~':
                # Decrement operator - subtract 1 from droplet value and move
                droplet.value -= 1
                droplet.x = new_x
                droplet.y = new_y
                new_positions.append((droplet, new_x, new_y))
            elif target_cell == ' ':
                # Empty space - move droplet
                droplet.x = new_x
                droplet.y = new_y
                new_positions.append((droplet, new_x, new_y))
            else:
                # Any other non-empty cell - destroy droplet
                droplets_to_remove.append(droplet)

        # Check for collisions after calculating all positions
        position_count = {}
        for droplet, x, y in new_positions:
            pos_key = (x, y)
            position_count[pos_key] = position_count.get(pos_key, 0) + 1

        # Mark droplets for removal if they collide (2+ droplets in same position)
        for droplet, x, y in new_positions:
            if position_count[(x, y)] > 1:
                droplets_to_remove.append(droplet)

        # Remove all destroyed droplets
        for droplet in droplets_to_remove:
            self.remove_droplet(droplet)

    def get_active_droplets(self) -> List[Droplet]:
        """
        Get a copy of the current active droplets list.

        Returns:
            List of active Droplet instances
        """
        return self.droplets.copy()

    def is_empty(self) -> bool:
        """
        Check if there are any active droplets.

        Returns:
            True if no droplets are active, False otherwise
        """
        return len(self.droplets) == 0

    def __str__(self) -> str:
        """Return string representation of the engine state."""
        return f"Engine(grid={self.grid}, active_droplets={len(self.droplets)})"

    def __repr__(self) -> str:
        """Return detailed string representation of the engine."""
        return f"Engine(grid={self.grid!r}, droplets={self.droplets!r})"