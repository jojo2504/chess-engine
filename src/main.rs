use std::env;

use chess_engine::engine::models::board::Board;

fn main() {
    // let args: Vec<String> = env::args().collect();

    println!("{:?}", Board::get_all_border_clear())
}
