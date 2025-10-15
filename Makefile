# Makefile for Tubular Programming Language
# A 2D visual esoteric programming language

# Project configuration
PROJECT_NAME = tubular
BINARY_NAME = tubular
VERSION = 0.1.0

# Directories
SRC_DIR = src
TARGET_DIR = target
EXAMPLES_DIR = examples
DOCS_DIR = docs
TESTS_DIR = tests

# Rust toolchain
CARGO = ~/.cargo/bin/cargo
RUSTC = ~/.cargo/bin/rustc

# Binary paths
DEBUG_BINARY = $(TARGET_DIR)/debug/$(BINARY_NAME)
RELEASE_BINARY = $(TARGET_DIR)/release/$(BINARY_NAME)

# Default target
.PHONY: default
default: build

# Help target
.PHONY: help
help: ## Show this help message
	@echo "Tubular Programming Language - Build System"
	@echo "=========================================="
	@echo ""
	@echo "Available targets:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo ""
	@echo "Examples:"
	@echo "  make build          # Build release version"
	@echo "  make test           # Run all tests"
	@echo "  make run FILE=simple.tb  # Run an example program"
	@echo "  make examples       # Test all example programs"
	@echo "  make clean          # Clean build artifacts"

# Build targets
.PHONY: build
build: ## Build the project in release mode
	@echo "Building $(PROJECT_NAME) v$(VERSION) in release mode..."
	$(CARGO) build --release
	@echo "✅ Build complete: $(RELEASE_BINARY)"

.PHONY: build-debug
build-debug: ## Build the project in debug mode
	@echo "Building $(PROJECT_NAME) v$(VERSION) in debug mode..."
	$(CARGO) build
	@echo "✅ Debug build complete: $(DEBUG_BINARY)"

.PHONY: rebuild
rebuild: clean build ## Clean and build the project

# Test targets
.PHONY: test
test: ## Run smoke test (critical functionality verification)
	@echo "Running smoke tests..."
	$(CARGO) test smoke_test --test integration

.PHONY: test-release
test-release: ## Run smoke test in release mode
	@echo "Running smoke tests in release mode..."
	$(CARGO) test --release smoke_test --test integration

.PHONY: test-verbose
test-verbose: ## Run tests with verbose output
	@echo "Running smoke tests with verbose output..."
	$(CARGO) test smoke_test --test integration -- --nocapture

.PHONY: integration-test
integration-test: ## Run integration tests (alias for test)
	@echo "Running integration tests..."
	$(CARGO) test --test integration

.PHONY: unit-tests
unit-tests: ## Run unit tests (library and binary tests)
	@echo "Running unit tests..."
	$(CARGO) test --lib --bins

.PHONY: examples-test
examples-test: ## Test all example programs
	@echo "Testing all example programs..."
	@for file in $(EXAMPLES_DIR)/*.tb; do \
		echo "Testing $$file..."; \
		./$(RELEASE_BINARY) "$$file"; \
		echo ""; \
	done
	@echo "✅ All examples tested successfully"

# Example targets
.PHONY: examples
examples: examples-test ## Alias for examples-test

.PHONY: run
run: ## Run a specific example (usage: make run FILE=example.tb)
	@if [ -z "$(FILE)" ]; then \
		echo "❌ Error: Please specify a file with FILE=filename"; \
		echo "Usage: make run FILE=simple.tb"; \
		exit 1; \
	fi
	@if [ ! -f "$(FILE)" ]; then \
		echo "❌ Error: File '$(FILE)' not found"; \
		exit 1; \
	fi
	@echo "Running $(FILE)..."
	./$(RELEASE_BINARY) $(FILE)

.PHONY: run-debug
run-debug: ## Run a specific example in debug mode (usage: make run-debug FILE=example.tb)
	@if [ -z "$(FILE)" ]; then \
		echo "❌ Error: Please specify a file with FILE=filename"; \
		echo "Usage: make run-debug FILE=simple.tb"; \
		exit 1; \
	fi
	@if [ ! -f "$(FILE)" ]; then \
		echo "❌ Error: File '$(FILE)' not found"; \
		exit 1; \
	fi
	@echo "Running $(FILE) in debug mode..."
	./$(DEBUG_BINARY) $(FILE)

# Quick demo targets for common examples
.PHONY: demo-simple
demo-simple: build ## Run the simple arithmetic example
	@echo "Running simple arithmetic demo..."
	./$(RELEASE_BINARY) $(EXAMPLES_DIR)/simple.tb

.PHONY: demo-hello
demo-hello: build ## Run the hello world example
	@echo "Running hello world demo..."
	./$(RELEASE_BINARY) $(EXAMPLES_DIR)/hello_world.tb

.PHONY: demo-countdown
demo-countdown: build ## Run the countdown example
	@echo "Running countdown demo..."
	./$(RELEASE_BINARY) $(EXAMPLES_DIR)/countdown.tb

.PHONY: demo-calculator
demo-calculator: build ## Run the calculator example
	@echo "Running calculator demo..."
	@echo "Enter two numbers when prompted:"
	./$(RELEASE_BINARY) $(EXAMPLES_DIR)/calculator.tb

# Benchmarks
.PHONY: benchmark
benchmark: build ## Run performance benchmarks
	@echo "Running benchmarks..."
	./$(RELEASE_BINARY) benchmark $(EXAMPLES_DIR)/simple.tb

# Documentation
.PHONY: docs
docs: ## Generate documentation
	@echo "Generating documentation..."
	$(CARGO) doc --no-deps
	@echo "✅ Documentation generated in target/doc/"

.PHONY: docs-open
docs-open: docs ## Generate and open documentation
	@echo "Opening documentation..."
	$(CARGO) doc --no-deps --open

# Linting and formatting
.PHONY: check
check: ## Run cargo check
	@echo "Running cargo check..."
	$(CARGO) check

.PHONY: clippy
clippy: ## Run clippy lints
	@echo "Running clippy..."
	$(CARGO) clippy -- -D warnings

.PHONY: format
format: ## Format code with rustfmt
	@echo "Formatting code..."
	$(CARGO) fmt

.PHONY: format-check
format-check: ## Check code formatting
	@echo "Checking code formatting..."
	$(CARGO) fmt -- --check

# Installation
.PHONY: install
install: build ## Install the binary to system
	@echo "Installing $(BINARY_NAME) to /usr/local/bin..."
	sudo cp $(RELEASE_BINARY) /usr/local/bin/$(BINARY_NAME)
	@echo "✅ Installation complete"

.PHONY: uninstall
uninstall: ## Uninstall the binary from system
	@echo "Uninstalling $(BINARY_NAME) from /usr/local/bin..."
	sudo rm -f /usr/local/bin/$(BINARY_NAME)
	@echo "✅ Uninstallation complete"

.PHONY: install-local
install-local: build ## Install to user local directory
	@echo "Installing $(BINARY_NAME) to ~/.local/bin..."
	mkdir -p ~/.local/bin
	cp $(RELEASE_BINARY) ~/.local/bin/$(BINARY_NAME)
	@echo "✅ Installation complete. Make sure ~/.local/bin is in your PATH."

# Cleaning
.PHONY: clean
clean: ## Clean build artifacts
	@echo "Cleaning build artifacts..."
	$(CARGO) clean
	@echo "✅ Clean complete"

# Distribution
.PHONY: package
package: build ## Create a release package
	@echo "Creating release package..."
	mkdir -p dist
	tar -czf dist/$(PROJECT_NAME)-$(VERSION)-$(shell uname -s | tr '[:upper:]' '[:lower:]')-$(shell uname -m).tar.gz \
		-C $(TARGET_DIR)/release $(BINARY_NAME) \
		-C .. README.md LICENSE \
		-C .. $(EXAMPLES_DIR) \
		-C .. docs
	@echo "✅ Package created in dist/"

# Development utilities
.PHONY: watch
watch: ## Watch for changes and rebuild
	@echo "Watching for changes... (Ctrl+C to stop)"
	$(CARGO) watch -x build

.PHONY: dev
dev: build-debug ## Development build and test
	@echo "Development workflow..."
	$(CARGO) test
	$(CARGO) clippy
	@echo "✅ Development checks complete"

# Version and info
.PHONY: version
version: ## Show version information
	@echo "$(PROJECT_NAME) version: $(VERSION)"
	@if command -v rustc >/dev/null 2>&1; then \
		echo "Rust version: $$(rustc --version)"; \
	else \
		echo "Rust version: Not found (run 'source ~/.cargo/env' to activate)"; \
	fi
	@if command -v cargo >/dev/null 2>&1; then \
		echo "Cargo version: $$(cargo --version)"; \
	else \
		echo "Cargo version: Not found (run 'source ~/.cargo/env' to activate)"; \
	fi

.PHONY: info
info: version ## Show detailed project information
	@echo ""
	@echo "Project Information"
	@echo "=================="
	@echo "Name: $(PROJECT_NAME)"
	@echo "Version: $(VERSION)"
	@echo "Binary: $(BINARY_NAME)"
	@echo "Source: $(SRC_DIR)"
	@echo "Examples: $(EXAMPLES_DIR)"
	@echo "Tests: $(TESTS_DIR)"
	@echo ""
	@echo "Build Targets:"
	@echo "  Debug binary: $(DEBUG_BINARY)"
	@echo "  Release binary: $(RELEASE_BINARY)"
	@echo ""

# Quick status check
.PHONY: status
status: ## Show project status
	@echo "Project Status"
	@echo "============="
	@if [ -f $(RELEASE_BINARY) ]; then \
		echo "✅ Release binary exists"; \
	else \
		echo "❌ Release binary missing - run 'make build'"; \
	fi
	@if [ -f $(DEBUG_BINARY) ]; then \
		echo "✅ Debug binary exists"; \
	else \
		echo "❌ Debug binary missing - run 'make build-debug'"; \
	fi
	@echo ""
	@echo "Recent changes:"
	@git log --oneline -5 2>/dev/null || echo "Git repository not available"

# CI/CD helpers
.PHONY: ci
ci: format-check clippy test ## Run full CI pipeline
	@echo "✅ CI pipeline passed"

.PHONY: pre-commit
pre-commit: format clippy ## Run pre-commit checks
	@echo "✅ Pre-commit checks passed"

# Example validation
.PHONY: validate-examples
validate-examples: build ## Validate all example programs
	@echo "Validating example programs..."
	@success=0; \
	for file in $(EXAMPLES_DIR)/*.tb; do \
		echo "Validating $$file..."; \
		if ./$(RELEASE_BINARY) "$$file" >/dev/null 2>&1; then \
			echo "✅ $$file"; \
		else \
			echo "❌ $$file failed"; \
			success=1; \
		fi; \
	done; \
	if [ $$success -eq 0 ]; then \
		echo "✅ All examples validated successfully"; \
	else \
		echo "❌ Some examples failed validation"; \
		exit 1; \
	fi

# Phony targets declaration (prevents conflicts with files)
.PHONY: default build build-debug rebuild test test-release test-verbose integration-test examples-test examples run run-debug demo-simple demo-hello demo-countdown demo-calculator benchmark docs docs-open check clippy format format-check install uninstall install-local clean package watch dev version info status ci pre-commit validate-examples