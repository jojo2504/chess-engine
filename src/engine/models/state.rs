#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use crate::engine::models::board::{Color, Square};

#[derive(Debug, Clone, Copy)]
/// Fields that can't be altered during the move but can be altered between moves
/// Which means that we restore a position from these fields
/// These fields indicate that at a given state, this move with this state has been played
/// For example, at white turn, halfmoveclock is 10, can white king castle, he decided to capture a piece, and didnt have en passant available 
/// So we need to push to the stack the state before updating anything 
pub struct State {
    pub(crate) turn_color: Color,
    pub(crate) half_move_clock: u32,
    pub(crate) castling_right: u8,
    pub(crate) en_passant_square: Option<Square>,
    // zobrirst key used to create the tranposition table.
    pub(crate) zobrist_key: u64
}

impl State {
    #[inline(always)]
    pub fn can_white_king_castle(&self) -> bool {
    self.castling_right & 1 != 0
    }

    #[inline(always)]
    pub fn can_white_queen_castle(&self) -> bool {
        self.castling_right & 2 != 0
    }
    
    #[inline(always)]
    pub fn can_black_king_castle(&self) -> bool {
        self.castling_right & 4 != 0
    }    
    
    #[inline(always)]
    pub fn can_black_queen_castle(&self) -> bool {
        self.castling_right & 8 != 0
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            turn_color: Color::White, 
            half_move_clock: 0, 
            castling_right: 0,
            en_passant_square: None,
            zobrist_key: 0 
        }
    }
}