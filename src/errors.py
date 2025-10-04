#!/usr/bin/env python3
"""
Error handling for the Tubular programming language parser.

This module defines error types, error reporting, and position tracking
for the Tubular language parser system.
"""

from typing import List, Optional
from dataclasses import dataclass

try:
    # Try relative imports (when run as module)
    from .tokens import Position, Token
except ImportError:
    # Fall back to absolute imports (when run as script)
    from tokens import Position, Token


class TubularError(Exception):
    """Base class for all Tubular parser errors."""

    def __init__(self, message: str, position: Optional[Position] = None):
        self.message = message
        self.position = position
        super().__init__(self._format_message())

    def _format_message(self) -> str:
        """Format the error message with position information."""
        if self.position:
            return f"Error at {self.position}: {self.message}"
        return f"Error: {self.message}"


class LexicalError(TubularError):
    """Error encountered during lexical analysis (tokenization)."""
    pass


class SyntaxError(TubularError):
    """Error encountered during parsing (structural validation)."""
    pass


class SemanticError(TubularError):
    """Error encountered during semantic analysis."""
    pass


class MultipleEntryPointsError(SemanticError):
    """Error when multiple @ (entry point) symbols are found."""

    def __init__(self, positions: List[Position]):
        self.positions = positions
        message = f"Multiple entry points (@) found at positions: {', '.join(str(pos) for pos in positions)}"
        super().__init__(message)


class UnreachableCodeError(SemanticError):
    """Error when code sections are unreachable from the entry point."""

    def __init__(self, unreachable_positions: List[Position]):
        self.unreachable_positions = unreachable_positions
        positions_str = ', '.join(str(pos) for pos in unreachable_positions[:5])  # Show first 5
        if len(unreachable_positions) > 5:
            positions_str += f" and {len(unreachable_positions) - 5} more"
        message = f"Unreachable code detected at positions: {positions_str}"
        super().__init__(message)


class InvalidOperatorError(LexicalError):
    """Error when an invalid character/operator is encountered."""

    def __init__(self, char: str, position: Position):
        message = f"Invalid character '{char}'"
        super().__init__(message, position)


class ErrorReporter:
    """Collects and reports errors encountered during parsing."""

    def __init__(self):
        self.errors: List[TubularError] = []
        self.warnings: List[TubularError] = []

    def add_error(self, error: TubularError) -> None:
        """Add an error to the error list."""
        self.errors.append(error)

    def add_warning(self, warning: TubularError) -> None:
        """Add a warning to the warning list."""
        self.warnings.append(warning)

    def has_errors(self) -> bool:
        """Check if there are any errors."""
        return len(self.errors) > 0

    def has_warnings(self) -> bool:
        """Check if there are any warnings."""
        return len(self.warnings) > 0

    def error_count(self) -> int:
        """Get the number of errors."""
        return len(self.errors)

    def warning_count(self) -> int:
        """Get the number of warnings."""
        return len(self.warnings)

    def clear(self) -> None:
        """Clear all errors and warnings."""
        self.errors.clear()
        self.warnings.clear()

    def report_lexical_error(self, message: str, position: Position) -> None:
        """Report a lexical error at the given position."""
        error = LexicalError(message, position)
        self.add_error(error)

    def report_syntax_error(self, message: str, position: Optional[Position] = None) -> None:
        """Report a syntax error at the given position."""
        error = SyntaxError(message, position)
        self.add_error(error)

    def report_semantic_error(self, message: str, position: Optional[Position] = None) -> None:
        """Report a semantic error at the given position."""
        error = SemanticError(message, position)
        self.add_error(error)

    def report_multiple_entry_points(self, positions: List[Position]) -> None:
        """Report multiple entry points error."""
        error = MultipleEntryPointsError(positions)
        self.add_error(error)

    def report_unreachable_code(self, unreachable_positions: List[Position]) -> None:
        """Report unreachable code error."""
        error = UnreachableCodeError(unreachable_positions)
        self.add_error(error)

    def format_errors(self) -> str:
        """Format all errors into a readable string."""
        if not self.errors:
            return ""

        lines = ["Parsing Errors:"]
        lines.append("=" * 50)

        for i, error in enumerate(self.errors, 1):
            lines.append(f"{i}. {error}")

        return "\n".join(lines)

    def format_warnings(self) -> str:
        """Format all warnings into a readable string."""
        if not self.warnings:
            return ""

        lines = ["Parsing Warnings:"]
        lines.append("=" * 50)

        for i, warning in enumerate(self.warnings, 1):
            lines.append(f"{i}. {warning}")

        return "\n".join(lines)

    def format_all(self) -> str:
        """Format all errors and warnings into a readable string."""
        result = []

        errors_str = self.format_errors()
        if errors_str:
            result.append(errors_str)

        warnings_str = self.format_warnings()
        if warnings_str:
            if result:
                result.append("")  # Add spacing
            result.append(warnings_str)

        return "\n".join(result)

    def print_errors(self) -> None:
        """Print all errors to stderr."""
        import sys
        error_str = self.format_errors()
        if error_str:
            print(error_str, file=sys.stderr)

    def print_warnings(self) -> None:
        """Print all warnings to stderr."""
        import sys
        warning_str = self.format_warnings()
        if warning_str:
            print(warning_str, file=sys.stderr)

    def print_all(self) -> None:
        """Print all errors and warnings to stderr."""
        import sys
        all_str = self.format_all()
        if all_str:
            print(all_str, file=sys.stderr)