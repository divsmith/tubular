#!/usr/bin/env python3
"""
Token definitions for the Tubular programming language parser.

This module defines the token types, token classes, and position tracking
for the Tubular language parser system.
"""

from enum import Enum, auto
from typing import NamedTuple, Optional
from dataclasses import dataclass


class TokenType(Enum):
    """Enumeration of all possible token types in Tubular language."""

    # Flow Control Pipes
    VERTICAL_PIPE = auto()      # |
    HORIZONTAL_PIPE = auto()    # -
    GO_UP_PIPE = auto()         # ^
    WALL = auto()               # #

    # Corner Pipes (Branching and Looping)
    FORWARD_SLASH = auto()      # /
    BACK_SLASH = auto()         # \

    # Data Sources
    PROGRAM_START = auto()      # @
    NUMBER = auto()             # 0-9
    TAPE_READER = auto()        # >
    CHAR_INPUT = auto()         # ?
    NUMERIC_INPUT = auto()      # ??

    # Data Sinks
    OUTPUT_SINK = auto()        # !
    CHAR_OUTPUT = auto()        # ,
    NUMERIC_OUTPUT = auto()     # n

    # Unary Operators
    INCREMENT = auto()          # +
    DECREMENT = auto()          # ~

    # Data Stack Operators
    PUSH = auto()               # :
    POP = auto()                # ;
    DUPLICATE = auto()          # d
    ADD = auto()                # A
    SUBTRACT = auto()           # S
    MULTIPLY = auto()           # M
    DIVIDE = auto()             # D
    EQUAL = auto()              # =
    LESS_THAN = auto()          # <
    GREATER_THAN = auto()       # >
    MODULO = auto()             # %

    # Reservoir (Memory) Operators
    GET = auto()                # G
    PUT = auto()                # P

    # Subroutine Operators
    CALL = auto()               # C
    RETURN = auto()             # R

    # Special tokens
    EOF = auto()                # End of file/input
    INVALID = auto()            # Invalid/unknown character


class Position(NamedTuple):
    """Represents a position in the source grid."""
    x: int  # Column
    y: int  # Row

    def __str__(self) -> str:
        return f"({self.x}, {self.y})"


@dataclass
class Token:
    """Represents a token with its type, value, and position."""

    type: TokenType
    value: Optional[str] = None
    position: Optional[Position] = None
    number_value: Optional[int] = None  # For NUMBER tokens

    def __str__(self) -> str:
        if self.type == TokenType.NUMBER and self.number_value is not None:
            return f"Token({self.type.name}, value='{self.number_value}' at {self.position})"
        elif self.value:
            return f"Token({self.type.name}, value='{self.value}' at {self.position})"
        else:
            return f"Token({self.type.name} at {self.position})"

    def __repr__(self) -> str:
        return self.__str__()


class TokenMapping:
    """Maps characters to their corresponding token types."""

    # Single character mappings
    CHAR_TO_TOKEN = {
        # Flow Control Pipes
        '|': TokenType.VERTICAL_PIPE,
        '-': TokenType.HORIZONTAL_PIPE,
        '^': TokenType.GO_UP_PIPE,
        '#': TokenType.WALL,

        # Corner Pipes
        '/': TokenType.FORWARD_SLASH,
        '\\': TokenType.BACK_SLASH,

        # Data Sources
        '@': TokenType.PROGRAM_START,
        '>': TokenType.TAPE_READER,
        '?': TokenType.CHAR_INPUT,
        '??': TokenType.NUMERIC_INPUT,  # Special case - two characters

        # Data Sinks
        '!': TokenType.OUTPUT_SINK,
        ',': TokenType.CHAR_OUTPUT,
        'n': TokenType.NUMERIC_OUTPUT,

        # Unary Operators
        '+': TokenType.INCREMENT,
        '~': TokenType.DECREMENT,

        # Data Stack Operators
        ':': TokenType.PUSH,
        ';': TokenType.POP,
        'd': TokenType.DUPLICATE,
        'A': TokenType.ADD,
        'S': TokenType.SUBTRACT,
        'M': TokenType.MULTIPLY,
        'D': TokenType.DIVIDE,
        '=': TokenType.EQUAL,
        '<': TokenType.LESS_THAN,
        '>': TokenType.GREATER_THAN,  # Note: conflicts with TAPE_READER, context matters
        '%': TokenType.MODULO,

        # Reservoir Operators
        'G': TokenType.GET,
        'P': TokenType.PUT,

        # Subroutine Operators
        'C': TokenType.CALL,
        'R': TokenType.RETURN,
    }

    @classmethod
    def get_token_type(cls, char: str) -> TokenType:
        """
        Get the token type for a single character.

        Args:
            char: The character to look up

        Returns:
            The corresponding TokenType, or INVALID if not found
        """
        return cls.CHAR_TO_TOKEN.get(char, TokenType.INVALID)

    @classmethod
    def is_valid_operator(cls, char: str) -> bool:
        """
        Check if a character is a valid Tubular operator.

        Args:
            char: The character to check

        Returns:
            True if the character is a valid operator, False otherwise
        """
        return char in cls.CHAR_TO_TOKEN

    @classmethod
    def is_number(cls, char: str) -> bool:
        """
        Check if a character represents a number (0-9).

        Args:
            char: The character to check

        Returns:
            True if the character is a digit, False otherwise
        """
        return char.isdigit()

    @classmethod
    def get_all_operators(cls) -> set:
        """
        Get all valid operator characters.

        Returns:
            A set of all valid operator characters
        """
        return set(cls.CHAR_TO_TOKEN.keys())