#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

pub mod engine;

#[cfg(test)]
mod tests {
    use crate::engine::models::{board::{Chessboard, Color, Square, get_piece_index}, piece::Piece};

    #[test]
    fn slide_test() {
        let mut chessboard = Chessboard::new();
        println!("{}", chessboard);
        
        chessboard.slide_piece(get_piece_index(Color::White, Piece::Pawn), Square::A2.bitboard(), Square::A4.bitboard(), Color::White, Piece::Pawn);
        println!("{}", chessboard);

        println!("{}", chessboard.pieces[get_piece_index(Color::White, Piece::King)])
    }
}