"""
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
        with open(filename, 'r') as file:
            grid_str = file.read()
        
        grid = Grid(grid_str)
        interpreter = TubularInterpreter(grid)
        output = interpreter.execute_program(max_ticks)
        
        print(output, end='')
        
    except FileNotFoundError:
        print(f'Error: File "{filename}" not found.', file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error running Tubular program: {e}", file=sys.stderr)
        sys.exit(1)


def main():
    """
    Main entry point for the Tubular CLI.
    """
    parser = argparse.ArgumentParser(description='Tubular Programming Language Interpreter')
    parser.add_argument('file', help='Path to the .tub file to execute')
    parser.add_argument('--max-ticks', type=int, default=10000,
                        help='Maximum number of execution ticks (default: 10000)')
    
    args = parser.parse_args()
    
    run_tubular_file(args.file, args.max_ticks)


if __name__ == '__main__':
    main()