use std::{collections::HashMap, sync::OnceLock};

use crate::engine::{models::{board::{Chessboard, Color}, piece::Piece}, movegen::generate_legal_moves};

/// First simple evaluation function
pub struct Evaluation {
    /// piece index -> score
    pieces_score: HashMap<i32, i32>,
    pawn_score: [i32; 64],
    kings_score: [i32; 64],
    knight_score: [i32; 64],
}

impl Evaluation {
    /// Evaluate the position by the value of pieces for now
    pub fn evaluate(chessboard: &mut Chessboard) -> i32 {
        let mut board_evaluation: i32 = 0;
        let eval = evaluation();

        // check for checkmate first
        if generate_legal_moves(chessboard).len() == 0 && chessboard.is_in_check() {
            let sign = match chessboard.get_current_turn() {
                Color::White => 1,
                Color::Black => -1,
            };
            return 10000 * sign
        }
        
        for (index, piece) in chessboard.pieces.iter().enumerate() {
            let piece_type = index % 6;
            let is_white = index < 6;
            let sign = if is_white { 1 } else { -1 };
            
            match Piece::ALL[index % 6] {
                Piece::Pawn => {
                    let mut bits = *piece;
                    while bits != 0 {
                        let square = bits.trailing_zeros() as usize;
                        let pos = if is_white { square } else { 63 - square };
                        board_evaluation += sign * (eval.pieces_score.get(&(piece_type as i32)).unwrap() + eval.pawn_score[pos]);
                        bits &= bits - 1;
                    }
                },
                Piece::Knight => {
                    let mut bits = *piece;
                    while bits != 0 {
                        let square = bits.trailing_zeros() as usize;
                        let pos = if is_white { square } else { 63 - square };
                        board_evaluation += sign * (eval.pieces_score.get(&(piece_type as i32)).unwrap() + eval.knight_score[pos]);
                        bits &= bits - 1;
                    }
                },
                Piece::King => {
                    let mut bits = *piece;
                    while bits != 0 {
                        let square = bits.trailing_zeros() as usize;
                        let pos = if is_white { square } else { 63 - square };
                        board_evaluation += sign * eval.kings_score[pos];
                        bits &= bits - 1;
                    }
                },
                _ => {
                    board_evaluation += sign * (piece.count_ones() as i32) * (eval.pieces_score.get(&(piece_type as i32)).unwrap());
                }
            }
        }
        board_evaluation
    }
}

/// Lazy static initializer for [Evaluation].
fn evaluation() -> &'static Evaluation {
    static EVALUATION: OnceLock<Evaluation> = OnceLock::new();
    EVALUATION.get_or_init(|| {
        let mut evaluation = Evaluation {
            pieces_score: HashMap::new(),
            pawn_score: [
                0, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 2, 2, 1, 1, 1,
                1, 1, 2, 3, 3, 2, 1, 1,
                2, 2, 3, 3, 3, 3, 2, 2,
                3, 3, 3, 3, 3, 3, 3, 3,
                4, 4, 4, 4, 4, 4, 4, 4,
                0, 0, 0, 0, 0, 0, 0, 0,
                ],
                kings_score: [
                1002, 1002, 1002, 1001, 1001, 1002, 1002, 1002,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                1002, 1002, 1002, 1001, 1001, 1002, 1002, 1002,
                ],
            knight_score: [
                3, 3, 3, 3, 3, 3, 3, 3,
                3, 3, 3, 4, 4, 3, 3, 3,
                3, 3, 4, 5, 5, 4, 3, 3,
                3, 4, 5, 6, 6, 5, 4, 3,
                3, 4, 5, 6, 6, 5, 4, 3,
                3, 3, 4, 5, 5, 4, 3, 3,
                3, 3, 3, 4, 4, 3, 3, 3,
                3, 3, 3, 3, 3, 3, 3, 3,
            ]
        };

        evaluation.pieces_score.insert(0, 1);
        evaluation.pieces_score.insert(1, 3);
        evaluation.pieces_score.insert(2, 3);
        evaluation.pieces_score.insert(3, 5);
        evaluation.pieces_score.insert(4, 9);
        evaluation.pieces_score.insert(5, 1000);

        evaluation
    })
}