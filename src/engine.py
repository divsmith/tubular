#!/usr/bin/env python3
"""
Execution Engine for Tubular programming language.

Manages the execution of droplets on a grid during program execution.
"""

from typing import List, Dict, Any, Optional
try:
    from .grid import Grid
    from .droplet import Droplet
    from .direction import Direction
    from .parser import ParsedProgram
    from .tokens import Token, TokenType, Position, TokenMapping
except ImportError:
    from grid import Grid
    from droplet import Droplet
    from direction import Direction
    from parser import ParsedProgram
    from tokens import Token, TokenType, Position, TokenMapping


class Engine:
    """
    Execution engine for Tubular programs.

    Manages a grid and a collection of active droplets, executing
    movement and collision detection during each tick.
    """

    def __init__(self, grid: Grid, program: Optional[ParsedProgram] = None):
        """
        Initialize the execution engine with a grid and optional parsed program.

        Args:
            grid: The Grid instance representing the program
            program: Optional ParsedProgram instance for enhanced execution
        """
        self.grid = grid
        self.program = program
        self.droplets: List[Droplet] = []

        # Data structures for enhanced execution
        self.data_stack: List[int] = []  # Stack for data operations
        self.reservoir: Dict[str, int] = {}  # 2D memory system (keyed by "x,y")
        self.call_stack: List[Dict[str, Any]] = []  # Call stack for subroutines
        self.tape_reader_position = 0  # Position for tape reader (>)

        # Input buffers for interactive input
        self.input_buffer: List[str] = []
        self.numeric_input_buffer: List[int] = []

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

    def _get_token_at(self, x: int, y: int) -> Optional[Token]:
        """Get the token at a specific position if program is available."""
        if self.program:
            return self.program.get_operator_at(x, y)

        # If no program available, create token from character for basic operators
        char = self.grid.get(x, y)
        if char in '+~:?>,!ASMD%=<>%GPCRd:/\\0123456789' or char in '@':
            from .tokens import Token, TokenType, Position
            token_type = TokenMapping.get_token_type(char)
            if token_type != TokenType.INVALID:
                return Token(
                    type=token_type,
                    value=char,
                    position=Position(x, y),
                    number_value=int(char) if char.isdigit() else None
                )
        return None

    def _is_valid_position(self, x: int, y: int) -> bool:
        """Check if position is within grid bounds."""
        return 0 <= x < self.grid.width and 0 <= y < self.grid.height

    def _get_new_position(self, droplet: Droplet) -> tuple[int, int]:
        """Calculate new position based on droplet direction."""
        new_x, new_y = droplet.x, droplet.y

        if droplet.direction == Direction.UP:
            new_y -= 1
        elif droplet.direction == Direction.DOWN:
            new_y += 1
        elif droplet.direction == Direction.LEFT:
            new_x -= 1
        elif droplet.direction == Direction.RIGHT:
            new_x += 1

        return new_x, new_y

    def _handle_unary_operators(self, droplet: Droplet, token: Token) -> None:
        """Handle unary operators (+ and ~)."""
        if token.type == TokenType.INCREMENT:
            droplet.value += 1
        elif token.type == TokenType.DECREMENT:
            droplet.value -= 1

    def _handle_data_sources(self, droplet: Droplet, token: Token) -> None:
        """Handle data source operators (0-9, >, ?, ??)."""
        if token.type == TokenType.NUMBER and token.number_value is not None:
            droplet.value = token.number_value
        elif token.type == TokenType.TAPE_READER:
            # For now, use a simple counter as tape data
            droplet.value = self.tape_reader_position
            self.tape_reader_position += 1
        elif token.type == TokenType.CHAR_INPUT:
            if self.input_buffer:
                char = self.input_buffer.pop(0)
                droplet.value = ord(char)
            else:
                # Default to space if no input
                droplet.value = ord(' ')
        elif token.type == TokenType.NUMERIC_INPUT:
            if self.numeric_input_buffer:
                droplet.value = self.numeric_input_buffer.pop(0)
            else:
                # Default to 0 if no input
                droplet.value = 0

    def _handle_stack_operations(self, droplet: Droplet, token: Token) -> None:
        """Handle stack operations (:, ;, d)."""
        if token.type == TokenType.PUSH:
            self.data_stack.append(droplet.value)
        elif token.type == TokenType.POP:
            if self.data_stack:
                droplet.value = self.data_stack.pop()
            else:
                droplet.value = 0  # Default if stack empty
        elif token.type == TokenType.DUPLICATE:
            if self.data_stack:
                self.data_stack.append(self.data_stack[-1])
            else:
                self.data_stack.append(0)  # Default if stack empty

    def _handle_arithmetic_operations(self, droplet: Droplet, token: Token) -> None:
        """Handle arithmetic operations (A, S, M, D, %)."""
        if not self.data_stack:
            return  # Need at least one operand

        if token.type == TokenType.ADD:
            if len(self.data_stack) >= 2:
                b = self.data_stack.pop()
                a = self.data_stack.pop()
                self.data_stack.append(a + b)
        elif token.type == TokenType.SUBTRACT:
            if len(self.data_stack) >= 2:
                b = self.data_stack.pop()
                a = self.data_stack.pop()
                self.data_stack.append(a - b)
        elif token.type == TokenType.MULTIPLY:
            if len(self.data_stack) >= 2:
                b = self.data_stack.pop()
                a = self.data_stack.pop()
                self.data_stack.append(a * b)
        elif token.type == TokenType.DIVIDE:
            if len(self.data_stack) >= 2:
                b = self.data_stack.pop()
                a = self.data_stack.pop()
                if b != 0:
                    self.data_stack.append(a // b)
                else:
                    self.data_stack.append(0)  # Division by zero
        elif token.type == TokenType.MODULO:
            if len(self.data_stack) >= 2:
                b = self.data_stack.pop()
                a = self.data_stack.pop()
                if b != 0:
                    self.data_stack.append(a % b)
                else:
                    self.data_stack.append(0)  # Modulo by zero

    def _handle_io_operations(self, droplet: Droplet, token: Token) -> None:
        """Handle I/O operations (!, ,, n)."""
        if token.type == TokenType.OUTPUT_SINK:
            print(droplet.value, end='')
        elif token.type == TokenType.CHAR_OUTPUT:
            print(chr(droplet.value), end='')
        elif token.type == TokenType.NUMERIC_OUTPUT:
            print(droplet.value)

    def _handle_memory_operations(self, droplet: Droplet, token: Token) -> None:
        """Handle memory operations (G, P)."""
        if token.type == TokenType.GET:
            key = f"{droplet.x},{droplet.y}"
            droplet.value = self.reservoir.get(key, 0)
        elif token.type == TokenType.PUT:
            key = f"{droplet.x},{droplet.y}"
            self.reservoir[key] = droplet.value

    def _handle_subroutine_operations(self, droplet: Droplet, token: Token) -> None:
        """Handle subroutine operations (C, R)."""
        if token.type == TokenType.CALL:
            # Save current state on call stack
            self.call_stack.append({
                'droplet_x': droplet.x,
                'droplet_y': droplet.y,
                'droplet_direction': droplet.direction,
                'droplet_value': droplet.value
            })
            # For now, just continue - in a full implementation,
            # this would jump to subroutine location
        elif token.type == TokenType.RETURN:
            if self.call_stack:
                state = self.call_stack.pop()
                droplet.x = state['droplet_x']
                droplet.y = state['droplet_y']
                droplet.direction = state['droplet_direction']
                droplet.value = state['droplet_value']
            else:
                # No call to return from - destroy droplet
                self.remove_droplet(droplet)

    def _handle_corner_pipes(self, droplet: Droplet, token: Token) -> tuple[int, int]:
        """Handle corner pipe operations (/ and \\) with conditional branching."""
        # Corner pipes change direction but don't change position in this implementation
        # The droplet stays at current position and only direction changes

        if token.type == TokenType.FORWARD_SLASH:
            # / - redirect based on droplet value
            if droplet.value > 0:
                # Turn right
                if droplet.direction == Direction.DOWN:
                    droplet.direction = Direction.LEFT
                elif droplet.direction == Direction.UP:
                    droplet.direction = Direction.RIGHT
                elif droplet.direction == Direction.LEFT:
                    droplet.direction = Direction.DOWN
                elif droplet.direction == Direction.RIGHT:
                    droplet.direction = Direction.UP
            else:
                # Turn left
                if droplet.direction == Direction.DOWN:
                    droplet.direction = Direction.RIGHT
                elif droplet.direction == Direction.UP:
                    droplet.direction = Direction.LEFT
                elif droplet.direction == Direction.LEFT:
                    droplet.direction = Direction.UP
                elif droplet.direction == Direction.RIGHT:
                    droplet.direction = Direction.DOWN

        elif token.type == TokenType.BACK_SLASH:
            # \ - redirect based on droplet value (opposite of /)
            if droplet.value <= 0:
                # Turn right
                if droplet.direction == Direction.DOWN:
                    droplet.direction = Direction.LEFT
                elif droplet.direction == Direction.UP:
                    droplet.direction = Direction.RIGHT
                elif droplet.direction == Direction.LEFT:
                    droplet.direction = Direction.DOWN
                elif droplet.direction == Direction.RIGHT:
                    droplet.direction = Direction.UP
            else:
                # Turn left
                if droplet.direction == Direction.DOWN:
                    droplet.direction = Direction.RIGHT
                elif droplet.direction == Direction.UP:
                    droplet.direction = Direction.LEFT
                elif droplet.direction == Direction.LEFT:
                    droplet.direction = Direction.UP
                elif droplet.direction == Direction.RIGHT:
                    droplet.direction = Direction.DOWN

        # Return current position (no movement for corner pipes)
        return droplet.x, droplet.y

    def tick(self) -> None:
        """
        Execute one tick of the simulation.

        For each active droplet:
        - Calculate the new position based on its direction
        - Handle all operators according to their behavior
        - Check bounds and handle collisions
        """
        droplets_to_remove: List[Droplet] = []
        new_positions: List[tuple] = []  # Track (droplet, new_x, new_y) for collision detection

        for droplet in self.droplets[:]:  # Create a copy to avoid modification during iteration
            # Calculate new position
            new_x, new_y = self._get_new_position(droplet)

            # Check if new position is valid
            if not self._is_valid_position(new_x, new_y):
                # Out of bounds - destroy droplet
                droplets_to_remove.append(droplet)
                continue

            target_cell = self.grid.get(new_x, new_y)
            token = self._get_token_at(new_x, new_y)

            # Handle different cell types
            if target_cell == ' ':
                # Empty space - move droplet
                droplet.x = new_x
                droplet.y = new_y
                new_positions.append((droplet, new_x, new_y))
            elif target_cell == '|':
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
            elif target_cell in '0123456789':
                # Number data source - set value and move
                droplet.value = int(target_cell)
                droplet.x = new_x
                droplet.y = new_y
                new_positions.append((droplet, new_x, new_y))
            elif target_cell in '+~:?>,!ASMD%=<>%GPCRd':
                # Enhanced operators - move first, then process
                droplet.x = new_x
                droplet.y = new_y
                new_positions.append((droplet, new_x, new_y))
            elif target_cell in '/\\':
                # Corner pipes - move first, then process direction change
                droplet.x = new_x
                droplet.y = new_y
                new_positions.append((droplet, new_x, new_y))
            else:
                # Any other non-empty cell - destroy droplet
                droplets_to_remove.append(droplet)

        # Process operators for droplets that moved successfully
        for droplet in self.droplets[:]:  # Use copy again to avoid modification issues
            if droplet not in droplets_to_remove:
                token = self._get_token_at(droplet.x, droplet.y)
                if token:
                    self._process_token(droplet, token)

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

    def _process_token(self, droplet: Droplet, token: Token) -> None:
        """Process a token's effect on a droplet."""
        # Handle different token types
        if token.type in [TokenType.INCREMENT, TokenType.DECREMENT]:
            self._handle_unary_operators(droplet, token)
        elif token.type in [TokenType.NUMBER, TokenType.TAPE_READER,
                           TokenType.CHAR_INPUT, TokenType.NUMERIC_INPUT]:
            self._handle_data_sources(droplet, token)
        elif token.type in [TokenType.PUSH, TokenType.POP, TokenType.DUPLICATE]:
            self._handle_stack_operations(droplet, token)
        elif token.type in [TokenType.ADD, TokenType.SUBTRACT, TokenType.MULTIPLY,
                           TokenType.DIVIDE, TokenType.MODULO]:
            self._handle_arithmetic_operations(droplet, token)
        elif token.type in [TokenType.OUTPUT_SINK, TokenType.CHAR_OUTPUT, TokenType.NUMERIC_OUTPUT]:
            self._handle_io_operations(droplet, token)
        elif token.type in [TokenType.GET, TokenType.PUT]:
            self._handle_memory_operations(droplet, token)
        elif token.type in [TokenType.CALL, TokenType.RETURN]:
            self._handle_subroutine_operations(droplet, token)

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

    def add_input(self, data: str) -> None:
        """Add character input to the input buffer."""
        self.input_buffer.extend(list(data))

    def add_numeric_input(self, value: int) -> None:
        """Add numeric input to the input buffer."""
        self.numeric_input_buffer.append(value)

    def get_stack_depth(self) -> int:
        """Get the current depth of the data stack."""
        return len(self.data_stack)

    def get_stack_top(self) -> Optional[int]:
        """Get the top value of the data stack without popping."""
        return self.data_stack[-1] if self.data_stack else None

    def get_reservoir_size(self) -> int:
        """Get the number of stored values in the reservoir."""
        return len(self.reservoir)

    def get_call_stack_depth(self) -> int:
        """Get the current depth of the call stack."""
        return len(self.call_stack)

    def clear_data_structures(self) -> None:
        """Clear all data structures (stack, reservoir, call stack)."""
        self.data_stack.clear()
        self.reservoir.clear()
        self.call_stack.clear()
        self.tape_reader_position = 0
        self.input_buffer.clear()
        self.numeric_input_buffer.clear()

    def __str__(self) -> str:
        """Return string representation of the engine state."""
        return (f"Engine(grid={self.grid}, active_droplets={len(self.droplets)}, "
                f"stack_depth={len(self.data_stack)}, reservoir_size={len(self.reservoir)}, "
                f"call_depth={len(self.call_stack)})")

    def __repr__(self) -> str:
        """Return detailed string representation of the engine."""
        return (f"Engine(grid={self.grid!r}, droplets={self.droplets!r}, "
                f"data_stack={self.data_stack!r}, reservoir={self.reservoir!r}, "
                f"call_stack_depth={len(self.call_stack)})")