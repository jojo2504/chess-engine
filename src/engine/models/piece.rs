use crate::engine::models::types::Bitboard;

pub enum Piece {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

pub struct Pawn(Bitboard);

impl Pawn {
}

pub struct Knight {
    
}

impl Knight {
}

pub struct Bishop {
    
}

impl Bishop {
}

pub struct Rook {
    
}

impl Rook {
}

pub struct Queen {
    
}

impl Queen {
}

pub struct King {
    
}

impl King {
}