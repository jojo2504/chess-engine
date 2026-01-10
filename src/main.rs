#![warn(missing_docs, dead_code)]
#![warn(unused_imports, unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use lib::{engine::{Engine, engine::EngineBuilder, models::board::Chessboard}, perft_tree};
use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};
use std::{alloc::System, env, io::{self, BufRead}};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() -> anyhow::Result<()> {
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
        let engine_builder: Result<Engine, String> = EngineBuilder::new().default_fen().search(5).build();
        if let Ok(mut engine) = engine_builder {
            engine.start_self_game();
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