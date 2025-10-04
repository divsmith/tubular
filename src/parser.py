#!/usr/bin/env python3
"""
Main parser components for the Tubular programming language.

This module implements the complete parser system including:
- Lexer: Tokenizes 2D grid input
- Parser: Validates structure and builds flow graphs
- ParsedProgram: Structured representation of parsed program
- TubularParser: Main orchestrator class
"""

from typing import List, Dict, Set, Tuple, Optional, Any
from dataclasses import dataclass, field
from collections import defaultdict, deque
import sys
import os

# Import our modules
try:
    # Try relative imports (when run as module)
    from .grid import Grid
    from .tokens import Token, TokenType, Position, TokenMapping
    from .errors import ErrorReporter, TubularError, LexicalError, SyntaxError, SemanticError
except ImportError:
    # Fall back to absolute imports (when run as script)
    from grid import Grid
    from tokens import Token, TokenType, Position, TokenMapping
    from errors import ErrorReporter, TubularError, LexicalError, SyntaxError, SemanticError


@dataclass
class FlowNode:
    """Represents a node in the program flow graph."""
    position: Position
    token: Token
    outgoing: List[Position] = field(default_factory=list)
    incoming: List[Position] = field(default_factory=list)


@dataclass
class ParsedProgram:
    """Structured representation of a parsed Tubular program."""

    # Grid dimensions and entry point
    width: int
    height: int
    entry_point: Optional[Position] = None

    # All operators in the program
    operators: Dict[Position, Token] = field(default_factory=dict)

    # Flow graph representation
    flow_graph: Dict[Position, FlowNode] = field(default_factory=dict)

    # Reachability information
    reachable_positions: Set[Position] = field(default_factory=set)
    unreachable_positions: Set[Position] = field(default_factory=set)

    # Metadata
    has_valid_structure: bool = False
    parsing_errors: List[str] = field(default_factory=list)

    def get_operator_at(self, x: int, y: int) -> Optional[Token]:
        """Get the token at a specific position."""
        position = Position(x, y)
        return self.operators.get(position)

    def is_reachable(self, x: int, y: int) -> bool:
        """Check if a position is reachable from the entry point."""
        position = Position(x, y)
        return position in self.reachable_positions

    def get_flow_paths(self) -> List[List[Position]]:
        """Get all possible execution paths through the program."""
        if not self.entry_point or not self.has_valid_structure:
            return []

        paths = []
        self._build_flow_paths(self.entry_point, [], paths)
        return paths

    def _build_flow_paths(self, current: Position, path: List[Position], all_paths: List[List[Position]]) -> None:
        """Recursively build all flow paths from current position."""
        if current in path:
            # Loop detected
            return

        path = path + [current]

        if current not in self.flow_graph:
            # End of path
            all_paths.append(path)
            return

        node = self.flow_graph[current]
        for next_pos in node.outgoing:
            self._build_flow_paths(next_pos, path, all_paths)


class Lexer:
    """Lexical analyzer for Tubular programs."""

    def __init__(self, grid: Grid, error_reporter: ErrorReporter):
        self.grid = grid
        self.error_reporter = error_reporter
        self.tokens: List[Token] = []
        self.current_x = 0
        self.current_y = 0

    def tokenize(self) -> List[Token]:
        """Tokenize the entire grid."""
        self.tokens = []

        for y in range(self.grid.height):
            x = 0
            while x < self.grid.width:
                char = self.grid.get(x, y)

                if char == ' ':
                    x += 1
                    continue  # Skip whitespace

                position = Position(x, y)

                # Handle ?? (numeric input) as special case
                if char == '?' and x + 1 < self.grid.width and self.grid.get(x + 1, y) == '?':
                    token = Token(
                        type=TokenType.NUMERIC_INPUT,
                        value='??',
                        position=position
                    )
                    self.tokens.append(token)
                    x += 2  # Skip both characters
                else:
                    token = self._create_token(char, position)

                    if token.type == TokenType.INVALID:
                        self.error_reporter.report_lexical_error(
                            f"Invalid character '{char}'", position
                        )
                    else:
                        self.tokens.append(token)
                    x += 1

        return self.tokens

    def _create_token(self, char: str, position: Position) -> Token:
        """Create a token from a character at a position."""
        if TokenMapping.is_number(char):
            return Token(
                type=TokenType.NUMBER,
                value=char,
                position=position,
                number_value=int(char)
            )
        else:
            token_type = TokenMapping.get_token_type(char)
            return Token(
                type=token_type,
                value=char,
                position=position
            )


class Parser:
    """Parser for structural validation and flow graph construction."""

    def __init__(self, grid: Grid, tokens: List[Token], error_reporter: ErrorReporter):
        self.grid = grid
        self.tokens = tokens
        self.error_reporter = error_reporter
        self.token_positions: Dict[Position, Token] = {}
        self.entry_points: List[Position] = []

        # Build token position map
        for token in tokens:
            if token.position:
                self.token_positions[token.position] = token
                if token.type == TokenType.PROGRAM_START:
                    self.entry_points.append(token.position)

    def parse(self) -> ParsedProgram:
        """Parse the program and return structured representation."""
        program = ParsedProgram(
            width=self.grid.width,
            height=self.grid.height
        )

        # Add all operators to program
        for position, token in self.token_positions.items():
            program.operators[position] = token

        # Validate structure
        self._validate_structure(program)

        if not self.error_reporter.has_errors():
            # Build flow graph
            self._build_flow_graph(program)

            # Analyze reachability
            self._analyze_reachability(program)

            program.has_valid_structure = True

        # Collect error messages
        if self.error_reporter.has_errors():
            program.parsing_errors = [str(error) for error in self.error_reporter.errors]

        return program

    def _validate_structure(self, program: ParsedProgram) -> None:
        """Validate basic structural requirements."""
        # Check for exactly one entry point
        if len(self.entry_points) == 0:
            self.error_reporter.report_semantic_error("No entry point (@) found")
        elif len(self.entry_points) > 1:
            self.error_reporter.report_multiple_entry_points(self.entry_points)

        # Set entry point if exactly one exists
        if len(self.entry_points) == 1:
            program.entry_point = self.entry_points[0]

    def _build_flow_graph(self, program: ParsedProgram) -> None:
        """Build the flow graph showing how droplets can move through the program."""
        if not program.entry_point:
            return

        # Initialize flow graph
        for position in program.operators.keys():
            program.flow_graph[position] = FlowNode(position, program.operators[position])

        # Build connections based on operator types and positions
        positions = list(program.operators.keys())
        positions.sort()  # Process in consistent order

        for position in positions:
            self._add_flow_connections(position, program)

    def _add_flow_connections(self, position: Position, program: ParsedProgram) -> None:
        """Add flow connections for a specific position."""
        token = program.operators[position]
        node = program.flow_graph[position]

        # For now, implement simple linear flow for testing
        # In a real implementation, this would be more sophisticated
        x, y = position.x, position.y

        # Simple flow logic: try to connect to adjacent positions
        adjacent_positions = [
            Position(x, y - 1),  # Up
            Position(x, y + 1),  # Down
            Position(x - 1, y),  # Left
            Position(x + 1, y),  # Right
        ]

        for adj_pos in adjacent_positions:
            if adj_pos in program.operators and adj_pos != position:
                # Connect the nodes
                node.outgoing.append(adj_pos)
                program.flow_graph[adj_pos].incoming.append(position)

    def _get_possible_exits(self, position: Position, token: Token) -> List[Position]:
        """Get possible exit positions from a given position and token."""
        exits = []
        x, y = position.x, position.y

        # Default behavior depends on token type
        if token.type in [TokenType.VERTICAL_PIPE, TokenType.PROGRAM_START]:
            # Can go up or down
            if y > 0:
                exits.append(Position(x, y - 1))  # Up
            if y < self.grid.height - 1:
                exits.append(Position(x, y + 1))  # Down

        elif token.type == TokenType.HORIZONTAL_PIPE:
            # Can go left or right
            if x > 0:
                exits.append(Position(x - 1, y))  # Left
            if x < self.grid.width - 1:
                exits.append(Position(x + 1, y))  # Right

        elif token.type == TokenType.GO_UP_PIPE:
            # Forces direction up
            if y > 0:
                exits.append(Position(x, y - 1))  # Up only

        elif token.type == TokenType.WALL:
            # No exits - droplet is destroyed
            pass

        elif token.type in [TokenType.FORWARD_SLASH, TokenType.BACK_SLASH]:
            # Corner pipes - direction depends on entry direction
            # For now, assume basic connectivity (can be enhanced)
            exits.append(Position(x, y))  # Placeholder

        # Add more specific logic for other operators as needed

        return exits

    def _analyze_reachability(self, program: ParsedProgram) -> None:
        """Analyze which positions are reachable from the entry point."""
        if not program.entry_point:
            return

        # Use BFS to find all reachable positions
        visited = set()
        queue = deque([program.entry_point])

        while queue:
            current = queue.popleft()
            if current in visited:
                continue

            visited.add(current)

            # Add all connected positions to queue
            if current in program.flow_graph:
                node = program.flow_graph[current]
                for next_pos in node.outgoing:
                    if next_pos not in visited:
                        queue.append(next_pos)

        program.reachable_positions = visited

        # Find unreachable positions
        all_positions = set(program.operators.keys())
        program.unreachable_positions = all_positions - visited

        # Report unreachable code if found
        if program.unreachable_positions:
            unreachable_list = list(program.unreachable_positions)
            self.error_reporter.report_unreachable_code(unreachable_list)


class TubularParser:
    """Main orchestrator for the Tubular parsing system."""

    def __init__(self, error_reporter: Optional[ErrorReporter] = None):
        self.error_reporter = error_reporter or ErrorReporter()

    def parse_from_file(self, filepath: str) -> ParsedProgram:
        """
        Parse a Tubular program from a file.

        Args:
            filepath: Path to the .tub file

        Returns:
            ParsedProgram instance representing the parsed program

        Raises:
            FileNotFoundError: If the file doesn't exist
        """
        # Load grid using existing Grid class
        grid = Grid.from_file(filepath)

        return self.parse_from_grid(grid)

    def parse_from_grid(self, grid: Grid) -> ParsedProgram:
        """
        Parse a Tubular program from a Grid object.

        Args:
            grid: Grid object containing the program

        Returns:
            ParsedProgram instance representing the parsed program
        """
        # Step 1: Lexical analysis
        lexer = Lexer(grid, self.error_reporter)
        tokens = lexer.tokenize()

        # Step 2: Parsing and structural validation
        parser = Parser(grid, tokens, self.error_reporter)
        program = parser.parse()

        return program

    def parse_from_string(self, program_text: str) -> ParsedProgram:
        """
        Parse a Tubular program from a string representation.

        Args:
            program_text: String containing the program grid

        Returns:
            ParsedProgram instance representing the parsed program
        """
        # Create a temporary grid from the string
        lines = program_text.strip().split('\n')
        height = len(lines)
        width = max(len(line) for line in lines) if lines else 0

        grid = Grid(width, height)
        for y, line in enumerate(lines):
            for x, char in enumerate(line):
                if char != ' ':
                    # Use direct grid access for simplicity
                    grid._grid[y][x] = char

        return self.parse_from_grid(grid)

    def validate_program(self, program: ParsedProgram) -> bool:
        """
        Validate a parsed program for structural correctness.

        Args:
            program: The ParsedProgram to validate

        Returns:
            True if the program is valid, False otherwise
        """
        return program.has_valid_structure and not self.error_reporter.has_errors()

    def get_errors(self) -> List[TubularError]:
        """Get all parsing errors."""
        return self.error_reporter.errors.copy()

    def get_warnings(self) -> List[TubularError]:
        """Get all parsing warnings."""
        return self.error_reporter.warnings.copy()

    def print_errors(self) -> None:
        """Print all parsing errors."""
        self.error_reporter.print_errors()

    def print_warnings(self) -> None:
        """Print all parsing warnings."""
        self.error_reporter.print_warnings()

    def clear_errors(self) -> None:
        """Clear all parsing errors and warnings."""
        self.error_reporter.clear()