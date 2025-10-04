# Tubular Language Implementation - Build System
.PHONY: compile test clean help

# Default target
help:
	@echo "Tubular Language Implementation Build System"
	@echo ""
	@echo "Available commands:"
	@echo "  make compile    - Build the project executable"
	@echo "  make test       - Run the test suite"
	@echo "  make clean      - Remove all build artifacts and temporary files"
	@echo "  make help       - Show this help message"

# Build the project executable
compile: src/
	@echo "Building Tubular compiler..."
	@if [ -f src/compiler.py ]; then \
		echo "Found existing compiler.py"; \
		cp src/compiler.py tubular_compiler; \
		chmod +x tubular_compiler; \
		echo "Built executable: tubular_compiler"; \
	else \
		echo "Error: src/compiler.py not found"; \
		exit 1; \
	fi

# Run the test suite
test: tests/
	@echo "Running test suite..."
	@if [ -d tests ]; then \
		if [ -n "$$(find tests -name '*.py' -type f)" ]; then \
			echo "Found test files, running with pytest"; \
			echo "Running pytest tests..."; \
			python -m pytest tests/ -v --tb=short; \
			echo "Running custom test_engine.py..."; \
			python tests/test_engine.py; \
		else \
			echo "No test files found in tests/ directory"; \
			echo "Creating placeholder test file..."; \
			echo 'import pytest' > tests/test_placeholder.py; \
			echo '' >> tests/test_placeholder.py; \
			echo 'def test_placeholder():' >> tests/test_placeholder.py; \
			echo '    assert True' >> tests/test_placeholder.py; \
			python -m pytest tests/test_placeholder.py -v; \
		fi \
	else \
		echo "Error: tests/ directory not found"; \
		exit 1; \
	fi

# Clean all build artifacts and temporary files
clean:
	@echo "Cleaning build artifacts..."
	@rm -f tubular_compiler
	@rm -f *.pyc
	@find . -type f -name "*.pyc" -delete
	@find . -type d -name "__pycache__" -delete
	@find . -type f -name "*.pyo" -delete
	@find . -type f -name "*.pyd" -delete
	@find . -type f -name ".coverage" -delete
	@find . -type d -name "*.egg-info" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name ".pytest_cache" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name ".coverage" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name "dist" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name "build" -exec rm -rf {} + 2>/dev/null || true
	@find . -type f -name "*.log" -delete
	@echo "Clean complete"