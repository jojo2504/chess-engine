// #![warn(missing_docs, dead_code)]
// #![warn(unused_imports, unused_mut)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![deny(clippy::unwrap_used, clippy::expect_used)]

use lib::{as_064b, draw_perft_tree, engine::models::{board::{Chessboard, Color}, r#move::Move, piece::Pawn}, perft, perft_to_file, perft_tree, search_test, utils::string_format::display_bitstring_as_chessboard};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::{alloc::System, env};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() -> anyhow::Result<()> {
    // let mut chessboard= Chessboard::from_fen("2bn1k2/3P4/8/8/8/8/8/7K w - - 0 1").unwrap();
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
        // let a = Color::White;
        // a.swap();
        // println!("{:?}", a);

        // for a in Pawn::get_attack_mask() {
        //     display_bitstring_as_chessboard(&format!("{:064b}", a));
        //     println!("------------------")
        // }
        // let uci = "b2b3";
        // println!("{}", Move::decode_uci(uci, &chessboard).unwrap());
        // println!("normal perft: ");
        println!("{}", perft(&mut chessboard, 5));
        // println!("white pieces");
        // display_bitstring_as_chessboard(&as_064b(chessboard.white_pieces));
        // println!("------------");
        // println!("black pieces");
        // display_bitstring_as_chessboard(&as_064b(chessboard.black_pieces));

        // println!("initial state {:?}", chessboard.state);

        // perft_to_file(&mut chessboard, 6, "./a.txt");
        // draw_perft_tree(&mut chessboard, 3, " ");
    }
    Ok(())
}
 