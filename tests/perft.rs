#[cfg(test)]
mod tests {
    use chess_engine::engine::models::board::Chessboard;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_perft_1() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let chessboard = Chessboard::with_fen(fen);
        assert_eq!(chessboard.perft(1), 20);
        assert_eq!(chessboard.perft(2), 400);
        assert_eq!(chessboard.perft(3), 8902);
        assert_eq!(chessboard.perft(4), 197281);
        assert_eq!(chessboard.perft(5), 4865609);
    }

    #[test]
    fn test_perft_2() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 48);
        assert_eq!(chessboard.perft(2), 2039);
        assert_eq!(chessboard.perft(3), 97862);
    }

    #[test]
    fn test_perft_3() {
        let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 14);
        assert_eq!(chessboard.perft(2), 191);
        assert_eq!(chessboard.perft(3), 2812);
        assert_eq!(chessboard.perft(4), 43238);
        assert_eq!(chessboard.perft(5), 674624);
    }

    #[test]
    fn test_perft_4() {
        let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 6);
        assert_eq!(chessboard.perft(2), 264);
        assert_eq!(chessboard.perft(3), 9467);
    }

    #[test]
    fn test_perft_5() {
        let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 44);
        assert_eq!(chessboard.perft(2), 1486);
        assert_eq!(chessboard.perft(3), 62379);
    }

    #[test]
    fn test_perft_6() {
        let fen = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 46);
        assert_eq!(chessboard.perft(2), 2079);
        assert_eq!(chessboard.perft(3), 89890);
    }

    #[test]
    fn test_perft_custom_1() {
        let fen = "k7/8/8/8/p7/8/7P/7K w - - 1 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 4);
        assert_eq!(chessboard.perft(2), 16);
    }

    #[test]
    fn test_perft_custom_2() {
        let fen = "k7/8/8/8/p6P/8/8/7K b - H3 1 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(2), 16);
    }

    #[test]
    fn test_perft_custom_3() {
        let fen = "k7/8/8/8/p7/8/7P/7K b - - 1 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 4);
        assert_eq!(chessboard.perft(2), 16);
    }

    #[test]
    fn test_perft_custom_4() {
        let fen = "r3k2r/p6p/P6P/8/8/p6p/P6P/R3K2R w KQkq - 0 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 12);
    }

    #[test]
    fn test_perft_2_custom_1() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 43);
    }

    #[test]
    fn test_perft_3_custom_1() {
        let fen = "8/2p5/3p4/KP5r/1R3pPk/8/4P3/8 b - g3 0 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 17);
    }

    #[test]
    fn test_perft_can_black_king_castle() {
        let fen = "r3k2r/p6p/P6P/8/8/8/8/4K3 b kq - 0 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 12);
    }

    #[test]
    fn test_perft_checkmate() {
        let fen = "5k2/7R/5K1P/5P2/8/8/8/8 w - - 0 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(2), 20);
    }

    #[test]
    fn test_perft_4_custom_1() {
        let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P1RPP/R2Q2K1 b kq - 0 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 45);
        assert_eq!(chessboard.perft(2), 1623);
    }

    #[test]
    fn test_perft_5_custom_1() {
        let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/P7/1PP1NnPP/RNBQK2R b KQ - 1 8";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 34);
        assert_eq!(chessboard.perft(2), 1373);
    }

    #[test]
    fn test_promotion() {
        let fen = "K6k/8/6Q1/8/8/8/3p4/8 b - - 0 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(1), 4);
    }

    #[test]
    fn test_custom_wtf_is_going_on() {
        let fen = "7k/8/8/8/1p6/7p/P5P1/R3K3 w Q - 0 1";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(4), 10087);
    }

    #[test]
    fn test_capture_perft_2() {
        let fen = "8/1p3p1p/5PkP/5pPp/P4PpP/5pKp/5P1P/8 b - - 0 2";
        let chessboard = Chessboard::with_fen(fen);

        assert_eq!(chessboard.perft(2), 3);
    }
}