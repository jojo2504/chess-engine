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

/// All data needed to encode one move.
#[derive(PartialEq, Eq)]
pub struct Move {
    /// The information required to uniquely describe a move is the initial square, also called from-, origin- or departure square, and the target square, 
    /// also called to- or destination square, and in case of promotions the promoted piece code. While this from-to information is also sufficient for castling
    /// in standard chess, due to the otherwise impossible double king step, it might not in Chess960. 
    /// Therefore and also for efficiency reasons, castles are tagged as "special" moves. 
    /// Such a move encoding conveniently fits inside a 16-bit word, 6 bits for from-to square each to index a butterfly board, 
    /// still leaves a nibble for flags for move kind and promoted piece code, for instance this arbitrary flags.
    /// 
    /// # Representation
    /// - from: 6 bytes, `0..63` (index of the square)
    /// - to: 6 bytes, `0..63` (index of the square)
    /// - special: 4 bytes, refers to [MoveKind]
    /// ```txt
    /// from   to     special
    /// 000000 000000 0000
    /// ```
    pub word: u16,
    /// Stored which type of piece this move moves.
    pub piece_type: Piece,
    /// Cached from- square as `u64` calculated in the constructor.
    pub from: u64,
    /// Cached to- square as `u64` calculated in the constructor.
    pub to: u64
}

impl Move {
    /// [Move]'s constructor taking a `word` and a `piece type`.
    pub fn from(word: u16, piece_type: Piece) -> Self {
        let from = 1u64 << (word >> 10);
        let to = 1u64 << ((word >> 4) & 0x3F);
        
        Self {
            word,
            piece_type,
            from,
            to,
        }
    }
    
    /// Returns the move kind code as `u8`. Note that it's representing the values of [MoveKind] variants.
    #[inline]
    pub fn move_kind_code(&self) -> u8 {
        (self.word & 0b1111) as u8
    }
    
    /// Checks if the move is a `castle`.
    #[inline]
    pub fn castle_flag(&self) -> bool {
        (self.word & 0b1010) == 0b0010
    }
    
    #[inline]
    /// Checks if the move is a `capture`.
    pub fn capture_flag(&self) -> bool {
        (self.word & 0b0100) != 0
    }
    
    #[inline]
    /// Checks if the move is a `promotion`.
    pub fn promotion_flag(&self) -> bool {
        (self.word & 0b1000) != 0
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