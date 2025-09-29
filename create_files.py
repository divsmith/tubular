#!/usr/bin/env python3
"""
Script to create the Tubular interpreter files with correct formatting.
"""

def create_grid_file():
    content = '''"""\
Grid data structure for the Tubular programming language.
The grid represents the 2D pipe system through which droplets flow.
"""
from typing import List, Tuple, Optional


class Grid:
    """
    A 2D grid representing the Tubular program.
    Each cell contains a character that defines the pipe behavior.
    """
    
    def __init__(self, grid_str: str):
        """
        Initialize the grid from a string representation.
        
        Args:
            grid_str: A multi-line string representing the grid
        """
        lines = grid_str.split('\n')
        # Remove potentially empty last line if the string ends with a newline
        if lines and lines[-1] == '':
            lines = lines[:-1]
        self.width = max(len(line) for line in lines) if lines else 0
        self.height = len(lines)
        
        # Pad lines to ensure consistent width
        self.grid: List[List[str]] = []
        for line in lines:
            padded_line = list(line.ljust(self.width))
            self.grid.append(padded_line)
        
        # Initialize the reservoir (2D memory grid)
        # Using a dictionary to store values for sparse memory representation
        self.reservoir: dict = {}
        
        # Find the starting position of the program ('@')
        self.start_pos: Optional[Tuple[int, int]] = None
        for y in range(self.height):
            for x in range(self.width):
                if self.grid[y][x] == "@":
                    self.start_pos = (x, y)
                    break
            if self.start_pos:
                break
        
        if not self.start_pos:
            raise ValueError("No starting position "@" found in the grid")
    
    def get_cell(self, x: int, y: int) -> str:
        """
        Get the character at position (x, y) in the grid.
        
        Args:
            x: X coordinate (column)
            y: Y coordinate (row)
            
        Returns:
            The character at the specified position, or " " (space) if out of bounds
        """
        if 0 <= y < self.height and 0 <= x < self.width:
            return self.grid[y][x]
        return " "  # Treat out-of-bounds as empty space
    
    def set_cell(self, x: int, y: int, char: str):
        """
        Set the character at position (x, y) in the grid.
        
        Args:
            x: X coordinate (column)
            y: Y coordinate (row)
            char: Character to set
        """
        if 0 <= y < self.height and 0 <= x < self.width:
            self.grid[y][x] = char[0] if char else " "
    
    def get_reservoir_value(self, x: int, y: int) -> int:
        """
        Get the value at position (x, y) in the reservoir.
        
        Args:
            x: X coordinate in the reservoir
            y: Y coordinate in the reservoir
            
        Returns:
            The value at the specified position, or 0 if not set
        """
        return self.reservoir.get((x, y), 0)
    
    def set_reservoir_value(self, x: int, y: int, value: int):
        """
        Set the value at position (x, y) in the reservoir.
        
        Args:
            x: X coordinate in the reservoir
            y: Y coordinate in the reservoir
            value: Value to set
        """
        self.reservoir[(x, y)] = value
    
    def __str__(self):
        """
        String representation of the grid.
        
        Returns:
            A string showing the grid layout
        """
        return "\\n".join("".join(row) for row in self.grid)
'''
    with open('src/grid.py', 'w') as f:
        f.write(content)


def create_droplet_file():
    content = '''"""\
Droplet data structure for the Tubular programming language.
A droplet represents a unit of data with a value and direction.
"""
from enum import Enum
from typing import Tuple


class Direction(Enum):
    """
    Direction in which a droplet is traveling.
    """
    UP = (-1, 0)
    DOWN = (1, 0)
    LEFT = (0, -1)
    RIGHT = (0, 1)


class Droplet:
    """
    A droplet in the Tubular programming language.
    Each droplet has a value and a direction of travel.
    """
    
    def __init__(self, value: int = 0, direction: Direction = Direction.DOWN):
        """
        Initialize a droplet.
        
        Args:
            value: Initial value of the droplet (default 0)
            direction: Initial direction of travel (default DOWN)
        """
        self.value = value
        self.direction = direction
    
    def move(self) -> Tuple[int, int]:
        """
        Move the droplet one cell in its current direction.
        
        Returns:
            A tuple (dy, dx) representing the change in position
        """
        dy, dx = self.direction.value
        return dy, dx
    
    def change_direction(self, new_direction: Direction):
        """
        Change the direction of the droplet.
        
        Args:
            new_direction: The new direction for the droplet
        """
        self.direction = new_direction
    
    def __repr__(self):
        """
        String representation of the droplet.
        
        Returns:
            A string showing the value and direction of the droplet
        """
        return f"Droplet(value={self.value}, direction={self.direction.name})"
'''
    with open('src/droplet.py', 'w') as f:
        f.write(content)


def create_interpreter_file():
    content = '''"""\
Main Tubular interpreter implementation.
This module handles the execution loop and all pipe operators.
"""
from typing import List, Tuple, Optional
from collections import deque
from .grid import Grid
from .droplet import Droplet, Direction


class TubularInterpreter:
    """
    The main Tubular interpreter that executes programs on a grid.
    """
    
    def __init__(self, grid: Grid):
        """
        Initialize the interpreter with a grid.
        
        Args:
            grid: The grid representing the Tubular program
        """
        self.grid = grid
        self.droplets: List[Droplet] = []
        self.tick_count = 0
        
        # Initialize the data stack
        self.data_stack: List[int] = []
        
        # Initialize the call stack
        self.call_stack: List[Tuple[int, int, Direction]] = []  # (x, y, direction)
        
        # For output handling
        self.output_buffer = []
        self.ascii_mode = False  # Whether we're in ASCII output mode
        
    def add_droplet(self, x: int, y: int, droplet: Droplet) -> bool:
        """
        Add a droplet at position (x, y) to the simulation.
        Prevents collisions by checking if a droplet already exists at that position.
        
        Args:
            x: X coordinate
            y: Y coordinate
            droplet: The droplet to add
            
        Returns:
            True if the droplet was added, False if there was a collision
        """
        # Check for collisions with existing droplets
        for existing_droplet_pos in [(d.x, d.y) for d in self.droplets]:
            if (x, y) == existing_droplet_pos:
                return False  # Collision - don't add the droplet
        
        # Attach coordinates to the droplet for tracking purposes
        droplet.x = x
        droplet.y = y
        self.droplets.append(droplet)
        return True
    
    def execute_program(self, max_ticks: int = 10000) -> str:
        """
        Execute the Tubular program until completion or max_ticks reached.
        
        Args:
            max_ticks: Maximum number of ticks to execute before stopping
            
        Returns:
            The output of the program
        """
        # Add the initial droplet at the start position
        start_x, start_y = self.grid.start_pos
        initial_droplet = Droplet(value=0, direction=Direction.DOWN)
        self.add_droplet(start_x, start_y, initial_droplet)
        
        while self.droplets and self.tick_count < max_ticks:
            self.tick()
            self.tick_count += 1
            
            # Prevent infinite loops by setting a maximum execution time
            if self.tick_count >= max_ticks:
                print(f"Execution stopped after {max_ticks} ticks to prevent infinite loop")
                break
        
        return "".join(self.output_buffer)
    
    def tick(self):
        """
        Execute one tick of the simulation.
        Each droplet moves one cell in its current direction.
        """
        # Store new droplets created during this tick to avoid modifying the list during iteration
        new_droplets = []
        
        # Process each droplet
        for droplet in self.droplets[:]:  # Create a copy to iterate over
            old_x, old_y = droplet.x, droplet.y
            
            # Calculate new position based on direction
            dy, dx = droplet.move()
            new_x = droplet.x + dx
            new_y = droplet.y + dy
            
            # Update droplet position
            droplet.x = new_x
            droplet.y = new_y
            
            # Get the character at the new position
            char = self.grid.get_cell(new_x, new_y)
            
            # Process the character according to the specification
            if char == " ":
                # Droplet moves into empty space - it's destroyed
                self.droplets.remove(droplet)
            else:
                # Process the pipe operator
                result = self._process_char(droplet, char, new_x, new_y)
                
                if result is False:
                    # Droplet was destroyed by the operator
                    if droplet in self.droplets:
                        self.droplets.remove(droplet)
                elif isinstance(result, Droplet):
                    # The droplet was replaced by a new one
                    if droplet in self.droplets:
                        self.droplets.remove(droplet)
                    # Add the new droplet if no collision
                    if not self.add_droplet(new_x, new_y, result):
                        # Collision occurred, destroy the new droplet
                        pass
                elif isinstance(result, list):
                    # Multiple new droplets were created
                    if droplet in self.droplets:
                        self.droplets.remove(droplet)
                    
                    for new_droplet in result:
                        # Add each new droplet if no collision
                        if not self.add_droplet(new_x, new_y, new_droplet):
                            # Collision occurred, destroy the new droplet
                            pass
                else:
                    # Continue with the same droplet
                    # Update position in case it was changed
                    pass
    
    def _process_char(self, droplet: Droplet, char: str, x: int, y: int) -> Optional[Droplet]:
        """
        Process a character in the grid and apply its effect to the droplet.
        
        Args:
            droplet: The droplet entering the cell
            char: The character in the cell
            x: X coordinate of the cell
            y: Y coordinate of the cell
            
        Returns:
            Either False (droplet destroyed), a new droplet (replacing the old),
            a list of new droplets, or None (continue with same droplet).
        """
        if char == "|":
            # Vertical Pipe: No change in behavior based on entry direction
            # Droplet just continues in its current direction
            return None
        elif char == "-":
            # Horizontal Pipe: No change in behavior based on entry direction
            # Droplet just continues in its current direction
            return None
        elif char == "^":
            # Go Up Pipe: Any droplet entering this pipe has its direction changed to Up
            droplet.change_direction(Direction.UP)
            return None
        elif char == "#":
            # Wall: Stops droplet's movement, effectively destroying it
            return False
        elif char == "/":
            # Forward-Slash Corner
            if droplet.direction == Direction.UP:
                # Conditional Branch: If droplet value is 0, direction becomes Left. 
                # If non-zero, direction becomes Right.
                if droplet.value == 0:
                    droplet.change_direction(Direction.LEFT)
                else:
                    droplet.change_direction(Direction.RIGHT)
            elif droplet.direction == Direction.DOWN:
                # Conditional Branch: If droplet value is 0, direction becomes Right. 
                # If non-zero, direction becomes Left.
                if droplet.value == 0:
                    droplet.change_direction(Direction.RIGHT)
                else:
                    droplet.change_direction(Direction.LEFT)
            elif droplet.direction == Direction.LEFT:
                # Redirects Up
                droplet.change_direction(Direction.UP)
            elif droplet.direction == Direction.RIGHT:
                # Redirects Down
                droplet.change_direction(Direction.DOWN)
            return None
        elif char == "\\\\":
            # Back-Slash Corner
            if droplet.direction == Direction.UP:
                # Conditional Branch: If droplet value is 0, direction becomes Right.
                # If non-zero, direction becomes Left.
                if droplet.value == 0:
                    droplet.change_direction(Direction.RIGHT)
                else:
                    droplet.change_direction(Direction.LEFT)
            elif droplet.direction == Direction.DOWN:
                # Conditional Branch: If droplet value is 0, direction becomes Left.
                # If non-zero, direction becomes Right.
                if droplet.value == 0:
                    droplet.change_direction(Direction.LEFT)
                else:
                    droplet.change_direction(Direction.RIGHT)
            elif droplet.direction == Direction.LEFT:
                # Redirects Down
                droplet.change_direction(Direction.DOWN)
            elif droplet.direction == Direction.RIGHT:
                # Redirects Up
                droplet.change_direction(Direction.UP)
            return None
        elif char.isdigit():
            # Number Source: When a droplet hits a number character, that character emits 
            # a new droplet downwards with the corresponding integer value. 
            # The original droplet is destroyed.
            new_droplet = Droplet(value=int(char), direction=Direction.DOWN)
            return new_droplet
        elif char == "@":
            # Program Start: This should only happen at the beginning, 
            # but if a droplet somehow returns here, it just continues
            return None
        elif char == ">":
            # Tape Reader: When a droplet hits this character, it begins reading all 
            # adjacent characters to its right until it hits a whitespace or a pipe character.
            # For each character read, it emits a new droplet downwards containing that 
            # character's ASCII value. The original droplet is destroyed.
            new_droplets = []
            read_x = x + 1
            
            while True:
                next_char = self.grid.get_cell(read_x, y)
                # Stop reading if we encounter a pipe character or whitespace
                if next_char in " @|^-\\\\/#+" or next_char.isspace():
                    break
                # Add the ASCII value of the character as a new droplet
                new_droplet = Droplet(value=ord(next_char), direction=Direction.DOWN)
                new_droplets.append(new_droplet)
                read_x += 1
            
            return new_droplets
        elif char == "?":
            # Character Input: Halts execution and waits for user input.
            # For now, we'll read a single character from stdin and set the droplet's value
            # to its ASCII code. On EOF, set to -1.
            try:
                user_input = input("Enter a character: ")
                if user_input:
                    droplet.value = ord(user_input[0])
                else:
                    droplet.value = -1
            except EOFError:
                droplet.value = -1
            return None
        elif char == "!":
            # Output Sink: Consumes any droplet that enters it.
            # If the droplet was created by a Tape Reader, interpret as ASCII.
            # Otherwise, print the integer value followed by a newline.
            if self.ascii_mode:
                # If we're in ASCII mode, output the character
                self.output_buffer.append(chr(droplet.value))
                self.ascii_mode = False
            else:
                # Output integer value followed by newline
                self.output_buffer.append(f"{droplet.value}\\n")
            return False  # Droplet is consumed
        elif char == "+":
            # Increment: Adds 1 to the droplet's value
            droplet.value += 1
            return None
        elif char == "~":
            # Decrement: Subtracts 1 from the droplet's value
            droplet.value -= 1
            return None
        elif char == ":":
            # Push: The droplet's current value is pushed onto the data stack.
            # The droplet passes through unchanged.
            self.data_stack.append(droplet.value)
            return None
        elif char == ";":
            # Pop: A value is popped from the data stack.
            # The droplet's value is replaced with the popped value.
            # If the stack is empty, the droplet's value becomes 0.
            if self.data_stack:
                droplet.value = self.data_stack.pop()
            else:
                droplet.value = 0
            return None
        elif char == "d":
            # Duplicate: Pushes a copy of the droplet's current value onto the data stack.
            # The droplet passes through unchanged.
            self.data_stack.append(droplet.value)
            return None
        elif char == "A":
            # Add: Pops two values (a and b) from the data stack, calculates b + a,
            # and pushes the result back onto the stack. The droplet is destroyed.
            if len(self.data_stack) >= 2:
                a = self.data_stack.pop()
                b = self.data_stack.pop()
                result = b + a
                self.data_stack.append(result)
            return False  # Droplet is destroyed
        elif char == "S":
            # Subtract: Pops two values (a and b) from the data stack, calculates b - a,
            # and pushes the result back onto the stack. The droplet is destroyed.
            if len(self.data_stack) >= 2:
                a = self.data_stack.pop()
                b = self.data_stack.pop()
                result = b - a
                self.data_stack.append(result)
            return False  # Droplet is destroyed
        elif char == "M":
            # Multiply: Pops two values (a and b) from the data stack, calculates b * a,
            # and pushes the result back onto the stack. The droplet is destroyed.
            if len(self.data_stack) >= 2:
                a = self.data_stack.pop()
                b = self.data_stack.pop()
                result = b * a
                self.data_stack.append(result)
            return False  # Droplet is destroyed
        elif char == "D":
            # Divide: Pops two values (a and b) from the data stack, calculates b / a,
            # and pushes the result back onto the stack. The droplet is destroyed.
            if len(self.data_stack) >= 2:
                a = self.data_stack.pop()
                b = self.data_stack.pop()
                # Integer division
                if a != 0:
                    result = b // a
                else:
                    result = 0  # Avoid division by zero, set to 0 as a default
                self.data_stack.append(result)
            return False  # Droplet is destroyed
        elif char == "G":
            # Get: Pops a y then an x coordinate from the data stack.
            # Reads the value from The Reservoir at (x, y) and pushes it onto the data stack.
            if len(self.data_stack) >= 2:
                y = self.data_stack.pop()
                x = self.data_stack.pop()
                value = self.grid.get_reservoir_value(x, y)
                self.data_stack.append(value)
            return False  # Droplet is destroyed
        elif char == "P":
            # Put: Pops a y, an x, and a value from the data stack.
            # Writes the value to The Reservoir at (x, y).
            if len(self.data_stack) >= 3:
                y = self.data_stack.pop()
                x = self.data_stack.pop()
                value = self.data_stack.pop()
                self.grid.set_reservoir_value(x, y, value)
            return False  # Droplet is destroyed
        elif char == "C":
            # Call: Pops a y then an x coordinate from the data stack.
            # Pushes the current droplet's position and direction onto the call stack.
            # The droplet is then transported to the new (x, y) coordinates.
            if len(self.data_stack) >= 2:
                y = self.data_stack.pop()
                x = self.data_stack.pop()
                # Save current position and direction to call stack
                self.call_stack.append((droplet.x, droplet.y, droplet.direction))
                # Move the droplet to the new position
                droplet.x = x
                droplet.y = y
            return None
        elif char == "R":
            # Return: Pops a position and direction from the call stack.
            # The current droplet is destroyed, and a new one is created at the return location,
            # moving in the stored direction.
            if self.call_stack:
                x, y, direction = self.call_stack.pop()
                new_droplet = Droplet(value=droplet.value, direction=direction)
                # Update the current droplet to the new state
                droplet.value = new_droplet.value
                droplet.direction = new_droplet.direction
                droplet.x = x
                droplet.y = y
            else:
                # If the call stack is empty, just destroy the droplet
                return False
            return None
        else:
            # Unknown character - treat as empty space, destroy droplet
            return False
'''
    with open('src/interpreter.py', 'w') as f:
        f.write(content)


def create_cli_file():
    content = '''"""\
Command-line interface for the Tubular programming language.
"""
import sys
import argparse
from .grid import Grid
from .interpreter import TubularInterpreter


def run_tubular_file(filename: str, max_ticks: int = 10000):
    """
    Run a Tubular program from a file.
    
    Args:
        filename: Path to the .tub file
        max_ticks: Maximum number of ticks to execute before stopping
    """
    try:
        with open(filename, "r") as file:
            grid_str = file.read()
        
        grid = Grid(grid_str)
        interpreter = TubularInterpreter(grid)
        output = interpreter.execute_program(max_ticks)
        
        print(output, end="")
        
    print(f'Error: File "{filename}" not found.', file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error running Tubular program: {e}", file=sys.stderr)
        sys.exit(1)


def main():
    """
    Main entry point for the Tubular CLI.
    """
    parser = argparse.ArgumentParser(description="Tubular Programming Language Interpreter")
    parser.add_argument("file", help="Path to the .tub file to execute")
    parser.add_argument("--max-ticks", type=int, default=10000,
                        help="Maximum number of execution ticks (default: 10000)")
    
    args = parser.parse_args()
    
    run_tubular_file(args.file, args.max_ticks)


if __name__ == "__main__":
    main()
'''
    with open('src/cli.py', 'w') as f:
        f.write(content)


if __name__ == "__main__":
    import os
    os.makedirs("src", exist_ok=True)
    
    create_grid_file()
    create_droplet_file() 
    create_interpreter_file()
    create_cli_file()
    
    print("All Tubular interpreter files have been created successfully!")