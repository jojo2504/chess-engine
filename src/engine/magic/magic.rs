#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::fs::{self};
use serde::Deserialize;
use crate::engine::models::board::Square;

/// Represent a magic bitboard for a given square along with the mask of the piece (rook or bishop) 
#[derive(Debug, Deserialize)]
pub struct Magic {
    /// Square the piece is on during the move-bitboard generation technique.
    pub square: Square,
    #[serde(rename = "magicNumber")]
    /// Also named magic bitboard, magic 64-bit factor.
    pub magic_number: u64,
    /// The relevant occupancy bits to form a key. For example if you had a rook on a1, the relevant occupancy bits will be from a2-a7 and b1-g1.
    pub mask: u64,
}

impl Magic {
    /// Helper method to load magic table for both [chess_engine::engine::models::piece::Rook] and bishop
    pub fn load_magic_table(path: &str) -> anyhow::Result<Vec<Magic>> {
        let json_str = fs::read_to_string(path)?;
        let magics: Vec<Magic> = serde_json::from_str(&json_str)?;
        Ok(magics)
    }
}