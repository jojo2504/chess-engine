use crate::engine::{models::{board::Chessboard, r#move::{Move, MoveKind}}, movegen::generate_moves, search::Search};

pub mod engine;
pub mod utils;

/// Performs a `perft` performance and debugging test returning the total number of positions at the end
pub fn perft(chessboard: &mut Chessboard, depth: u8) -> u64 {
    if depth == 0 {
        return 1u64;
    }

    let mut nodes = 0;
    let all_pseudo_legal_moves = generate_moves(chessboard);
    let n_moves = all_pseudo_legal_moves.len();
    for _move in all_pseudo_legal_moves.iter().take(n_moves) {
        chessboard.make(_move);
        if !chessboard.is_in_check(chessboard.state_stack[chessboard.ply_index].turn_color) {
            nodes += perft(chessboard, depth - 1);
        }
        println!("chessboard: \n{}", chessboard);
        chessboard.unmake(_move);
    }

    nodes
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
            MoveKind::try_from(mv.move_kind_code()).unwrap()
        );

        chessboard.make(mv);

        let is_in_check =
            chessboard.is_in_check(chessboard.state_stack[chessboard.ply_index].turn_color);

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

        let in_check = chessboard.is_in_check(chessboard.state_stack[chessboard.ply_index].turn_color);
        if !in_check {
            let move_nodes = perft_tree(chessboard, depth - 1);
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

pub fn search_test() {
    let mut chessboard = Chessboard::new();
    let mut search = Search::new(3);
    let best_move = search.think(&mut chessboard).unwrap();
    println!("asd");
    println!("{}", best_move);
}

#[cfg(test)]
mod tests {
    use crate::engine::{models::{board::{Chessboard, Color, Square, get_piece_index}, piece::Piece}, search::{Search, evaluation::Evaluation}};

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

    #[test]
    fn evaluation() {
        let chessboard = Chessboard::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/5Q1p/PP1B2PP/R3K2R w kq - 0 1").unwrap();
        let evaluation = Evaluation::evaluate(&chessboard);
        println!("{}", evaluation);
    }
}