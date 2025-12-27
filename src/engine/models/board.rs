use std::fmt;

pub enum Rank {
    Rank1, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8
}

pub enum File {
    FileA, FileB, FileC, FileD, FileE, FileF, FileG, FileH
}

pub enum TurnColor {
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
    pub fn new() -> Self {
        todo!()
    }

    pub fn with_fen(fen: &str) -> Self {
        todo!()
    }

    fn parse_fen(&self, fen: String) {
        todo!()
    }

    pub fn perft(&self, depth: u8) -> u64 {
        todo!()
    }
}

impl fmt::Display for Chessboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}