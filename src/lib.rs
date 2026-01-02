use crate::engine::{models::{board::Chessboard, r#move::Move}, movegen::generate_moves};

pub mod engine;
pub mod utils;

/// Performs a `perft` performance and debugging test returning the total number of positions at the end
pub fn perft(chessboard: &mut Chessboard, depth: u8) -> u64 {
    if depth == 0 {
        return 1u64;
    }

    let mut all_pseudo_legal_moves: Vec<Move> = Vec::with_capacity(256);
    let mut nodes = 0;

    generate_moves(chessboard, &mut all_pseudo_legal_moves);
    let n_moves = all_pseudo_legal_moves.len();
    for _move in all_pseudo_legal_moves.iter().take(n_moves) {
        chessboard.make(_move);
        if !chessboard.is_in_check(chessboard.state_stack[chessboard.ply_index].turn_color) {
            nodes += perft(chessboard, depth - 1);
        }
        chessboard.unmake(_move);
    }

    nodes
}

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

    #[test]
    fn convert_square() {
        let index = 63;
        let square = Square::try_from(index).unwrap();

        println!("{:?}", square);
    }
}