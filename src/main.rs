#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use chess_engine::engine::models::{board::{Chessboard, Color, Square}, piece::Piece};

fn main() -> anyhow::Result<()> {
    let mut chessboard = Chessboard::new();
    // Move a pawn from A2 to A3
    chessboard.slide_piece(&mut chessboard.get_piece(Color::White, Piece::Pawn), Square::A2.bitboard(), Square::A3.bitboard(), Color::White);
    // Remove the captured piece
    chessboard.toggle_piece(&mut chessboard.get_piece(Color::White, Piece::Pawn), Square::A2.bitboard(), Color::White);
    Ok(())
}
