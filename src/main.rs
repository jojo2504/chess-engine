#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]

use chess_engine::engine::magic::magic::Magic;

fn main() -> anyhow::Result<()> {
    let a = Magic::load_magic_table("/home/jojo/Documents/rust/chess-engine/src/engine/magic/BMagicTable.json")?;
    println!("{:?}", a);
    Ok(())
}
