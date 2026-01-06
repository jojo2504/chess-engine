// #![warn(missing_docs, dead_code)]
// #![warn(unused_imports, unused_mut)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![deny(clippy::unwrap_used, clippy::expect_used)]

use lib::{as_064b, draw_perft_tree, engine::{Engine, models::{board::{Chessboard, Color}, r#move::Move, piece::Pawn}}, perft, perft_to_file, perft_tree, search_test, utils::string_format::display_bitstring_as_chessboard};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::{alloc::System, env};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() -> anyhow::Result<()> {
    // let mut chessboard= Chessboard::from_fen("2bn1k2/3P4/8/8/8/8/8/7K w - - 0 1").unwrap();
    let mut chessboard = Chessboard::new();

    let args: Vec<String> = env::args().collect();
    if args.len() >= 3 {
        let depth: u8 = args[1]
            .parse()
            .expect("Depth must be a number");

        let fen = &args[2];

        let mut chessboard = Chessboard::from_fen(fen).unwrap();
        perft_tree(&mut chessboard, depth);
    }
    else {
        println!("{}", perft(&mut chessboard, 6));
        // let mut engine = Engine::new();
        // engine.start_self_game();
    }
    Ok(())
}
 