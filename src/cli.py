"""
Command-line interface for the Tubular programming language.
"""
import sys
import argparse
from .grid import Grid
from .interpreter import TubularInterpreter
from .compiler import TubularCompiler


def run_tubular_file(filename: str, max_ticks: int = 10000):
    """
    Run a Tubular program from a file using the interpreter.
    
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


def compile_tubular_file(input_filename: str, output_filename: str):
    """
    Compile a Tubular program to WebAssembly.
    
    Args:
        input_filename: Path to the input .tub file
        output_filename: Path to the output .wat file
    """
    try:
        with open(input_filename, 'r') as file:
            grid_str = file.read()
        
        grid = Grid(grid_str)
        compiler = TubularCompiler()
        wat_code = compiler.compile(grid)
        
        with open(output_filename, 'w') as file:
            file.write(wat_code)
        
        print(f'Successfully compiled {input_filename} to {output_filename}')
        
    except FileNotFoundError:
        print(f'Error: File "{input_filename}" not found.', file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error compiling Tubular program: {e}", file=sys.stderr)
        sys.exit(1)


def main():
    """
    Main entry point for the Tubular CLI.
    """
    parser = argparse.ArgumentParser(description='Tubular Programming Language Toolkit')
    parser.add_argument('file', help='Path to the .tub file')
    
    # Add subparsers for different commands
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # Run command (default behavior)
    run_parser = subparsers.add_parser('run', help='Run a Tubular program with the interpreter')
    run_parser.add_argument('--max-ticks', type=int, default=10000,
                        help='Maximum number of execution ticks (default: 10000)')
    
    # Compile command
    compile_parser = subparsers.add_parser('compile', help='Compile a Tubular program to WASM')
    compile_parser.add_argument('-o', '--output', required=True,
                                help='Output .wat file path')
    
    args = parser.parse_args()
    
    if args.command == 'compile':
        compile_tubular_file(args.file, args.output)
    else:  # Default to run if no command specified or 'run' command
        max_ticks = getattr(args, 'max_ticks', 10000)
        run_tubular_file(args.file, max_ticks)


if __name__ == '__main__':
    main()