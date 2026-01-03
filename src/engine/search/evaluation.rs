use std::{collections::HashMap, sync::OnceLock};

use crate::engine::models::{board::Chessboard, piece::Piece};

/// First simple evaluation function
pub struct Evaluation {
    /// piece index -> score
    pieces_score: HashMap<i32, i32>
}

impl Evaluation {
    /// Evaluate the position by the value of pieces for now
    pub fn evaluate(chessboard: &Chessboard) -> i32 {
        let mut board_evaluation: i32 = 0;
        for (index, piece) in chessboard.pieces.iter().enumerate() {
            if index < 6 {
                board_evaluation += (piece.count_ones() as i32) * (evaluation().pieces_score.get(&(index as i32 % 6)).unwrap());
            }
            else {
                board_evaluation -= (piece.count_ones() as i32) * (evaluation().pieces_score.get(&(index as i32 % 6)).unwrap());
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
            pieces_score: HashMap::new() 
        };

        evaluation.pieces_score.insert(0, 1);
        evaluation.pieces_score.insert(1, 3);
        evaluation.pieces_score.insert(2, 3);
        evaluation.pieces_score.insert(3, 5);
        evaluation.pieces_score.insert(4, 9);
        evaluation.pieces_score.insert(5, 100);

        evaluation
    })
}