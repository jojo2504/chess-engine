// #![warn(missing_docs, dead_code)]
// #![warn(unused_imports, unused_mut)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![deny(clippy::unwrap_used, clippy::expect_used)]

use lib::{as_064b, draw_perft_tree, engine::{self, Engine, engine::EngineBuilder, models::{board::{Chessboard, Color}, r#move::Move, piece::Pawn}}, perft, perft_to_file, perft_tree, utils::string_format::display_bitstring_as_chessboard};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::{alloc::System, env, io::{self, BufRead}};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() -> anyhow::Result<()> {
    let mut chessboard= Chessboard::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap();

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
        // println!("{}", draw_perft_tree(&mut chessboard, 2, " "));
        println!("{}", perft_to_file(&mut chessboard, 2, "./a"));
        panic!();
        let engine_builder: Result<Engine, String> = EngineBuilder::new().default_fen().search(5).build();
        if let Ok(mut engine) = engine_builder {
            engine.start_self_game();
            panic!();
            let stdin = io::stdin();
            let mut input = stdin.lock().lines();
            if let Ok(mut engine) = engine.validate_uci_connection(&mut input) {
                engine.start_uci_game(&mut input)?;
            }
        }
        else {
            eprintln!("{}", engine_builder.err().unwrap())
        }
    }
    Ok(())
}