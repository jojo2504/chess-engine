use std::{collections::HashMap, fmt};

use crate::engine::models::{r#move::Move, piece::Piece, state::State};

/// Represents a board rank, or horizontal line. `A1..H1`
pub enum Rank {
    Rank1, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8
}

impl Rank {
    pub fn get_mask(self) -> u64 {
        use Rank::*;
        match self {
            Rank1 => 0xFF,
            Rank2 => 0xFF00,
            Rank3 => 0xFF0000,
            Rank4 => 0xFF000000,
            Rank5 => 0xFF00000000,
            Rank6 => 0xFF0000000000,
            Rank7 => 0xFF000000000000,
            Rank8 => 0xFF00000000000000,
        }
    }
        
    pub fn get_clear(self) -> u64 {
        use Rank::*;
        match self {
            Rank1 => 0xFFFFFFFFFFFFFF00,
            Rank2 => 0xFFFFFFFFFFFF00FF,
            Rank3 => 0xFFFFFFFFFF00FFFF,
            Rank4 => 0xFFFFFFFF00FFFFFF,
            Rank5 => 0xFFFFFF00FFFFFFFF,
            Rank6 => 0xFFFF00FFFFFFFFFF,
            Rank7 => 0xFF00FFFFFFFFFFFF,
            Rank8 => 0x00FFFFFFFFFFFFFF,
        }
    }
}

/// Represents a board file, or vertical line. `A1..A8`
pub enum File {
    FileA, FileB, FileC, FileD, FileE, FileF, FileG, FileH
}

impl File {
    pub fn get_mask(self) -> u64 {
        use File::*;
        match self {
            FileA => 0x0101010101010101,
            FileB => 0x0202020202020202,
            FileC => 0x0404040404040404,
            FileD => 0x0808080808080808,
            FileE => 0x1010101010101010,
            FileF => 0x2020202020202020,
            FileG => 0x4040404040404040,
            FileH => 0x8080808080808080,
        }
    }

    pub fn get_clear(self) -> u64 {
        use File::*;
        match self {
            FileA => 0xFEFEFEFEFEFEFEFE,
            FileB => 0xFDFDFDFDFDFDFDFD,
            FileC => 0xFBFBFBFBFBFBFBFB,
            FileD => 0xF7F7F7F7F7F7F7F7,
            FileE => 0xEFEFEFEFEFEFEFEF,
            FileF => 0xDFDFDFDFDFDFDFDF,
            FileG => 0xBFBFBFBFBFBFBFBF,
            FileH => 0x7F7F7F7F7F7F7F7F,
        }
    }
}

/// Reprensents one of the two piece's color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black
}

/// Constant values of a board state.
#[repr(u64)]
pub enum Board {
    EMPTY = 0u64,
    FULL = u64::MAX
}

impl Board {
    pub fn get_corner_clear() -> u64 {
        0x7EFFFFFFFFFFFF7E
    }

    pub fn get_corner_mask() -> u64 {
        0x8100000000000081
    }

    pub fn get_all_border_clear() -> u64 {
        0x7E7E7E7E7E7E00
    } 

    pub fn get_all_border_mask() -> u64 {
        0xFF818181818181FF
    }
}

#[repr(u64)]
pub enum Square {
    A1 = 1u64 << 0, B1 = 1u64 << 1, C1 = 1u64 << 2, D1 = 1u64 << 3, E1 = 1u64 << 4, F1 = 1u64 << 5, G1 = 1u64 << 6, H1 = 1u64 << 7,
    A2 = 1u64 << 8, B2 = 1u64 << 9, C2 = 1u64 << 10, D2 = 1u64 << 11, E2 = 1u64 << 12, F2 = 1u64 << 13, G2 = 1u64 << 14, H2 = 1u64 << 15, 
    A3 = 1u64 << 16, B3 = 1u64 << 17, C3 = 1u64 << 18, D3 = 1u64 << 19, E3 = 1u64 << 20, F3 = 1u64 << 21, G3 = 1u64 << 22, H3 = 1u64 << 23,
    A4 = 1u64 << 24, B4 = 1u64 << 25, C4 = 1u64 << 26, D4 = 1u64 << 27, E4 = 1u64 << 28, F4 = 1u64 << 29, G4 = 1u64 << 30, H4 = 1u64 << 31,
    A5 = 1u64 << 32, B5 = 1u64 << 33, C5 = 1u64 << 34, D5 = 1u64 << 35, E5 = 1u64 << 36, F5 = 1u64 << 37, G5 = 1u64 << 38, H5 = 1u64 << 39,
    A6 = 1u64 << 40, B6 = 1u64 << 41, C6 = 1u64 << 42, D6 = 1u64 << 43, E6 = 1u64 << 44, F6 = 1u64 << 45, G6 = 1u64 << 46, H6 = 1u64 << 47,
    A7 = 1u64 << 48, B7 = 1u64 << 49, C7 = 1u64 << 50, D7 = 1u64 << 51, E7 = 1u64 << 52, F7 = 1u64 << 53, G7 = 1u64 << 54, H7 = 1u64 << 55,
    A8 = 1u64 << 56, B8 = 1u64 << 57, C8 = 1u64 << 58, D8 = 1u64 << 59, E8 = 1u64 << 60, F8 = 1u64 << 61, G8 = 1u64 << 62, H8 = 1u64 << 63,
}

impl From<u64> for Square {
    fn from(index: u64) -> Self {
        unsafe { std::mem::transmute(index) }
    }
}

/// ```txt
/// |r|n|b|q|k|b|n|r|
/// |p|p|p|p|p|p|p|p|
/// | | | | | | | | |
/// | | | | | | | | |
/// | | | | | | | | |
/// | | | | | | | | |
/// |P|P|P|P|P|P|P|P|
/// |R|N|B|Q|K|B|N|R|
/// ```
pub struct Chessboard {
    pub pieces: [u64; 12],
    pub white_pieces: u64,
    pub black_pieces: u64,

    pub state: State
}

impl Chessboard {
    /// Default chessboard's constructor initilized with the default fen value, or classic starting position 
    pub fn new() -> Self {
        let pieces = [
            // White pieces (indices 0-5)
            0b11111111 << 8,           // White pawns
            0b10000001,                // White rooks
            0b01000010,                // White knights
            0b00100100,                // White bishops
            0b00010000,                // White queens
            0b00001000,                // White king
            // Black pieces (indices 6-11)
            0b11111111 << 48,          // Black pawns
            0b10000001 << 56,          // Black rooks
            0b01000010 << 56,          // Black knights
            0b00100100 << 56,          // Black bishops
            0b00010000 << 56,          // Black queens
            0b00001000 << 56,          // Black king
        ];
        
        let white_pieces = pieces[0] | pieces[1] | pieces[2] | pieces[3] | pieces[4] | pieces[5];
        let black_pieces = pieces[6] | pieces[7] | pieces[8] | pieces[9] | pieces[10] | pieces[11];
        
        Chessboard { pieces, white_pieces, black_pieces, state: State::default()}
    }

    #[inline]
    pub fn get_piece(&self, color: Color, piece: Piece) -> u64 {
        self.pieces[color as usize * 6 + piece as usize]
    }

    pub fn from_fen(fen: &str) -> Self {
        todo!()
    }

    /// Use this method when required to "slide" a piece, meaning a piece leaving its starting square and ending on its destination square.
    /// 
    /// You should also combine it with [Move::toggle_piece()] when capturing pieces.
    /// 
    /// # Exemple 
    /// ```rust
    /// use chess_engine::engine::models::r#move::Move;
    /// 
    /// let chessboard = Chessboard::new();
    /// // Move a bishop to e4
    /// Move::slide_piece();
    /// // Remove the captured piece
    /// Move::toggle_piece(...);
    /// ```
    #[inline]
    pub fn slide_piece(piece_bitboard: &mut u64, color_piece_bitboard: &mut u64, from: u64, to: u64) {
        *piece_bitboard ^= from ^ to;
        *color_piece_bitboard ^= from ^ to;
    }

    /// Use this method when required to put a piece without moving one or removing a piece, like during game initialization, captures or promotions.
    pub fn toggle_piece(&mut self, square: u64, piece: Piece, side: Color) {
        todo!()
    }

    /// Make a move on the chessboard itself.
    pub fn make(&mut self, r#move: Move) {
        todo!()
    }
    
    /// Unmake a move on the chessboard itself.
    pub fn unmake(&mut self, r#move: Move) {
        todo!()
    }
    
    pub fn get_all_possible_moves() {
        todo!()
    }
    
    /// Checks if the current tested side king is in check or not
    pub fn is_in_check(side: Color) -> bool {
        todo!()
    }
    
    /// Performs a `perft` performance and debugging test returning the total number of positions at the end
    pub fn perft(&self, depth: u8) -> u64 {
        todo!()
    }
}

impl fmt::Display for Chessboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}