use crate::engine::models::board::Chessboard;

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
    pub fn get_ucis() -> Vec<String> {
        todo!()
    }

    pub fn decode_uci() -> Move {
        todo!()
    }

    pub fn slide_piece() {
        todo!()
    }

    pub fn put_piece() {
        todo!()
    }

    pub fn get_move_code() {
        todo!()
    }

    pub fn make(chessboard: &mut Chessboard, r#move: Move) {
        todo!()
    }

    pub fn unmake(chessboard: &mut Chessboard, r#move: Move) {
        todo!()
    }
}