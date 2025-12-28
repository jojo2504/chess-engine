use crate::engine::models::{board::{Chessboard, Color}, piece::Piece};

enum MoveKind {
    QuietMoves = 0,
    DoublePawnPush = 1,
    KingCastle = 2,
    QueenCastle = 3,
    Captures = 4,
    EpCapture = 5,
    KnightPromotion = 8,
    BishopPromotion = 9,
    RookPromotion = 10,
    QueenPromotion = 11,
    KnightPromotionCapture = 12,
    BishopPromotionCapture = 13,
    RookPromotionCapture = 14,
    QueenPromotionCapture = 15
}

pub struct Move {
    
}

impl Move {
    /// Returns all the playable moves encoded as uci.
    pub fn get_ucis() -> Vec<String> {
        todo!()
    }

    /// Decodes incoming uci encoded move into a `Move` object.
    pub fn decode_uci() -> Move {
        todo!()
    }

    /// Use this method when required to put a piece by moving one, like passive moves, captures, etc...
    /// 
    /// You should also combine it with [Move::toggle_piece()] when capturing pieces.
    /// 
    /// # Exemple 
    /// ```rust
    /// use chess_engine::engine::models::r#move::Move;
    /// 
    /// // Move a bishop to e4
    /// Move::slide_piece();
    /// // Remove the captured piece
    /// Move::toggle_piece();
    /// ```
    pub fn slide_piece(chessboard: &mut Chessboard, board: &mut u64, from: u64, to: u64, side: Color) {
        todo!()
    }

    /// Use this method when required to put a piece without moving one or removing a piece, like during game initialization, captures or promotions.
    pub fn toggle_piece(chessboard: &mut Chessboard, board: &mut u64, square: u64, piece: Piece, side: Color) {
        todo!()
    }

    /// Used by UCI to decode a move from a string when played from the GUI.
    pub fn get_move_kind() {
        todo!()
    }

    /// Make a move on the chessboard itself.
    pub fn make(chessboard: &mut Chessboard, r#move: Move) {
        todo!()
    }
    
    /// Unmake a move on the chessboard itself.
    pub fn unmake(chessboard: &mut Chessboard, r#move: Move) {
        todo!()
    }
}