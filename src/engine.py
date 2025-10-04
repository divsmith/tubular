#!/usr/bin/env python3
"""
Execution Engine for Tubular programming language.

Manages the execution of droplets on a grid during program execution.
"""

from typing import List
from .grid import Grid
from .droplet import Droplet
from .direction import Direction


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

    def tick(self) -> None:
        """
        Execute one tick of the simulation.

        For each active droplet:
        - Calculate the new position based on its direction
        - If the target cell is empty (' '), move the droplet there
        - If the target cell is non-empty or out of bounds, destroy the droplet
        """
        droplets_to_remove: List[Droplet] = []

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

            # Check if new position is valid and empty
            if (0 <= new_x < self.grid.width and
                0 <= new_y < self.grid.height and
                self.grid.get(new_x, new_y) == ' '):

                # Move droplet to new position
                droplet.x = new_x
                droplet.y = new_y
            else:
                # Mark droplet for removal (collision or out of bounds)
                droplets_to_remove.append(droplet)

        # Remove destroyed droplets
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