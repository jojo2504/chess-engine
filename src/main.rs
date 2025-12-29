use std::env;

use chess_engine::engine::{magic::magic::Magic, models::board::Board};

fn main() -> anyhow::Result<()> {
    // let args: Vec<String> = env::args().collect();

    let a = Magic::load_magic_table("/home/jojo/Documents/rust/chess-engine/src/engine/magic/BMagicTable.json")?;
    println!("{:?}", a);
    Ok(())
}
