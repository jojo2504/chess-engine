use crate::{
    engine::models::{
        board::{Chessboard, Color},
        r#move::{Move, MoveKind},
        piece::{Bishop, King, Knight, Pawn, Piece, Queen, Rook},
    },
    utils::{bit_operations::pop_1st_bit, string_format::display_bitstring_as_chessboard},
};

/// Add pawn moves to the current move generation cycle.
#[inline(always)]
pub(crate) fn add_all_possible_moves_pawn(
    from: u64,
    mut possible_moves: u64,
    chessboard: &Chessboard,
    all_pseudo_legal_moves: &mut Vec<Move>,
) {
    let from_index = from.trailing_zeros();
    let word_from = (from_index << 10) as u16;

    while possible_moves != 0 {
        let to: u64 = 1 << possible_moves.trailing_zeros();
        let to_index = to.trailing_zeros();
        possible_moves ^= to;

        let mut word: u16 = word_from | (to_index << 4) as u16;
        
        let is_capture = ((1u64 << to_index) & chessboard.get_all_pieces()) != 0;
        
        let mut captured_piece: Option<Piece> = None;
        if is_capture {
            for (i, piece) in chessboard.pieces.iter().enumerate() {
                if to & piece != 0 {
                    captured_piece = Some(Piece::try_from((i % 6) as i32).unwrap())
                }
            }
        }

        // Promotion moves (ranks 0-7 or 56-63)
        if !(8..56).contains(&to_index) {
            let promotions = if is_capture {
                &[
                    MoveKind::KnightPromotionCapture,
                    MoveKind::BishopPromotionCapture,
                    MoveKind::RookPromotionCapture,
                    MoveKind::QueenPromotionCapture,
                ]
            } else {
                &[
                    MoveKind::KnightPromotion,
                    MoveKind::BishopPromotion,
                    MoveKind::RookPromotion,
                    MoveKind::QueenPromotion,
                ]
            };

            for &promotion in promotions {
                let promotion_word = word | (promotion as u16);
                let _move = Move::from(promotion_word, Piece::Pawn, captured_piece);
                all_pseudo_legal_moves.push(_move);
            }
        } else {
            // Regular pawn moves - determine special code
            let distance = (from_index as i32 - to_index as i32).abs();

            let special_code = if is_capture {
                MoveKind::Captures
            } else if distance == 16 {
                MoveKind::DoublePawnPush
            } else if distance == 7 || distance == 9 {
                MoveKind::EpCapture
            } else {
                MoveKind::QuietMoves
            };

            word |= special_code as u16;
            let _move = Move::from(word, Piece::Pawn, captured_piece);
            all_pseudo_legal_moves.push(_move);
        }
    }
}

/// Add king moves to the current move generation cycle.
#[inline(always)]
pub(crate) fn add_all_possible_moves_king(
    from: u64,
    mut possible_moves: u64,
    chessboard: &Chessboard,
    all_pseudo_legal_moves: &mut Vec<Move>,
) {
    let from_index = from.trailing_zeros();
    let word_from = from_index << 10;

    while possible_moves != 0 {
        let to: u64 = 1 << possible_moves.trailing_zeros();
        let to_index = to.trailing_zeros();
        possible_moves ^= to;

        let mut word: u16 = word_from as u16 | (to_index << 4) as u16;
        let mut captured_piece: Option<Piece> = None;

        if (from_index as i32 - to_index as i32).abs() == 2 {
            if to_index == 2 || to_index == 58 {
                word |= MoveKind::QueenCastle as u16;
            } else if to_index == 6 || to_index == 62 {
                word |= MoveKind::KingCastle as u16;
            }
        } else if to & chessboard.get_all_pieces() != 0 {
            word |= MoveKind::Captures as u16;
            for (i, piece) in chessboard.pieces.iter().enumerate() {
                if to & piece != 0 {
                    captured_piece = Some(Piece::try_from((i % 6) as i32).unwrap())
                }
            }
        }

        let _move = Move::from(word, Piece::King, captured_piece);
        all_pseudo_legal_moves.push(_move);
    }
}

/// Add all other pieces to the current move generation cycle.
#[inline(always)]
pub(crate) fn add_all_possible_moves(
    from: u64,
    mut possible_moves: u64,
    chessboard: &Chessboard,
    piece: Piece,
    all_pseudo_legal_moves: &mut Vec<Move>,
) {
    let word_from: u16 = (from.trailing_zeros() as u16) << 10;
    while possible_moves != 0 {
        let to: u64 = 1 << possible_moves.trailing_zeros();
        possible_moves ^= to;

        let mut word: u16 = word_from | ((to.trailing_zeros() as u16) << 4);

        let mut captured_piece: Option<Piece> = None;
        if to & chessboard.get_all_pieces() != 0 {
            word |= MoveKind::Captures as u16;
            for (i, piece) in chessboard.pieces.iter().enumerate() {
                if to & piece != 0 {
                    captured_piece = Some(Piece::try_from((i % 6) as i32).unwrap())
                }
            }
        }

        let _move = Move::from(word, piece, captured_piece);
        all_pseudo_legal_moves.push(_move);
    }
}

/// Generate all **SPEUDO LEGAL** moves for a given piece and color, updating the `all_pseudo_legal_moves` vector at the same time.
pub(crate) fn get_all_possible_piece_moves(
    chessboard: &Chessboard,
    side: Color,
    piece: Piece,
    all_pseudo_legal_moves: &mut Vec<Move>,
) {
    let mut pieces = chessboard.get_piece(side, piece);
    let mut _possible_moves = 0u64;

    match piece {
        Piece::Pawn => {
            while pieces != 0 {
                let from = 1 << pieces.trailing_zeros();
                pop_1st_bit(&mut pieces);

                _possible_moves = Pawn::compute_possible_moves(from, chessboard, side);
                // println!("PAWN POSSIBLE MOVES:"); 
                // display_bitstring_as_chessboard(&format!("{:064b}", _possible_moves));
                add_all_possible_moves_pawn(
                    from,
                    _possible_moves,
                    chessboard,
                    all_pseudo_legal_moves,
                );
            }
        }
        Piece::Rook => {
            while pieces != 0 {
                let from = 1 << pieces.trailing_zeros();
                pop_1st_bit(&mut pieces);

                _possible_moves = Rook::compute_possible_moves(from, chessboard, side);
                add_all_possible_moves(
                    from,
                    _possible_moves,
                    chessboard,
                    Piece::Rook,
                    all_pseudo_legal_moves,
                );
            }
        }
        Piece::Knight => {
            while pieces != 0 {
                let from = 1 << pieces.trailing_zeros();
                pop_1st_bit(&mut pieces);

                _possible_moves = Knight::compute_possible_moves(from, chessboard, side);
                // println!("KNIGHT POSSIBLE MOVES:"); 
                // display_bitstring_as_chessboard(&format!("{:064b}", _possible_moves));
                add_all_possible_moves(
                    from,
                    _possible_moves,
                    chessboard,
                    Piece::Knight,
                    all_pseudo_legal_moves,
                );
            }
        }
        Piece::Bishop => {
            while pieces != 0 {
                let from = 1 << pieces.trailing_zeros();
                pop_1st_bit(&mut pieces);

                _possible_moves = Bishop::compute_possible_moves(from, chessboard, side);
                // println!("BISHOP POSSIBLE MOVES:"); 
                // display_bitstring_as_chessboard(&format!("{:064b}", _possible_moves));
                add_all_possible_moves(
                    from,
                    _possible_moves,
                    chessboard,
                    Piece::Bishop,
                    all_pseudo_legal_moves,
                );
            }
        }
        Piece::Queen => {
            while pieces != 0 {
                let from = 1 << pieces.trailing_zeros();
                pop_1st_bit(&mut pieces);

                _possible_moves = Queen::compute_possible_moves(from, chessboard, side);
                // println!("QUEEN POSSIBLE MOVES:"); 
                // display_bitstring_as_chessboard(&format!("{:064b}", _possible_moves));
                add_all_possible_moves(
                    from,
                    _possible_moves,
                    chessboard,
                    Piece::Queen,
                    all_pseudo_legal_moves,
                );
            }
        }
        Piece::King => {
            _possible_moves = King::compute_possible_moves(pieces, chessboard, side);

            if chessboard.should_check_castling() {
                _possible_moves |= King::compute_possible_castling_moves(pieces, chessboard, side);
            }

            add_all_possible_moves_king(
                pieces,
                _possible_moves,
                chessboard,
                all_pseudo_legal_moves,
            );
        }
    }
}

/// Generate all **PSEUDO LEGAL** moves and return them into a vector.
pub fn generate_moves(chessboard: &Chessboard) -> Vec<Move> {
    let mut all_pseudo_legal_moves = Vec::with_capacity(256);

    for i in 0..6 {
        get_all_possible_piece_moves(
            chessboard,
            chessboard.state.turn_color,
            Piece::try_from(i).unwrap(),
            &mut all_pseudo_legal_moves,
        );
    }
    all_pseudo_legal_moves
}

pub fn generate_legal_moves(chessboard: &mut Chessboard) -> Vec<Move> {
    let mut all_pseudo_legal_moves = Vec::with_capacity(256);

    for i in 0..6 {
        get_all_possible_piece_moves(
            chessboard,
            chessboard.state.turn_color,
            Piece::try_from(i).unwrap(),
            &mut all_pseudo_legal_moves,
        );
    }

    // test and filter
    all_pseudo_legal_moves.into_iter().filter(|mv| {
        chessboard.make(&mv);
        let is_legal = !chessboard.is_in_check();
        chessboard.unmake(mv);
        is_legal
    }).collect()
}