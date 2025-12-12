//! Rust utilities for Advent of Code problems
//!
//! This crate provides common data structures and algorithms that are frequently
//! needed when solving Advent of Code puzzles.

pub mod file;
pub mod grid;
pub mod ranges;

// Re-export commonly used items for convenience
pub use file::read_lines_from_file;
pub use grid::Coordinate;
pub use ranges::ranges_overlap;
