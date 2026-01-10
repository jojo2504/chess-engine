use std::{fs::File, io::BufWriter};
use std::io::Write;

use crate::engine::{models::{board::Chessboard, r#move::MoveKind}, movegen::generate_moves};

pub mod engine;
pub mod utils;

pub fn pause(banner: &str) {
    println!("{}", banner);
    console::Term::stdout().read_char();
}

pub fn as_064b(value: u64) -> String {
    format!("{:064b}", value)
}
/// Performs a `perft` performance and debugging test returning the total number of positions at the end
pub fn perft(chessboard: &mut Chessboard, depth: u8) -> u64 {
    if depth == 0 {
        return 1u64;
    }

    let mut nodes = 0;
    let all_pseudo_legal_moves = generate_moves(chessboard);
    let n_moves = all_pseudo_legal_moves.len();
    for _move in all_pseudo_legal_moves.iter().take(n_moves) {
        // println!("Moves = {:?} ({})", all_pseudo_legal_moves, n_moves);
        // pause(format!("Before make {} (# {}/{}) as {:?}", _move, i, n_moves, chessboard.get_current_turn()).as_str());
        chessboard.make(_move);
        // println!("{}", chessboard);

        if !chessboard.is_in_check() {
            nodes += perft(chessboard, depth - 1);
        }
        // println!("Moves = {:?} ({})", all_pseudo_legal_moves, n_moves);
        // pause(format!("Before [un]make {} as {:?}", _move, chessboard.get_current_turn()).as_str());
        chessboard.unmake(_move);
        // println!("{}", chessboard);

        // i += 1;
    }

    nodes
}

pub fn perft_to_file(chessboard: &mut Chessboard, depth: u8, file_path: &str) -> u64 {
    let file = File::create(file_path).expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    fn perft_inner(chessboard: &mut Chessboard, depth: u8, writer: &mut BufWriter<File>) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;
        let all_pseudo_legal_moves = crate::engine::movegen::generate_moves(chessboard);
        let n_moves = all_pseudo_legal_moves.len();

        for mv in all_pseudo_legal_moves.iter().take(n_moves) {
            chessboard.make(mv);
            if !chessboard.is_in_check() {
                let move_nodes = perft_inner(chessboard, depth - 1, writer);
                nodes += move_nodes;
            }
        }

        nodes
    }

    let total_nodes = perft_inner(chessboard, depth, &mut writer);
    writeln!(writer, "\nTotal nodes: {}", total_nodes).expect("Failed to write total");
    total_nodes
}


pub fn draw_perft_tree(
    chessboard: &mut Chessboard,
    depth: u8,
    indent: &str,
) -> u64 {
    if depth == 0 {
        println!("{indent}└─ leaf: 1");
        return 1;
    }

    let mut total_nodes = 0;

    let all_pseudo_legal_moves = generate_moves(chessboard);
    let n_moves = all_pseudo_legal_moves.len();

    for (i, mv) in all_pseudo_legal_moves.iter().enumerate() {
        let is_last_move = i + 1 == n_moves;
        let branch = if is_last_move { "└─" } else { "├─" };
        let new_indent = if is_last_move {
            format!("{indent}   ")
        } else {
            format!("{indent}│  ")
        };

        println!(
            "{indent}{branch} {:?} {} {:?}",
            chessboard.state.turn_color,
            mv,
            MoveKind::from_u8_unchecked(mv.move_kind_code())
        );

        chessboard.make(mv);

        let is_in_check = chessboard.is_in_check();
        if !is_in_check {
            let subtree_nodes = draw_perft_tree(chessboard, depth - 1, &new_indent);
            total_nodes += subtree_nodes;
        } else {
            println!("{new_indent}└─ illegal (in check)");
        }

        chessboard.unmake(mv);
    }

    println!("{indent}└─ nodes: {total_nodes}");
    total_nodes
}

/// Recursive perft tree that prints each move's subtree and total nodes
pub fn perft_tree(chessboard: &mut Chessboard, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    
    let all_pseudo_legal_moves = generate_moves(chessboard);
    let n_moves = all_pseudo_legal_moves.len();

    for mv in all_pseudo_legal_moves.iter().take(n_moves) {
        chessboard.make(mv);

        let in_check = chessboard.is_in_check();
        if !in_check {
            let move_nodes = perft(chessboard, depth - 1);
            nodes += move_nodes;
            chessboard.unmake(mv);
            println!("{} {}", mv, move_nodes);
        } else {
            chessboard.unmake(mv);
            println!("{} 0", mv);
        }
    }

    println!();
    println!("{}", nodes);

    nodes
}