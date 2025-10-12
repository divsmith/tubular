# Technology Research: Tubular Language Interpreter

**Feature**: `001-a-basic-initial` | **Date**: 2025-10-11 | **Spec**: [link](spec.md)

## Primary Language Decision

**Decision**: Rust for production implementation
**Rationale**:
- Zero-cost abstractions and predictable performance for 1000+ concurrent droplets
- Memory safety without garbage collector prevents pauses in large grid simulations
- Strong static typing catches complex state management bugs at compile time
- Excellent ecosystem for CLI applications, testing, and data structures
- Built-in support for safe concurrent programming

**Alternative considered**: Python with NumPy/PyPy for rapid prototyping - rejected due to performance concerns with large-scale droplet simulation and GIL limitations for true parallelism.

## Core Dependencies

**Decision**: CLI-based Rust application with targeted libraries
**Rationale**:
- `clap` for CLI argument parsing (industry standard, derive macros)
- `ndarray` for efficient 2D array operations (better performance than naive Vec<Vec<>>)
- `num-bigint` for arbitrary precision integers (requirement from spec)
- `anyhow`/`thiserror` for comprehensive error handling
- `proptest` for property-based testing (critical for complex state transitions)
- `criterion` for performance benchmarking

**Storage**: File-based grid input (ASCII files) - no database needed
**Testing**: Rust's built-in test framework + proptest + criterion

## Target Platform & Performance

**Target Platform**: Cross-platform CLI tool (Linux, macOS, Windows)
**Rationale**: Language interpreters are typically developer tools used across platforms

**Performance Goals**:
- Execute 1000 concurrent droplets without >10% degradation per 100 additional droplets
- Process 1000x1000 grid within memory limits
- Discrete tick execution with <1ms per tick for typical programs

**Constraints**:
- Memory usage scales with active grid area, not total grid size
- Stack depth minimum of 1000 levels
- Arbitrary precision integer support (unbounded memory for numbers)

**Scale/Scope**: Single interpreter executable handling multiple program files

## Architecture Decisions

**Project Structure**: Single project with modular internal structure
**Rationale**: Interpreter is a cohesive unit - unnecessary to split into multiple projects

**Key Architectural Patterns**:
- Sparse grid representation for memory efficiency
- Object pooling for droplet management
- Batch processing by destination for collision detection
- Chunked grid for large program support
- Spatial partitioning for performance optimization

## Testing Strategy

**Decision**: Comprehensive testing with property-based approach
**Rationale**: Complex state transitions and edge cases require systematic testing

**Testing Layers**:
1. Unit tests for individual operations (arithmetic, stack, memory)
2. Integration tests for complete program execution
3. Property-based tests for invariants (collision detection, stack behavior)
4. Performance benchmarks for scalability validation
5. Snapshot tests for example program outputs

## Implementation Complexity

**No major violations** of simplicity principles:
- Single cohesive interpreter (not over-engineered)
- Clear separation of concerns (parsing, execution, operations)
- Performance optimizations are targeted and justified
- Testing approach is comprehensive but focused on real requirements

## Risk Mitigation

**Technical Risks Addressed**:
- Memory usage: Sparse representation and chunked grid
- Performance: Object pooling and batch processing
- Correctness: Property-based testing and comprehensive validation
- Maintainability: Clear module boundaries and strong typing

**Development Approach**: Incremental implementation with testing at each phase