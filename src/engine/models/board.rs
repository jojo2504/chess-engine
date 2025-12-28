use std::fmt;

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

pub struct Chessboard {

}

impl Chessboard {
    /// Default chessboard's constructor initilized with the default fen value, or classic starting position 
    pub fn new() -> Self {
        todo!()
    }

    /// Special chessboard's constructor for initilizing a custom position and state using a custom fen value
    /// ```rust
    /// Chessboard::with_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    pub fn with_fen(fen: &str) -> Self {
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