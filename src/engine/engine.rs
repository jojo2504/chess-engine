use crate::engine::models::{board::Chessboard, r#move};

/// This is the entry of our chess engine, which will be used to start the game using a chessboard
/// 
/// The engine will support:
/// - UCI game against another player (i.e. via Litchess bridge)
/// - UCI game against another engine
/// - UCI game against itself
/// - game against itself
/// 
/// # Exemples
/// ```rust
/// use chess_engine::engine::engine::Engine;
/// 
/// let engine = Engine::new();
/// engine.start_uci_game(); // connecting and playing against another player using the litchess bot bridge
/// ```
pub struct Engine {
    chessboard: Chessboard
}

impl Engine {
    /// Initializing the engine's chessboard with the classic starting chess position.
    pub fn new() -> Self {
        Self {
            chessboard: Chessboard::new()
        }
    }

    /// Initializing the engine's chessboard with a custom position, parsed using fen.
    pub fn from_fen(fen: &str) -> Self {
        Self {
            chessboard: Chessboard::from_fen(fen)
        }
    }

    /// This method starts an UCI game, the engine or AI will return after each of its turn its corresponding "best move" as UCI encoding.
    pub fn start_uci_game(&self) {
        todo!()
    }
    
    /// This method starts game against itself, the engine or AI will return after each of its turn its corresponding "best move".
    pub fn start_self_game(&self) {
        todo!()
    }

    pub fn start_self_uci_game(&self) {
        todo!()
    }
}