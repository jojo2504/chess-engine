use crate::engine::models::{board::Square, piece::Piece};

#[derive(Default, Clone, Copy)]
pub struct Undo {
    pub captured_piece: Option<Piece>,
    /// Saved previous en passant square if any
    pub en_passant_square: Option<Square>,
    /// Represents the last casting rights before the move, so we can restore the state in the unmake function.
    pub castling_right: u8,
    /// Saved half move clock before the move
    pub half_move_clock: u32,
}
