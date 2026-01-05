// #![warn(missing_docs, dead_code)]
// #![warn(unused_imports, unused_mut)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![deny(clippy::unwrap_used, clippy::expect_used)]

use lib::{draw_perft_tree, engine::models::board::Chessboard, perft, perft_to_file, perft_tree, search_test};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::{alloc::System, env};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() -> anyhow::Result<()> {
    // let mut chessboard= Chessboard::from_fen("k7/8/1Q6/8/8/1Pp3q1/2P5/7K w - - 0 1").unwrap();
    let mut chessboard = Chessboard::new();

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
        println!("normal perft: ");
        println!("{}", perft(&mut chessboard, 2));
        // perft_to_file(&mut chessboard, 3, "./a.txt");
        // draw_perft_tree(&mut chessboard, 3, " ");
    }
    Ok(())
}
 