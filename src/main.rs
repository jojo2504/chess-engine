#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use chess_engine::engine::models::{board::{Chessboard}};

fn main() -> anyhow::Result<()> {
    let chessboard = Chessboard::new();
    println!("{}", chessboard);
    Ok(())
}
