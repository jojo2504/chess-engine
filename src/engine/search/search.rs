use std::{cmp::max, collections::HashMap, sync::{Arc, Mutex}};

use crate::engine::{models::{board::{Chessboard, Color}, r#move::Move}, movegen::generate_moves, search::evaluation::Evaluation};
use rayon::prelude::*;

#[derive(Default, Clone)]
enum NodeType {
    Exact,
    Lowerbound,
    Upperbound,
    #[default]
    None
}

#[derive(Default, Clone)]
pub struct TTEntry {
    flag: NodeType,
    depth: i32,
    value: i32
}

impl TTEntry {
    pub fn new() -> Self {
        Self {..Default::default()}
    }
}

#[derive(Default, Clone)]
pub struct Search {
    pub depth: i32,
    pub tt: HashMap<u64, TTEntry> // zobrist_key, TTEntry
}

impl Search {
    pub fn new(depth: i32) -> Self {
        println!("search created");
        Self { 
            depth,
            ..Default::default()
        }
    }

    fn negamax(&mut self, chessboard: &mut Chessboard, depth: i32, mut alpha: i32, beta: i32, color: i32) -> i32 {
        let alpha_orig = alpha;
        
        if let Some(tt_entry) = self.tt.get(&chessboard.state.zobrist_key) && tt_entry.depth >= depth {
            match tt_entry.flag {
                NodeType::Exact => return tt_entry.value,
                NodeType::Lowerbound if tt_entry.value >= beta => return tt_entry.value,
                NodeType::Upperbound if tt_entry.value <= alpha => return tt_entry.value,
                _ => ()
            }
        }

        if depth == 0 || chessboard.state.checkmated {
            return color * Evaluation::evaluate(chessboard);
        }

        let child_nodes = generate_moves(chessboard);
        
        let mut best_score = i32::MIN;
        for child in &child_nodes {
            chessboard.make(child);
            best_score = max(best_score, self.negamax(chessboard, depth - 1, beta.saturating_neg(), alpha.saturating_neg(), -color).saturating_neg());
            chessboard.unmake(child);

            alpha = max(alpha, best_score);
            if alpha >= beta {
                break;
            }
        }

        let mut tt_entry = TTEntry::new();
        if best_score <= alpha_orig {
            tt_entry.flag = NodeType::Upperbound;
        }
        else if best_score >= beta {
            tt_entry.flag = NodeType::Lowerbound;
        }
        else {
            tt_entry.flag = NodeType::Exact;
        }

        tt_entry.depth = depth;
        tt_entry.value = best_score;
        self.tt.insert(chessboard.state.zobrist_key, tt_entry);

        best_score
    }

    pub(crate) fn think(&mut self, chessboard: &mut Chessboard) -> Option<Move> {
        let all_moves = generate_moves(chessboard);
        println!("generated all moves");
        
        let best_move= Arc::new(Mutex::new(None));
        let best_score = Arc::new(Mutex::new(i32::MIN));
        let all_moves = all_moves.as_slice();
        all_moves.par_iter().for_each(|_move| {
            let mut game_copy = chessboard.clone();
            let mut search_copy = self.clone();

            game_copy.make(_move);
            let color = match game_copy.state.turn_color {
                Color::White => 1,
                Color::Black => -1,
            };
            let move_score = search_copy.negamax(&mut game_copy, search_copy.depth, i32::MIN, i32::MAX, color).saturating_neg();
            game_copy.unmake(_move);
            
            let mut score_guard = best_score.lock().unwrap();
            if move_score > *score_guard {
                *score_guard = move_score;
                *best_move.lock().unwrap() = Some(_move);
            }
        });

        best_move.lock().unwrap().cloned()
    }
}