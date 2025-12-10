//! Rust utilities for Advent of Code problems
//!
//! This crate provides common data structures and algorithms that are frequently
//! needed when solving Advent of Code puzzles.

pub mod grid;
pub mod ranges;

// Re-export commonly used items for convenience
pub use grid::{check_adjacent, parse_to_char_grid, parse_to_grid, Coordinate};
pub use ranges::ranges_overlap;
