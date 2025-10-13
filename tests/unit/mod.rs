//! Comprehensive unit test suite for the Tubular interpreter
//!
//! This module contains unit tests for all core components of the Tubular interpreter,
//! ensuring correct behavior and preventing regressions.

pub mod types;
pub mod interpreter;
pub mod operations;
pub mod parser;
pub mod cli;
pub mod property_tests;
pub mod benchmarks;

// Re-export test utilities for convenience
pub use tubular::tests_common::*;