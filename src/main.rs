#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use chess_engine::engine::engine::Engine;

fn main() -> anyhow::Result<()> {
    let engine = Engine::new();
    let engine = engine.validate_uci_connection()?;
    engine.start_uci_game();
    Ok(())
}
 