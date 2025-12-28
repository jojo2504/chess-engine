use std::{collections::HashMap, fmt};

/// Represents a board rank, or horizontal line. `A1..H1`
pub enum Rank {
    Rank1, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8
}

/// Represents a board file, or vertical line. `A1..A8`
pub enum File {
    FileA, FileB, FileC, FileD, FileE, FileF, FileG, FileH
}

/// Reprensents one of the two piece's color
pub enum Color {
    White,
    Black
}

#[repr(u64)]
enum Board {
    EMPTY = 0u64,
    FULL = u64::MAX
}

type Bitboard = u64;

/**
 * Describe the pieces of the board,
 * solely from a specific side.
 * 
 * Does consider exclusively the very position
 * of the pieces. Everything else is ignored.
 */
struct OneSideBoard {
    pawns: u64,
    rooks: u64,
    knights: u64,
    bishops: u64,
    queens: u64,
    king: u64,
}

pub struct Chessboard {
    white_pieces: OneSideBoard,
    black_pieces: OneSideBoard,

    checkmated: bool,
    stalemated: bool,
}

/**
 * |R|N|B|Q|K|B|N|R|
 * |P|P|P|P|P|P|P|P|
 * | | | | | | | | |
 * | | | | | | | | |
 * | | | | | | | | |
 * | | | | | | | | |
 * | | | | | | | | |
 * | | | | | | | | |
*/
impl Chessboard {
    /// Default chessboard's constructor initilized with the default fen value, or classic starting position 
    pub fn new() -> Self {
        let black_pieces = OneSideBoard {
            rooks  : 0b10000001 << 56,
            knights: 0b01000010 << 56,
            bishops: 0b00100100 << 56,
            queens : 0b00010000 << 56,
            king   : 0b00001000 << 56,
            pawns  : 0b11111111 << 48,
        };

        let white_pieces = OneSideBoard {
            rooks  : 0b10000001,
            knights: 0b01000010,
            bishops: 0b00100100,
            queens : 0b00010000,
            king   : 0b00001000,
            pawns  : 0b11111111 << 8,
        };
        Chessboard { white_pieces, black_pieces, checkmated: false, stalemated: false }
    }

    pub fn from_fen(fen: &str) -> Result<Self, &'static str> {
        let halfmoves: u8 = 0;
        let fullmoves: u8 = 0;

        Ok(Chessboard { white_pieces: (), black_pieces: (), checkmated: (), stalemated: () })
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