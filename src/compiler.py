"""Tubular to WASM Compiler

This module implements a compiler that translates Tubular source code
into WebAssembly (WASM) text format (WAT).
"""

from typing import List, Tuple, Dict, Optional
from .grid import Grid
from .droplet import Direction


class Token:
    """
    Represents a token in the Tubular source code.
    """
    
    def __init__(self, char: str, x: int, y: int):
        self.char = char
        self.x = x
        self.y = y
        
    def __repr__(self):
        return f"Token('{self.char}' at ({self.x}, {self.y}))"


class Lexer:
    """
    Lexical analyzer for Tubular source code.
    Reads the .tub grid and validates characters.
    """
    
    def __init__(self, grid: Grid):
        self.grid = grid
        self.tokens: List[Token] = []
        
    def tokenize(self) -> List[Token]:
        """
        Convert the grid into a list of tokens.
        This is a simple sequential scan of the grid, but for an actual compiler
        we might need to traverse in execution order.
        
        Returns:
            List of tokens representing the grid
        """
        self.tokens = []
        
        # Scan the grid line by line
        for y in range(self.grid.height):
            for x in range(self.grid.width):
                char = self.grid.get_cell(x, y)
                
                # Only add non-empty cells to the token list
                if char != ' ':
                    token = Token(char, x, y)
                    self.tokens.append(token)
        
        return self.tokens
    
    def validate_characters(self) -> Tuple[bool, List[str]]:
        """
        Validate that all characters in the grid are valid Tubular operators.
        
        Returns:
            Tuple of (is_valid, list_of_invalid_characters_with_positions)
        """
        invalid_chars = []
        
        for y in range(self.grid.height):
            for x in range(self.grid.width):
                char = self.grid.get_cell(x, y)
                
                if char != ' ' and not self._is_valid_char(char):
                    invalid_chars.append(f"Invalid character '{char}' at ({x}, {y})")
        
        return len(invalid_chars) == 0, invalid_chars
    
    def _is_valid_char(self, char: str) -> bool:
        """
        Check if a character is a valid Tubular operator.
        """
        valid_chars = {
            '@',  # Program start
            '|', '-', '^', '#',  # Flow control
            '/', '\\',  # Corners
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',  # Number sources
            '>', '?', '!',  # I/O
            '+', '~',  # Unary operators
            ':', ';', 'd', 'A', 'S', 'M', 'D',  # Stack operators
            'G', 'P',  # Reservoir operators
            'C', 'R'  # Subroutine operators
        }
        return char in valid_chars


class CodeGenerator:
    """
    Code generator that takes a validated Tubular grid and emits WASM text format (WAT).
    """
    
    def __init__(self, grid: Grid):
        self.grid = grid
        
    def generate_wat(self) -> str:
        """
        Generate WebAssembly text format (WAT) from the Tubular grid.
        
        Returns:
            String containing WASM text format code
        """
        # Analyze the grid to understand the structure
        self.analyze_grid_topology()
        
        # Start with a basic WASM module structure
        wat_code = []
        wat_code.append("(module")
        
        # Import necessary functions for I/O
        wat_code.append('  (import "env" "memory" (memory 1))')
        wat_code.append('  (import "env" "log" (func $log (param i32)))')
        wat_code.append('  (import "env" "log_char" (func $log_char (param i32)))')
        
        # Define memory for the droplet simulation
        # We'll use a fixed-size array to store droplets (x, y, direction, value)
        # Each droplet needs 4 i32 values, so 100 droplets would need 400 values = 1600 bytes
        wat_code.append('  (global $droplet_count (mut i32) (i32.const 0))')
        wat_code.append('  (global $max_droplets (i32) (i32.const 100))')
        
        # Define linear memory for data structures
        wat_code.append('  (global $stack_ptr (mut i32) (i32.const 1024))')
        wat_code.append('  (global $stack_base (i32) (i32.const 1024))')
        wat_code.append('  (global $call_stack_ptr (mut i32) (i32.const 2048))')
        wat_code.append('  (global $call_stack_base (i32) (i32.const 2048))')
        
        # Memory layout for droplets: each droplet takes 4 slots [x, y, direction_idx, value]
        # Directions: 0=UP, 1=DOWN, 2=LEFT, 3=RIGHT
        # Droplets array starts at 4096
        wat_code.append('  (global $droplets_base (i32) (i32.const 4096))')
        
        # Reservoir memory starts at 8192
        wat_code.append('  (global $reservoir_base (i32) (i32.const 8192))')
        
        # Helper functions
        wat_code.extend(self._generate_helper_functions())
        
        # Generate functions for each Tubular operator
        wat_code.extend(self._generate_operator_functions())
        
        # Main execution function that simulates the droplet movement
        wat_code.append('  (func $execute (export "execute")')
        wat_code.extend(self._generate_execution_logic())
        wat_code.append('  )')  # End execute function
        
        wat_code.append(")")
        
        return "\n".join(wat_code)
    
    def analyze_grid_topology(self):
        """
        Analyze the grid to understand droplet flow paths and connections.
        This is a critical component for translating Tubular to WASM.
        """
        # We need to create a detailed representation of how droplets flow through the grid
        # This includes tracking possible states and transitions
        
        # The execution model for Tubular is complex:
        # 1. Multiple droplets can exist simultaneously
        # 2. Droplets move simultaneously each "tick"
        # 3. Collisions destroy droplets
        # 4. Some operators create or destroy droplets
        
        # For compilation, we'll model the grid as a series of state transitions
        # in WASM code that simulates this behavior
        
        # Track all cells that contain operators
        self.operators = {}
        for y in range(self.grid.height):
            for x in range(self.grid.width):
                char = self.grid.get_cell(x, y)
                if char != ' ' and char != '@':  # Non-empty cells except start
                    self.operators[(x, y)] = char

    def _generate_helper_functions(self):
        """
        Generate helper functions for common operations.
        """
        wat_code = []
        
        # Function to get grid character at position (x, y)
        wat_code.append('  (func $get_grid_char (param $x i32) (param $y i32) (result i32)')
        wat_code.append('    ;; This would need to access a memory representation of the grid')
        wat_code.append('    ;; For now, using a data segment to store the grid')
        wat_code.append('    local.get $y')
        wat_code.append('    i32.const {}'.format(self.grid.width))
        wat_code.append('    i32.mul')
        wat_code.append('    local.get $x')
        wat_code.append('    i32.add')
        wat_code.append('    ;; This would access the grid in memory')
        wat_code.append('    ;; Placeholder - return 0 for now')
        wat_code.append('    i32.const 0')
        wat_code.append('  )')
        
        # Stack operations
        wat_code.append('  (func $stack_push (param $value i32)')
        wat_code.append('    global.get $stack_ptr')
        wat_code.append('    local.get $value')
        wat_code.append('    i32.store')
        wat_code.append('    global.get $stack_ptr')
        wat_code.append('    i32.const 4')
        wat_code.append('    i32.add')
        wat_code.append('    global.set $stack_ptr')
        wat_code.append('  )')
        
        wat_code.append('  (func $stack_pop (result i32)')
        wat_code.append('    global.get $stack_ptr')
        wat_code.append('    i32.const 4')
        wat_code.append('    i32.sub')
        wat_code.append('    global.tee $stack_ptr')
        wat_code.append('    i32.load')
        wat_code.append('  )')
        
        # Reservoir operations
        wat_code.append('  (func $reservoir_get (param $x i32) (param $y i32) (result i32)')
        wat_code.append('    ;; Map 2D (x,y) to 1D index: y*width + x, then multiply by 4 for i32 size')
        wat_code.append('    local.get $y')
        wat_code.append('    i32.const {}'.format(self.grid.width))
        wat_code.append('    i32.mul')
        wat_code.append('    local.get $x')
        wat_code.append('    i32.add')
        wat_code.append('    i32.const 4')
        wat_code.append('    i32.mul')
        wat_code.append('    global.get $reservoir_base')
        wat_code.append('    i32.add')
        wat_code.append('    i32.load')
        wat_code.append('  )')
        
        wat_code.append('  (func $reservoir_set (param $x i32) (param $y i32) (param $value i32)')
        wat_code.append('    ;; Map 2D (x,y) to 1D index: y*width + x, then multiply by 4 for i32 size')
        wat_code.append('    local.get $y')
        wat_code.append('    i32.const {}'.format(self.grid.width))
        wat_code.append('    i32.mul')
        wat_code.append('    local.get $x')
        wat_code.append('    i32.add')
        wat_code.append('    i32.const 4')
        wat_code.append('    i32.mul')
        wat_code.append('    global.get $reservoir_base')
        wat_code.append('    i32.add')
        wat_code.append('    local.get $value')
        wat_code.append('    i32.store')
        wat_code.append('  )')
        
        # Droplet operations
        wat_code.append('  (func $create_droplet (param $x i32) (param $y i32) (param $dir_idx i32) (param $value i32)')
        wat_code.append('    ;; Check if we can create a new droplet')
        wat_code.append('    global.get $droplet_count')
        wat_code.append('    global.get $max_droplets')
        wat_code.append('    i32.ge_s')
        wat_code.append('    if')  # If at max capacity, return
        wat_code.append('      return')
        wat_code.append('    end')
        wat_code.append('    ;; Calculate position in droplet array')
        wat_code.append('    global.get $droplet_count')
        wat_code.append('    i32.const 4')  # 4 values per droplet
        wat_code.append('    i32.mul')
        wat_code.append('    global.get $droplets_base')
        wat_code.append('    i32.add')
        wat_code.append('    ;; Store x coordinate')
        wat_code.append('    local.get $x')
        wat_code.append('    i32.store')
        wat_code.append('    ;; Store y coordinate')
        wat_code.append('    global.get $droplet_count')
        wat_code.append('    i32.const 4')
        wat_code.append('    i32.mul')
        wat_code.append('    global.get $droplets_base')
        wat_code.append('    i32.add')
        wat_code.append('    i32.const 4')  # Offset for y (second value)')
        wat_code.append('    i32.add')
        wat_code.append('    local.get $y')
        wat_code.append('    i32.store')
        wat_code.append('    ;; Store direction')
        wat_code.append('    global.get $droplet_count')
        wat_code.append('    i32.const 4')
        wat_code.append('    i32.mul')
        wat_code.append('    global.get $droplets_base')
        wat_code.append('    i32.add')
        wat_code.append('    i32.const 8')  # Offset for direction (third value)')
        wat_code.append('    i32.add')
        wat_code.append('    local.get $dir_idx')
        wat_code.append('    i32.store')
        wat_code.append('    ;; Store value')
        wat_code.append('    global.get $droplet_count')
        wat_code.append('    i32.const 4')
        wat_code.append('    i32.mul')
        wat_code.append('    global.get $droplets_base')
        wat_code.append('    i32.add')
        wat_code.append('    i32.const 12')  # Offset for value (fourth value)')
        wat_code.append('    i32.add')
        wat_code.append('    local.get $value')
        wat_code.append('    i32.store')
        wat_code.append('    ;; Increment droplet count')
        wat_code.append('    global.get $droplet_count')
        wat_code.append('    i32.const 1')
        wat_code.append('    i32.add')
        wat_code.append('    global.set $droplet_count')
        wat_code.append('  )')
        
        return wat_code
    
    def _generate_operator_functions(self):
        """
        Generate WASM functions for each Tubular operator.
        """
        wat_code = []
        
        # For each unique operator in the grid, create a corresponding WASM function
        unique_ops = set(self.operators.values())
        
        for op in unique_ops:
            # Create a function that implements the logic for this operator
            if op == '+':  # Increment
                wat_code.append('  (func $op_increment (param $current_value i32) (result i32)')
                wat_code.append('    local.get $current_value')
                wat_code.append('    i32.const 1')
                wat_code.append('    i32.add')
                wat_code.append('  )')
            elif op == '~':  # Decrement
                wat_code.append('  (func $op_decrement (param $current_value i32) (result i32)')
                wat_code.append('    local.get $current_value')
                wat_code.append('    i32.const 1')
                wat_code.append('    i32.sub')
                wat_code.append('  )')
            elif op == ':':  # Push
                wat_code.append('  (func $op_push (param $current_value i32)')
                wat_code.append('    call $stack_push')
                wat_code.append('    ;; Value is still available as a return value if needed')
                wat_code.append('    local.get $current_value')
                wat_code.append('    drop')
                wat_code.append('  )')
            elif op == ';':  # Pop
                wat_code.append('  (func $op_pop (result i32)')
                wat_code.append('    call $stack_pop')
                wat_code.append('  )')
            elif op.isdigit():  # Number source
                value = int(op)
                wat_code.append('  (func $op_number_{} (result i32)'.format(op))
                wat_code.append('    i32.const {}'.format(value))
                wat_code.append('  )')
        
        return wat_code
    
    def _generate_execution_logic(self):
        """
        Generate the main execution loop that simulates droplet movement.
        """
        execution_code = []
        
        # Initialize with the starting droplet
        start_x, start_y = self.grid.start_pos
        execution_code.append('    ;; Create initial droplet at start position')
        execution_code.append('    i32.const {}'.format(start_x))
        execution_code.append('    i32.const {}'.format(start_y))
        execution_code.append('    i32.const 1')  # Direction DOWN = 1
        execution_code.append('    i32.const 0')  # Initial value = 0
        execution_code.append('    call $create_droplet')
        
        # Main simulation loop (with a maximum tick count to prevent infinite loops)
        execution_code.append('    ;; Main simulation loop (max 10000 ticks)')
        execution_code.append('    (local $tick_count i32)')
        execution_code.append('    (local $i i32)')  # Loop counter for iterating droplets
        execution_code.append('    (local $current_x i32)')
        execution_code.append('    (local $current_y i32)')
        execution_code.append('    (local $current_dir i32)')
        execution_code.append('    (local $current_value i32)')
        execution_code.append('    (local $new_x i32)')
        execution_code.append('    (local $new_y i32)')
        execution_code.append('    (local $char_code i32)')
        execution_code.append('    (local $result_value i32)')
        execution_code.append('    ')
        execution_code.append('    (loop $main_loop')
        execution_code.append('      ;; Check termination conditions')
        execution_code.append('      local.get $tick_count')
        execution_code.append('      i32.const 10000')
        execution_code.append('      i32.ge_s')
        execution_code.append('      if')
        execution_code.append('        ;; Max ticks reached')
        execution_code.append('        br $main_loop_end')
        execution_code.append('      end')
        execution_code.append('      global.get $droplet_count')
        execution_code.append('      i32.eqz')
        execution_code.append('      if')
        execution_code.append('        ;; No more droplets')
        execution_code.append('        br $main_loop_end')
        execution_code.append('      end')
        execution_code.append('      ')
        execution_code.append('      ;; Process all current droplets for this tick')
        execution_code.append('      local.set $i (i32.const 0)')
        execution_code.append('      (loop $process_droplets')
        execution_code.append('        ;; Check if we\'ve processed all droplets')
        execution_code.append('        local.get $i')
        execution_code.append('        global.get $droplet_count')
        execution_code.append('        i32.ge_s')
        execution_code.append('        br_if $process_droplets_end')
        execution_code.append('        ')
        execution_code.append('        ;; Get the current droplet data')
        execution_code.append('        ;; Calculate memory address for droplet i')
        execution_code.append('        local.get $i')
        execution_code.append('        i32.const 4')
        execution_code.append('        i32.mul')
        execution_code.append('        global.get $droplets_base')
        execution_code.append('        i32.add')
        execution_code.append('        ;; Load x')
        execution_code.append('        i32.load')
        execution_code.append('        local.set $current_x')
        execution_code.append('        ;; Load y')
        execution_code.append('        local.get $i')
        execution_code.append('        i32.const 4')
        execution_code.append('        i32.mul')
        execution_code.append('        global.get $droplets_base')
        execution_code.append('        i32.add')
        execution_code.append('        i32.const 4')
        execution_code.append('        i32.add')
        execution_code.append('        i32.load')
        execution_code.append('        local.set $current_y')
        execution_code.append('        ;; Load direction')
        execution_code.append('        local.get $i')
        execution_code.append('        i32.const 4')
        execution_code.append('        i32.mul')
        execution_code.append('        global.get $droplets_base')
        execution_code.append('        i32.add')
        execution_code.append('        i32.const 8')
        execution_code.append('        i32.add')
        execution_code.append('        i32.load')
        execution_code.append('        local.set $current_dir')
        execution_code.append('        ;; Load value')
        execution_code.append('        local.get $i')
        execution_code.append('        i32.const 4')
        execution_code.append('        i32.mul')
        execution_code.append('        global.get $droplets_base')
        execution_code.append('        i32.add')
        execution_code.append('        i32.const 12')
        execution_code.append('        i32.add')
        execution_code.append('        i32.load')
        execution_code.append('        local.set $current_value')
        execution_code.append('        ')
        execution_code.append('        ;; Get character at droplet position')
        execution_code.append('        local.get $current_x')
        execution_code.append('        local.get $current_y')
        execution_code.append('        call $get_grid_char')
        execution_code.append('        local.set $char_code')
        execution_code.append('        ')
        execution_code.append('        ;; Process the character')
        execution_code.append('        ;; In a real implementation, we would have a large if/else chain')
        execution_code.append('        ;; or use a br_table to jump to the appropriate handler')
        execution_code.append('        ;; For simplicity in this example, we\'ll handle a few key cases')
        execution_code.append('        local.get $char_code')
        execution_code.append('        i32.const 48')  # ASCII for \'0\'')
        execution_code.append('        i32.sub')
        execution_code.append('        local.tee $result_value')
        execution_code.append('        i32.const 0')
        execution_code.append('        i32.ge_s')
        execution_code.append('        local.get $result_value')
        execution_code.append('        i32.const 9')
        execution_code.append('        i32.le_s')
        execution_code.append('        i32.and')
        execution_code.append('        if')  # If it\'s a digit
        execution_code.append('          ;; Create new droplet with digit value, direction DOWN')
        execution_code.append('          local.get $current_x')
        execution_code.append('          local.get $current_y')
        execution_code.append('          i32.const 1')  # Direction DOWN')
        execution_code.append('          local.get $result_value')
        execution_code.append('          call $create_droplet')
        execution_code.append('          ;; Remove current droplet by marking it for deletion')
        execution_code.append('          ;; (In a real implementation, we would handle this properly)')
        execution_code.append('        else')
        execution_code.append('          ;; Handle other characters - this is a simplified version')
        execution_code.append('          ;; In reality, each character would have its own handling logic')
        execution_code.append('          block $skip_char_handling')
        execution_code.append('            ;; If char is \'!\' (output), handle it')
        execution_code.append('            local.get $char_code')
        execution_code.append('            i32.const 33')  # ASCII for \'!\'')
        execution_code.append('            i32.eq')
        execution_code.append('            if')
        execution_code.append('              local.get $current_value')
        execution_code.append('              call $log')
        execution_code.append('              ;; Remove this droplet (output sink)')
        execution_code.append('              br $skip_char_handling')
        execution_code.append('            end')
        execution_code.append('            ;; If char is \'+\' (increment), handle it')
        execution_code.append('            local.get $char_code')
        execution_code.append('            i32.const 43')  # ASCII for \'+\'')
        execution_code.append('            i32.eq')
        execution_code.append('            if')
        execution_code.append('              local.get $current_value')
        execution_code.append('              call $op_increment')
        execution_code.append('              local.set $current_value')
        execution_code.append('              ;; Update the droplet\'s value in memory')
        execution_code.append('              local.get $i')
        execution_code.append('              i32.const 4')
        execution_code.append('              i32.mul')
        execution_code.append('              global.get $droplets_base')
        execution_code.append('              i32.add')
        execution_code.append('              i32.const 12')  # offset for value')
        execution_code.append('              i32.add')
        execution_code.append('              local.get $current_value')
        execution_code.append('              i32.store')
        execution_code.append('              br $skip_char_handling')
        execution_code.append('            end')
        execution_code.append('            ;; Add more character handling as needed')
        execution_code.append('            ;; For now, just continue moving in the current direction')
        execution_code.append('            local.get $current_x')
        execution_code.append('            local.set $new_x')
        execution_code.append('            local.get $current_y')
        execution_code.append('            local.set $new_y')
        execution_code.append('            local.get $current_dir')
        execution_code.append('            i32.const 0')  # UP = 0')
        execution_code.append('            i32.eq')
        execution_code.append('            if')
        execution_code.append('              local.get $current_y')
        execution_code.append('              i32.const 1')
        execution_code.append('              i32.sub')
        execution_code.append('              local.set $new_y')
        execution_code.append('            end')
        execution_code.append('            local.get $current_dir')
        execution_code.append('            i32.const 1')  # DOWN = 1')
        execution_code.append('            i32.eq')
        execution_code.append('            if')
        execution_code.append('              local.get $current_y')
        execution_code.append('              i32.const 1')
        execution_code.append('              i32.add')
        execution_code.append('              local.set $new_y')
        execution_code.append('            end')
        execution_code.append('            local.get $current_dir')
        execution_code.append('            i32.const 2')  # LEFT = 2')
        execution_code.append('            i32.eq')
        execution_code.append('            if')
        execution_code.append('              local.get $current_x')
        execution_code.append('              i32.const 1')
        execution_code.append('              i32.sub')
        execution_code.append('              local.set $new_x')
        execution_code.append('            end')
        execution_code.append('            local.get $current_dir')
        execution_code.append('            i32.const 3')  # RIGHT = 3')
        execution_code.append('            i32.eq')
        execution_code.append('            if')
        execution_code.append('              local.get $current_x')
        execution_code.append('              i32.const 1')
        execution_code.append('              i32.add')
        execution_code.append('              local.set $new_x')
        execution_code.append('            end')
        execution_code.append('            ;; Update the droplet\'s position in memory')
        execution_code.append('            local.get $i')
        execution_code.append('            i32.const 4')
        execution_code.append('            i32.mul')
        execution_code.append('            global.get $droplets_base')
        execution_code.append('            i32.add')
        execution_code.append('            local.get $new_x')
        execution_code.append('            i32.store')  # Store new x')
        execution_code.append('            local.get $i')
        execution_code.append('            i32.const 4')
        execution_code.append('            i32.mul')
        execution_code.append('            global.get $droplets_base')
        execution_code.append('            i32.add')
        execution_code.append('            i32.const 4')
        execution_code.append('            i32.add')
        execution_code.append('            local.get $new_y')
        execution_code.append('            i32.store')  # Store new y')
        execution_code.append('          end')  # End of skip_char_handling block')
        execution_code.append('        end')  # End of digit handling if-else')
        execution_code.append('        ')
        execution_code.append('        local.get $i')
        execution_code.append('        i32.const 1')
        execution_code.append('        i32.add')
        execution_code.append('        local.set $i')
        execution_code.append('        br $process_droplets')
        execution_code.append('      end')  # End of process_droplets loop')
        execution_code.append('      ')
        execution_code.append('      ;; Increment tick counter')
        execution_code.append('      local.get $tick_count')
        execution_code.append('      i32.const 1')
        execution_code.append('      i32.add')
        execution_code.append('      local.set $tick_count')
        execution_code.append('      br $main_loop')
        execution_code.append('    end')  # End of main_loop')
        execution_code.append('    ')
        execution_code.append('    ;; Label to break to when ending the loop')
        execution_code.append('    block $main_loop_end')
        execution_code.append('    end')
        
        return execution_code


class TubularCompiler:
    """
    Main compiler class that orchestrates the compilation process
    from Tubular source code to WebAssembly.
    """
    
    def __init__(self):
        self.lexer: Optional[Lexer] = None
        self.code_generator: Optional[CodeGenerator] = None
        
    def compile(self, grid: Grid) -> str:
        """
        Compile a Tubular grid to WebAssembly text format.
        
        Args:
            grid: The Tubular program grid to compile
            
        Returns:
            String containing the generated WASM text format
        """
        # Create the lexer and validate the input
        self.lexer = Lexer(grid)
        is_valid, invalid_chars = self.lexer.validate_characters()
        
        if not is_valid:
            raise ValueError(f"Invalid characters in Tubular grid: {', '.join(invalid_chars)}")
        
        # Tokenize the grid
        tokens = self.lexer.tokenize()
        
        # Create the code generator and generate WASM
        self.code_generator = CodeGenerator(grid)
        wat_code = self.code_generator.generate_wat()
        
        return wat_code