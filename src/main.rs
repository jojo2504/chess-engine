#![warn(missing_docs, dead_code)]
#![warn(unused_imports, unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use lib::{engine::models::board::Chessboard, perft, search_test};

fn main() -> anyhow::Result<()> {
    let mut chessboard= Chessboard::new();
    println!("{}", perft(&mut chessboard, 2));
    // search_test();
    Ok(())
}
 