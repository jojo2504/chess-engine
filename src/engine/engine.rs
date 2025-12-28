use crate::engine::models::{board::Chessboard, r#move};

pub struct Engine {
    chessboard: Chessboard
}

impl Engine {
    pub fn new() -> Self {
        Self {
            chessboard: Chessboard::new()
        }
    }

    pub fn with_fen(fen: &str) -> Self {
        Self {
            chessboard: Chessboard::with_fen(fen)
        }
    }

    pub fn start_uci_game() {
        todo!()
    }

    pub fn start_self_game() {
        todo!()
    }

    pub fn start_self_uci_game() {
        todo!()
    }
}