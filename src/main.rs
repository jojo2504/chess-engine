// #![warn(missing_docs, dead_code)]
// #![warn(unused_imports, unused_mut)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![deny(clippy::unwrap_used, clippy::expect_used)]

use lib::{draw_perft_tree, engine::models::{board::Chessboard, r#move::Move}, perft, perft_to_file, perft_tree, search_test};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::{alloc::System, env};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() -> anyhow::Result<()> {
    let mut chessboard= Chessboard::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/P7/1PP1NnPP/RNBQK2R b KQ - 1 8").unwrap();
    // let mut chessboard = Chessboard::new();

    // let before = GLOBAL.stats();
    // draw_perft_tree(&mut chessboard, 2, "");
    // // println!("{}", perft(&mut chessboard, 5));
    // let after = GLOBAL.stats();
    // println!("{:#?}", after - before);
    // search_test();
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
        // let uci = "b2b3";
        // println!("{}", Move::decode_uci(uci, &chessboard).unwrap());
        // println!("normal perft: ");
        // println!("{}", perft(&mut chessboard, 1));
        perft_to_file(&mut chessboard, 1, "./a.txt");
        // draw_perft_tree(&mut chessboard, 3, " ");
    }
    Ok(())
}
 