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
    // let mut chessboard= Chessboard::from_fen("8/8/8/8/8/6q1/4P3/7K w - - 0 1").unwrap();
    let mut chessboard = Chessboard::new();

    // let before = GLOBAL.stats();
    // draw_perft_tree(&mut chessboard, 2, "");
    // // println!("{}", perft(&mut chessboard, 5));
    // let after = GLOBAL.stats();
    // println!("{:#?}", after - before);
    // search_test();
    let mut args = env::args();
    if args.len() > 1 {
        println!("perftree: ");
        perft_tree(&mut chessboard, args.nth(1).unwrap().as_bytes()[0]);
    }
    else {
        println!("normal perft: ");
        // perft(&mut chessboard, 2);
        perft_to_file(&mut chessboard, 3, "./a.txt");
    }
    Ok(())
}
 