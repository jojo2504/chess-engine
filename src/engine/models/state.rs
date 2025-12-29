#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]

use crate::engine::models::{board::{Color, 
    Square}, piece::Piece};

#[derive(Clone, Copy)]
pub struct State {
    // Fields that can't be altered during the move but can be altered between moves
    // Which means that we restore a position from these fields
    // These fields indicate that at a given state, this move with this state has been played
    // example : at white turn, halfmoveclock is 10, can white king castle, he decided to capture a piece, and didnt have en passant available 
    // So we need to push to the stack the state before updating anything 
    pub turn_color: Color,
    pub full_move_number: u32,
    pub half_move_clock: u32,
    pub can_white_king_castle: bool,
    pub can_white_queen_castle: bool,
    pub can_black_king_castle: bool,
    pub can_black_queen_castle: bool,
    pub en_passant_square: Option<Square>, // null if none

    // Fields that can be altered right during the move
    // Which means that after making a move, the new state has these fields updated, we need to push after this
    pub captured_piece: Option<Piece>,  // null if none

    // Fields that doesn't affect restoration
    pub checkmated: bool,
    pub stalemated: bool,
    pub zobrist_hash_key: u64 // Zobrist hash key for the position
}

impl Default for State {
    fn default() -> Self {
        Self { 
            turn_color: Color::White, 
            full_move_number: 0, 
            half_move_clock: 0, 
            can_white_king_castle: true, 
            can_white_queen_castle: true, 
            can_black_king_castle: true, 
            can_black_queen_castle: true, 
            en_passant_square: None, 
            captured_piece: None, 
            checkmated: false, 
            stalemated: false, 
            zobrist_hash_key: 0 
        }
    }
}