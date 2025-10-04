#!/usr/bin/env python3
"""
Tubular Language Compiler/Interpreter

This is the main compiler and interpreter for the Tubular programming language.
This is a placeholder implementation created during Step 0 of the project setup.

As the implementation progresses through the steps outlined in implementation_checklist.md,
this file will be developed to include:

- Grid and Droplet data structures
- Execution engine with tick-based processing
- Pipe character handling and flow control
- Stack operations and arithmetic
- Input/output capabilities
- Reservoir (memory) management
- Subroutine support

Current Status: Step 4 - Program Start & Basic Output Complete
Next: Step 5 - Unary Operators Implementation
"""

import sys
import argparse

# Import core data structures for Step 1
try:
    # Try relative imports (when run as module)
    from .grid import Grid
    from .droplet import Droplet
    from .direction import Direction
    from .engine import Engine
    from .parser import TubFileParser
except ImportError:
    # Fall back to absolute imports (when run as script)
    from grid import Grid
    from droplet import Droplet
    from direction import Direction
    from engine import Engine
    from parser import TubFileParser


def main():
    """Main entry point for the Tubular compiler."""
    parser = argparse.ArgumentParser(description='Tubular Language Compiler/Interpreter')
    parser.add_argument('file', nargs='?', help='Tubular source file to execute')
    parser.add_argument('--version', action='version', version='Tubular Compiler v0.1.0')

    args = parser.parse_args()

    if not args.file:
        print("Tubular Language Compiler/Interpreter")
        print("=====================================")
        print()
        print("Usage: python compiler.py <file.tub>")
        print()
        print("Step 4 Complete: Program Start (@) and Numeric Output (n) implemented.")
        print("The execution engine is now functional with basic program execution.")
        print()
        print("See implementation_checklist.md for the development plan.")
        return 0

    try:
        # Load the grid from file using TubFileParser
        parser = TubFileParser()
        grid = parser.parse_file(args.file)
        print(f"Loaded grid from: {args.file}")

        # Create engine and run the program
        engine = Engine(grid)
        print("Executing program...")

        # Run the execution loop until no droplets remain
        tick_count = 0
        while not engine.is_empty():
            engine.tick()
            tick_count += 1
            # Prevent infinite loops (safety measure)
            if tick_count > 10000:
                print("Execution terminated: maximum tick limit reached")
                break

        print("Program execution completed.")
        return 0

    except FileNotFoundError:
        print(f"Error: File '{args.file}' not found.")
        return 1
    except Exception as e:
        print(f"Error executing program: {e}")
        return 1


if __name__ == "__main__":
    sys.exit(main())