use crate::engine::models::types::Bitboard;

pub enum Piece {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

pub struct Pawn(pub Bitboard);

impl Pawn {
}

pub struct Knight(pub Bitboard);

impl Knight {
}

pub struct Bishop(pub Bitboard);

impl Bishop {
}

pub struct Rook(pub Bitboard);

impl Rook {
}

pub struct Queen(pub Bitboard);

impl Queen {
}

pub struct King(pub Bitboard);

impl King {
}
