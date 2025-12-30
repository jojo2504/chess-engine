#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use crate::engine::models::{piece::Piece};

/// Quick enum to match move kinds
#[derive(Debug, Clone, Copy)]
enum MoveKind {
    QuietMoves = 0,
    DoublePawnPush = 1,
    KingCastle = 2,
    QueenCastle = 3,
    Captures = 4,
    EpCapture = 5,
    KnightPromotion = 8,
    BishopPromotion = 9,
    RookPromotion = 10,
    QueenPromotion = 11,
    KnightPromotionCapture = 12,
    BishopPromotionCapture = 13,
    RookPromotionCapture = 14,
    QueenPromotionCapture = 15
}

#[derive(PartialEq, Eq)]
pub struct Move {
    pub word: u16,
    pub piece_type: Piece,
    pub from: u64,
    pub to: u64
}

impl Move {
    pub fn new(word: u16, piece_type: Piece) -> Self {
        let from = 1u64 << (word >> 10);
        let to = 1u64 << ((word >> 4) & 0x3F);
        
        Self {
            word,
            piece_type,
            from,
            to,
        }
    }
    
    #[inline]
    pub fn move_kind_code(&self) -> u8 {
        (self.word & 0b1111) as u8
    }
    
    #[inline]
    pub fn castle_flag(&self) -> bool {
        (self.word & 0b1010) == 0b0010
    }
    
    #[inline]
    pub fn capture_flag(&self) -> bool {
        (self.word & 0b0100) != 0
    }
    
    #[inline]
    pub fn promotion_flag(&self) -> bool {
        (self.word & 0b1000) != 0
    }
    
    #[inline]
    pub fn piece_type(&self) -> Piece {
        self.piece_type
    }

    pub fn get_ucis() -> Vec<String> {
        todo!()
    }
    
    /// Decodes incoming uci encoded move into a `Move` object.
    pub fn decode_uci() -> Move {
        todo!()
    }
    
    /// Used by UCI to decode a move from a string when played from the GUI.
    pub fn get_move_kind() {
        todo!()
    }
}