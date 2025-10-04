import unittest
from src.grid import Grid
from src.droplet import Droplet
from src.direction import Direction
from src.engine import Engine

class TestStep7Verification(unittest.TestCase):
    def test_forward_slash_corner_zero_value(self):
        # Test '/' with zero value droplet
        grid = Grid(3, 3)
        grid._grid[1][1] = '/'
        
        # From UP
        droplet = Droplet(0, 1, 0, Direction.DOWN)
        engine = Engine(grid)
        engine.droplets = [droplet]
        engine.tick()
        self.assertEqual(droplet.direction, Direction.LEFT)

        # From DOWN
        grid = Grid(3, 3)
        grid._grid[1][1] = '/'
        droplet = Droplet(0, 1, 2, Direction.UP)
        engine = Engine(grid)
        engine.droplets = [droplet]
        engine.tick()
        self.assertEqual(droplet.direction, Direction.RIGHT)

        # From LEFT
        grid = Grid(3, 3)
        grid._grid[1][1] = '/'
        droplet = Droplet(0, 0, 1, Direction.RIGHT)
        engine = Engine(grid)
        engine.droplets = [droplet]
        engine.tick()
        self.assertEqual(droplet.direction, Direction.DOWN)

        # From RIGHT
        grid = Grid(3, 3)
        grid._grid[1][1] = '/'
        droplet = Droplet(0, 2, 1, Direction.LEFT)
        engine = Engine(grid)
        engine.droplets = [droplet]
        engine.tick()
        self.assertEqual(droplet.direction, Direction.UP)

    def test_forward_slash_corner_non_zero_value(self):
        # Test '/' with non-zero value droplet
        grid = Grid(3, 3)
        grid._grid[1][1] = '/'
        
        # From UP
        droplet = Droplet(42, 1, 0, Direction.DOWN)
        engine = Engine(grid)
        engine.droplets = [droplet]
        engine.tick()
        self.assertEqual(droplet.direction, Direction.RIGHT)

        # From DOWN
        grid = Grid(3, 3)
        grid._grid[1][1] = '/'
        droplet = Droplet(42, 1, 2, Direction.UP)
        engine = Engine(grid)
        engine.droplets = [droplet]
        engine.tick()
        self.assertEqual(droplet.direction, Direction.LEFT)

        # From LEFT
        grid = Grid(3, 3)
        grid._grid[1][1] = '/'
        droplet = Droplet(42, 0, 1, Direction.RIGHT)
        engine = Engine(grid)
        engine.droplets = [droplet]
        engine.tick()
        self.assertEqual(droplet.direction, Direction.UP)

        # From RIGHT
        grid = Grid(3, 3)
        grid._grid[1][1] = '/'
        droplet = Droplet(42, 2, 1, Direction.LEFT)
        engine = Engine(grid)
        engine.droplets = [droplet]
        engine.tick()
        self.assertEqual(droplet.direction, Direction.DOWN)

    def test_back_slash_corner(self):
        # Test '\\' with any value droplet
        grid = Grid(3, 3)
        grid._grid[1][1] = '\\'
        
        # From UP
        droplet = Droplet(0, 1, 0, Direction.DOWN)
        engine = Engine(grid)
        engine.droplets = [droplet]
        engine.tick()
        self.assertEqual(droplet.direction, Direction.RIGHT)

        # From DOWN
        grid = Grid(3, 3)
        grid._grid[1][1] = '\\'
        droplet = Droplet(0, 1, 2, Direction.UP)
        engine = Engine(grid)
        engine.droplets = [droplet]
        engine.tick()
        self.assertEqual(droplet.direction, Direction.LEFT)

        # From LEFT
        grid = Grid(3, 3)
        grid._grid[1][1] = '\\'
        droplet = Droplet(0, 0, 1, Direction.RIGHT)
        engine = Engine(grid)
        engine.droplets = [droplet]
        engine.tick()
        self.assertEqual(droplet.direction, Direction.DOWN)

        # From RIGHT
        grid = Grid(3, 3)
        grid._grid[1][1] = '\\'
        droplet = Droplet(0, 2, 1, Direction.LEFT)
        engine = Engine(grid)
        engine.droplets = [droplet]
        engine.tick()
        self.assertEqual(droplet.direction, Direction.UP)

if __name__ == '__main__':
    unittest.main()