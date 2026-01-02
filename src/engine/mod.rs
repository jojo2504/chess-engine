/// The chess engine module containing core game logic and models.
///
/// This module provides the main chess engine implementation, game models,
/// and internal magic bitboard functionality for move generation.
pub mod models;
/// Core chess engine implementation and evaluation logic.
#[allow(clippy::module_inception)]
pub mod engine;
/// Internal magic bitboard implementation for efficient move generation.
pub(crate) mod magic;