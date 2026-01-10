use std::sync::OnceLock;
use rand::Rng;

use crate::engine::models::{board::Color, piece::Piece, state::State};

struct Zobrist {
    /// Zobrist array
    /// 1 number for each piece at each square                                                   (2 * 6 * 64)
    /// 4 numbers to indicate the castling rights, though usually 16 (2^4) are used for speed    (16)
    /// 8 numbers to indicate the file of a valid En passant square, if any                      (8)
    /// 1 number to indicate the side to move is black                                           (1)
    /// This leaves us with an array with 793 (12*64 + 1 + 16 + 8) random numbers.
    token_square: [u64; 768],
    /// Index | WK | WQ | BK | BQ | Binary
    /// ------|----|----|----|----|-------
    /// 0     | F  | F  | F  | F  | 0000
    /// 1     | T  | F  | F  | F  | 0001
    /// 2     | F  | T  | F  | F  | 0010
    /// 3     | T  | T  | F  | F  | 0011
    /// 4     | F  | F  | T  | F  | 0100
    /// 5     | T  | F  | T  | F  | 0101
    /// 6     | F  | T  | T  | F  | 0110
    /// 7     | T  | T  | T  | F  | 0111
    /// 8     | F  | F  | F  | T  | 1000
    /// 9     | T  | F  | F  | T  | 1001
    /// 10    | F  | T  | F  | T  | 1010
    /// 11    | T  | T  | F  | T  | 1011
    /// 12    | F  | F  | T  | T  | 1100
    /// 13    | T  | F  | T  | T  | 1101
    /// 14    | F  | T  | T  | T  | 1110
    /// 15    | T  | T  | T  | T  | 1111
    castling_rights: [u64; 16],
    en_passant_file: [u64; 8],
    black_to_move: u64
}

impl Zobrist {
    pub fn compute_castling_rights_hash(state: State) -> u64 {
        let mut castling_rights_index = 0;
        
        if state.can_white_king_castle() {
            castling_rights_index |= 1;   // 0001
        }
        if state.can_white_queen_castle() {
            castling_rights_index |= 2;  // 0010
        }
        if state.can_black_king_castle() {
            castling_rights_index |= 4;   // 0100
        }
        if state.can_black_queen_castle() {
            castling_rights_index |= 8;  // 1000
        }
        
        zobrist().castling_rights[castling_rights_index]
    }

    pub fn get_piece_square_index(color: Color, piece: Piece, square: u8) -> usize {
        (color as usize * 6 + piece as usize) * 64 + square as usize
    }
}

/// Lazy static initializer for [Zobrist].
fn zobrist() -> &'static Zobrist {
    static ZOBRIST: OnceLock<Zobrist> = OnceLock::new();
    ZOBRIST.get_or_init(|| {                
        let mut token_square = [0u64; 768];
        for i in 0..768 {
            token_square[i] = rand::random();
        }

        let mut castling_rights = [0u64; 16];
        for i in 0..16 {
            castling_rights[i] = rand::random();
        }

        let mut en_passant_file = [0u64; 8];
        for i in 0..8 {
            en_passant_file[i] = rand::random();
        }

        let black_to_move = rand::random();

        let zobrist = Zobrist {
            token_square,
            castling_rights,
            en_passant_file,
            black_to_move,
        };

        zobrist
    })
}