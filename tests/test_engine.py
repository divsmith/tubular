#!/usr/bin/env python3
"""
Test suite for the Execution Engine implementation.

Verifies the acceptance criteria for Step 2:
1. Single droplet movement test
2. Collision/destruction test
3. State management test
4. Edge cases (grid boundaries, multiple droplets)
"""

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from src.engine import Engine
from src.grid import Grid
from src.droplet import Droplet
from src.direction import Direction


class TestEngine:
    """Test class for engine verification."""

    def __init__(self):
        """Initialize test suite."""
        self.results = []

    def log_result(self, test_name: str, passed: bool, details: str = ""):
        """Log a test result."""
        status = "PASS" if passed else "FAIL"
        result = f"[{status}] {test_name}"
        if details:
            result += f": {details}"
        self.results.append(result)
        print(result)

    def test_single_droplet_movement(self):
        """Test 1: Single droplet movement on empty grid."""
        print("\n=== Test 1: Single Droplet Movement ===")

        # Create a 5x5 empty grid
        grid = Grid(5, 5)
        engine = Engine(grid)

        # Create a droplet at (2, 2) moving RIGHT
        droplet = Droplet(42, 2, 2, Direction.RIGHT)
        engine.add_droplet(droplet)

        # Verify initial state
        initial_x, initial_y = droplet.x, droplet.y
        self.log_result("Initial position", droplet.x == 2 and droplet.y == 2, f"Expected (2, 2), got ({droplet.x}, {droplet.y})")

        # Verify droplet is in active list
        active_droplets = engine.get_active_droplets()
        self.log_result("Droplet in active list", len(active_droplets) == 1, f"Expected 1 droplet, got {len(active_droplets)}")

        # Execute one tick
        engine.tick()

        # Verify droplet moved right
        expected_x, expected_y = initial_x + 1, initial_y
        self.log_result("Movement occurred", droplet.x == expected_x and droplet.y == expected_y,
                       f"Expected ({expected_x}, {expected_y}), got ({droplet.x}, {droplet.y})")

        # Verify droplet still in active list
        active_droplets = engine.get_active_droplets()
        self.log_result("Droplet still active", len(active_droplets) == 1, f"Expected 1 droplet, got {len(active_droplets)}")

        # Execute another tick
        engine.tick()

        # Verify droplet moved right again
        expected_x, expected_y = initial_x + 2, initial_y
        self.log_result("Second movement", droplet.x == expected_x and droplet.y == expected_y,
                       f"Expected ({expected_x}, {expected_y}), got ({droplet.x}, {droplet.y})")

    def test_collision_destruction(self):
        """Test 2: Collision/destruction when hitting non-empty cell."""
        print("\n=== Test 2: Collision/Destruction ===")

        # Create a 5x5 grid with an obstacle
        grid = Grid(5, 5)
        engine = Engine(grid)

        # Place an obstacle at (2, 2) - 'X' is non-empty (droplet will hit this in first tick)
        grid._grid[2][2] = 'X'

        # Create a droplet at (1, 2) moving RIGHT (will hit obstacle at (2, 2))
        droplet = Droplet(99, 1, 2, Direction.RIGHT)
        engine.add_droplet(droplet)

        # Verify initial state
        active_droplets = engine.get_active_droplets()
        self.log_result("Initial droplet count", len(active_droplets) == 1, f"Expected 1 droplet, got {len(active_droplets)}")

        # Execute tick - should destroy droplet due to collision
        engine.tick()

        # Verify droplet was removed
        active_droplets = engine.get_active_droplets()
        self.log_result("Droplet destroyed on collision", len(active_droplets) == 0, f"Expected 0 droplets, got {len(active_droplets)}")

        # Test collision with different directions
        test_cases = [
            (Droplet(1, 0, 1, Direction.DOWN), (0, 2), 'X'),  # DOWN collision - obstacle at target position
            (Droplet(2, 3, 1, Direction.UP), (3, 0), 'X'),    # UP collision - obstacle at target position
            (Droplet(3, 1, 1, Direction.LEFT), (0, 1), 'X'),  # LEFT collision - obstacle at target position
        ]

        for i, (droplet, obs_pos, obs_char) in enumerate(test_cases, 1):
            # Reset grid and engine
            grid = Grid(5, 5)
            engine = Engine(grid)

            # Place obstacle
            grid._grid[obs_pos[1]][obs_pos[0]] = obs_char

            # Add droplet
            engine.add_droplet(droplet)

            # Execute tick
            engine.tick()

            # Verify destruction
            active_droplets = engine.get_active_droplets()
            self.log_result(f"Collision test {i}", len(active_droplets) == 0, f"Droplet should be destroyed, got {len(active_droplets)} active droplets")

    def test_boundary_destruction(self):
        """Test 3: Boundary collision destruction."""
        print("\n=== Test 3: Boundary Collision ===")

        # Test droplet moving out of bounds
        boundary_tests = [
            (Droplet(1, 0, 0, Direction.UP), "Top boundary"),
            (Droplet(2, 0, 4, Direction.DOWN), "Bottom boundary"),
            (Droplet(3, 0, 0, Direction.LEFT), "Left boundary"),
            (Droplet(4, 4, 0, Direction.RIGHT), "Right boundary"),
        ]

        for i, (droplet, description) in enumerate(boundary_tests, 1):
            grid = Grid(5, 5)
            engine = Engine(grid)
            engine.add_droplet(droplet)

            # Execute tick
            engine.tick()

            # Verify droplet was destroyed
            active_droplets = engine.get_active_droplets()
            self.log_result(f"Boundary test {i} ({description})", len(active_droplets) == 0,
                           f"Droplet should be destroyed at boundary, got {len(active_droplets)} active droplets")

    def test_state_management(self):
        """Test 4: State management of active droplets list."""
        print("\n=== Test 4: State Management ===")

        grid = Grid(10, 10)
        engine = Engine(grid)

        # Test adding multiple droplets
        droplets = [
            Droplet(1, 1, 1, Direction.RIGHT),
            Droplet(2, 2, 2, Direction.DOWN),
            Droplet(3, 3, 3, Direction.LEFT),
            Droplet(4, 4, 4, Direction.UP),
        ]

        for droplet in droplets:
            engine.add_droplet(droplet)

        active_droplets = engine.get_active_droplets()
        self.log_result("Multiple droplets added", len(active_droplets) == 4, f"Expected 4 droplets, got {len(active_droplets)}")

        # Test that get_active_droplets returns a copy (not reference)
        active_droplets.clear()
        engine_droplets = engine.get_active_droplets()
        self.log_result("Returns copy", len(engine_droplets) == 4, "get_active_droplets should return a copy")

        # Test is_empty method
        self.log_result("Not empty", not engine.is_empty(), "Engine should not be empty with 4 droplets")

        # Test removing specific droplet
        droplet_to_remove = droplets[1]
        engine.remove_droplet(droplet_to_remove)
        active_droplets = engine.get_active_droplets()
        self.log_result("Specific droplet removal", len(active_droplets) == 3, f"Expected 3 droplets, got {len(active_droplets)}")

        # Clear all droplets
        engine.droplets.clear()
        self.log_result("Engine empty after clear", engine.is_empty(), "Engine should be empty after clearing all droplets")

    def test_multiple_droplets_simulation(self):
        """Test 5: Multiple droplets moving simultaneously."""
        print("\n=== Test 5: Multiple Droplets Simulation ===")

        grid = Grid(10, 10)
        engine = Engine(grid)

        # Create multiple droplets that won't collide
        droplets = [
            Droplet(1, 1, 1, Direction.RIGHT),  # Will move to (2, 1)
            Droplet(2, 8, 1, Direction.LEFT),   # Will move to (7, 1)
            Droplet(3, 1, 8, Direction.UP),     # Will move to (1, 7)
            Droplet(4, 8, 8, Direction.DOWN),   # Will move to (8, 9)
        ]

        for droplet in droplets:
            engine.add_droplet(droplet)

        # Record initial positions
        initial_positions = [(d.x, d.y) for d in droplets]

        # Execute one tick
        engine.tick()

        # Verify all droplets moved correctly
        active_droplets = engine.get_active_droplets()
        self.log_result("All droplets survived", len(active_droplets) == 4, f"Expected 4 droplets, got {len(active_droplets)}")

        # Check each droplet moved correctly
        for i, (initial_pos, droplet) in enumerate(zip(initial_positions, active_droplets)):
            expected_x, expected_y = initial_pos
            if droplet.direction == Direction.RIGHT:
                expected_x += 1
            elif droplet.direction == Direction.LEFT:
                expected_x -= 1
            elif droplet.direction == Direction.DOWN:
                expected_y += 1
            elif droplet.direction == Direction.UP:
                expected_y -= 1

            moved_correctly = (droplet.x == expected_x and droplet.y == expected_y)
            self.log_result(f"Droplet {i+1} movement", moved_correctly,
                           f"Expected ({expected_x}, {expected_y}), got ({droplet.x}, {droplet.y})")

    def test_edge_cases(self):
        """Test 6: Edge cases and boundary conditions."""
        print("\n=== Test 6: Edge Cases ===")

        # Test with 1x1 grid
        grid = Grid(1, 1)
        engine = Engine(grid)
        droplet = Droplet(1, 0, 0, Direction.RIGHT)
        engine.add_droplet(droplet)

        engine.tick()
        active_droplets = engine.get_active_droplets()
        self.log_result("1x1 grid boundary", len(active_droplets) == 0, "Droplet should be destroyed in 1x1 grid")

        # Test with empty grid (0x0)
        grid = Grid(0, 0)
        engine = Engine(grid)
        droplet = Droplet(1, 0, 0, Direction.RIGHT)
        engine.add_droplet(droplet)

        engine.tick()
        active_droplets = engine.get_active_droplets()
        self.log_result("0x0 grid handling", len(active_droplets) == 0, "Droplet should be destroyed in 0x0 grid")

        # Test removing non-existent droplet
        grid = Grid(5, 5)
        engine = Engine(grid)
        fake_droplet = Droplet(999, 999, 999, Direction.UP)

        # Should not raise an exception
        try:
            engine.remove_droplet(fake_droplet)
            self.log_result("Remove non-existent droplet", True, "Should handle gracefully")
        except Exception as e:
            self.log_result("Remove non-existent droplet", False, f"Should not raise exception: {e}")

    def run_all_tests(self):
        """Run all verification tests."""
        print("Starting Execution Engine Verification Tests")
        print("=" * 50)

        self.test_single_droplet_movement()
        self.test_collision_destruction()
        self.test_boundary_destruction()
        self.test_state_management()
        self.test_multiple_droplets_simulation()
        self.test_edge_cases()

        print("\n" + "=" * 50)
        print("VERIFICATION RESULTS SUMMARY")
        print("=" * 50)

        passed = sum(1 for result in self.results if result.startswith("[PASS]"))
        total = len(self.results)

        for result in self.results:
            print(result)

        print(f"\nOverall: {passed}/{total} tests passed")

        if passed == total:
            print("✅ All tests PASSED - Engine implementation meets acceptance criteria!")
        else:
            print(f"❌ {total - passed} tests FAILED - Engine implementation needs fixes!")

        return passed == total


def main():
    """Main function to run tests."""
    tester = TestEngine()
    success = tester.run_all_tests()
    return 0 if success else 1


if __name__ == "__main__":
    exit(main())