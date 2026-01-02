#![warn(missing_docs, dead_code)]
#![warn(unused_imports, unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use chess_engine::{engine::{Engine, models::board::Chessboard}, perft};
use utils;

fn main() -> anyhow::Result<()> {
    // let engine = Engine::new();
    // let engine = engine.validate_uci_connection()?;
    // engine.start_uci_game()?;
    let mut chessboard = Chessboard::new();
    println!("{}", perft(&mut chessboard, 3));
    Ok(())
}
 