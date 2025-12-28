use std::{collections::HashMap, fmt};

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

    pub fn from_fen(fen: &str) -> Self {
        let halfmoves: u8 = 0;
        let fullmoves: u8 = 0;

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