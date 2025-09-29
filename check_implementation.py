#!/usr/bin/env python3
"""
Simple test script to check if our Tubular interpreter components work.
"""

import sys
import os
# Add the src directory to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'src'))

def test_grid():
    """Test the Grid functionality."""
    from grid import Grid
    
    # Create a simple grid with the start character
    grid_str = "@"
    grid = Grid(grid_str)
    
    print("Grid created successfully")
    print(f"Grid dimensions: {grid.width} x {grid.height}")
    print(f"Start position: {grid.start_pos}")
    print(f"Cell (0,0): {grid.get_cell(0, 0)}")
    
    # Test reservoir
    grid.set_reservoir_value(1, 2, 42)
    print(f"Reservoir (1,2): {grid.get_reservoir_value(1, 2)}")
    
    return True

def test_droplet():
    """Test the Droplet functionality."""
    from droplet import Droplet, Direction
    
    # Create a droplet
    droplet = Droplet(value=5, direction=Direction.RIGHT)
    
    print(f"Droplet: {droplet}")
    print(f"Droplet move: {droplet.move()}")
    
    # Change direction
    droplet.change_direction(Direction.UP)
    print(f"Droplet after direction change: {droplet}")
    
    return True

if __name__ == "__main__":
    print("Testing Grid...")
    test_grid()
    
    print("
Testing Droplet...")
    test_droplet()
    
    print("
All basic tests passed!")
