use std::sync::OnceLock;

use crate::engine::models::board::{Chessboard, Color, File, Rank};

/// Quick enum to match pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Pawn {
    pawn_attack_masks: [u64; 128]
}

impl Pawn {
    pub fn get_attack_mask() -> [u64; 128] {
        pawn().pawn_attack_masks
    }

    pub fn compute_possible_moves(location: u64, chessboard: &Chessboard, turn_color: Color) -> u64 {
        match turn_color {
            Color::White => {
                let pawn_one_step: u64 = (location << 8) & !chessboard.get_all_pieces();
                let pawn_two_steps: u64 = ((pawn_one_step & Rank::Rank3.mask()) << 8) & !chessboard.get_all_pieces();
                // the union of the movements dictate the possible moves forward available
                let pawn_valid_moves: u64 = pawn_one_step | pawn_two_steps;
        
                /*  now we calculate the attack moves
                    check the left side of the pawn, minding the underflow File A */
                let pawn_left_attack: u64 = (location & File::FileA.clear()) << 7;
                // then check the right side of the pawn, minding the overflow File H
                let pawn_right_attack: u64 = (location & File::FileH.clear()) << 9;
                // Calculate where I can actually attack something + en passant
                //Logger.Log(BitOperations.ToSquare(pawn_left_attack), BitOperations.ToSquare(pawn_right_attack));
        
                let pawn_valid_attacks = (pawn_left_attack | pawn_right_attack) & 
                    (chessboard.get_all_pieces() | 
                    chessboard.state.en_passant_square.map_or(0, |sq| sq as u64));
                return pawn_valid_moves | pawn_valid_attacks;
            },
            Color::Black => {
                let pawn_one_step: u64 = (location >> 8) & !chessboard.get_all_pieces();
                let pawn_two_steps: u64 = ((pawn_one_step & Rank::Rank6.mask()) >> 8) & !chessboard.get_all_pieces();
                let pawn_valid_moves: u64 = pawn_one_step | pawn_two_steps;
        
                let pawn_left_attack: u64 = (location & File::FileA.clear()) >> 9;
                let pawn_right_attack: u64 = (location & File::FileH.clear()) >> 7;
        
                let pawn_valid_attacks = (pawn_left_attack | pawn_right_attack) & 
                    (chessboard.get_all_pieces() | 
                    chessboard.state.en_passant_square.map_or(0, |sq| sq as u64));
                return pawn_valid_moves | pawn_valid_attacks;
            }
        }
    }

    pub fn compute_possible_attacks(location: u64, chessboard: &Chessboard, turn_color: Color) -> u64 {
        let own_side = chessboard.get_color_pieces(turn_color);
        pawn().pawn_attack_masks[(turn_color as usize + 1) * location.trailing_zeros() as usize] & !own_side
    }
}

fn pawn() -> &'static Pawn {
    static PAWN: OnceLock<Pawn> = OnceLock::new();
    PAWN.get_or_init(|| {
        let mut pawn = Pawn { pawn_attack_masks: [0; 128] };
        
        let mut pawn_left_attack: u64;
        let mut pawn_right_attack: u64;

        for i in 0..64 {
            let location = 1u64 << i;
            pawn_left_attack = (location & File::FileA.clear()) << 7;
            pawn_right_attack = (location & File::FileH.clear()) << 9;
            pawn.pawn_attack_masks[i] = pawn_left_attack | pawn_right_attack;

            pawn_left_attack = (location & File::FileA.clear()) >> 9;
            pawn_right_attack = (location & File::FileH.clear()) >> 7;
            pawn.pawn_attack_masks[i + 64] = pawn_left_attack | pawn_right_attack;
        }

        pawn
    })
}

pub struct Knight {
    knight_move_masks: [u64; 64]
}

impl Knight {
    pub fn get_move_mask() -> [u64; 64] {
        knight().knight_move_masks
    }

    pub fn compute_possible_moves(location: u64, chessboard: &Chessboard, turn_color: Color) -> u64 {
        let own_side = chessboard.get_color_pieces(turn_color);
        knight().knight_move_masks[location.trailing_zeros() as usize] & !own_side
    }
}

fn knight() -> &'static Knight {
    static KNIGHT: OnceLock<Knight> = OnceLock::new();
    KNIGHT.get_or_init(|| {
        let mut knight = Knight { knight_move_masks: [0; 64] };
        
        let spot_1_clip = File::FileA.clear() & File::FileB.clear();
        let spot_2_clip = File::FileA.clear();
        let spot_3_clip = File::FileH.clear();
        let spot_4_clip = File::FileH.clear() & File::FileG.clear();

        let spot_5_clip = File::FileH.clear() & File::FileG.clear();
        let spot_6_clip = File::FileH.clear();
        let spot_7_clip = File::FileA.clear();
        let spot_8_clip = File::FileA.clear() & File::FileB.clear();

        for i in 0..64 {
            let knight_location = 1u64 << i;

            /* The clipping masks we just created will be used to ensure that no
                under or overflow positions are computed when calculating the
                possible moves of the knight in certain files. */

            let spot_1 = (knight_location & spot_1_clip) << 6;
            let spot_2 = (knight_location & spot_2_clip) << 15;
            let spot_3 = (knight_location & spot_3_clip) << 17;
            let spot_4 = (knight_location & spot_4_clip) << 10;

            let spot_5 = (knight_location & spot_5_clip) >> 6;
            let spot_6 = (knight_location & spot_6_clip) >> 15;
            let spot_7 = (knight_location & spot_7_clip) >> 17;
            let spot_8 = (knight_location & spot_8_clip) >> 10;

            knight.knight_move_masks[i] = spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;
        }

        knight
    })
}

pub struct Bishop;

impl Bishop {
}

pub struct Rook;

impl Rook {
}

pub struct Queen;

impl Queen {
}

#[repr(u64)]
enum CastlingMasks {
    WhiteKingSideEmpty = (1u64 << 5) | (1u64 << 6), // F1, G1
    WhiteQueenSideEmpty = (1u64 << 3) | (1u64 << 2) | (1u64 << 1), // D1, C1, B1
    
    BlackKingSideEmpty = (1u64 << 61) | (1u64 << 62), // F8, G8
    BlackQueenSideEmpty = (1u64 << 59) | (1u64 << 58) | (1u64 << 57), // D8, C8, B8
    
    WhiteKingSideAttack = (1u64 << 4) | (1u64 << 5) | (1u64 << 6), // E1, F1, G1
    WhiteQueenSideAttack = (1u64 << 4) | (1u64 << 3) | (1u64 << 2), // E1, D1, C1
    
    BlackKingSideAttack = (1u64 << 60) | (1u64 << 61) | (1u64 << 62), // E8, F8, G8
    BlackQueenSideAttack = (1u64 << 60) | (1u64 << 59) | (1u64 << 58), // E8, D8, C8
}

pub struct King {
    king_move_masks: [u64; 64],
}

impl King {
    pub fn get_move_mask() -> [u64; 64] {
        king().king_move_masks
    }

    pub fn compute_possible_moves(location: u64, chessboard: &Chessboard, turn_color: Color) -> u64 {
        let own_side = chessboard.get_color_pieces(turn_color);
        king().king_move_masks[location.trailing_zeros() as usize] & !own_side
    }

    pub fn compute_possible_castling_moves(location: u64, chessboard: &Chessboard, turn_color: Color) -> u64 {
        let mut castle_king: u64 = 0;
        let mut castle_queen: u64 = 0;

        match turn_color {
            Color::White => {
                if chessboard.state.can_white_king_castle &&
                !chessboard.are_any_squares_occupied(CastlingMasks::WhiteKingSideEmpty as u64) &&
                !chessboard.are_any_squares_attacked_by_color(CastlingMasks::WhiteKingSideAttack as u64, Color::Black) {
                    castle_king = location << 2;
                }

                if chessboard.state.can_white_queen_castle &&
                !chessboard.are_any_squares_occupied(CastlingMasks::WhiteQueenSideEmpty as u64) &&
                !chessboard.are_any_squares_attacked_by_color(CastlingMasks::WhiteQueenSideAttack as u64, Color::Black) {
                    castle_queen = location >> 2;
                }
            },
            Color::Black => {
                if chessboard.state.can_black_king_castle &&
                !chessboard.are_any_squares_occupied(CastlingMasks::BlackKingSideEmpty as u64) &&
                !chessboard.are_any_squares_attacked_by_color(CastlingMasks::BlackKingSideAttack as u64, Color::White) {
                    castle_king = location << 2;
                }
                
                if chessboard.state.can_black_queen_castle &&
                !chessboard.are_any_squares_occupied(CastlingMasks::BlackQueenSideEmpty as u64) &&
                !chessboard.are_any_squares_attacked_by_color(CastlingMasks::BlackQueenSideAttack as u64, Color::White) {
                    castle_queen = location >> 2;
                }
            }
        }

        return castle_king | castle_queen;
    }
}

fn king() -> &'static King {
    static KING: OnceLock<King> = OnceLock::new();
    KING.get_or_init(|| {
        let mut king = King { king_move_masks: [0; 64] };

        for i in 0..64 {
            let location = 1u64 << i;
            let king_clip_file_a: u64 = location & File::FileA.clear();
            let king_clip_file_h: u64 = location & File::FileH.clear();

            /* remember the representation of the board in relation to the bitindex 
                when looking at these shifts.... */
            let spot_1: u64 = location << 8;    // king moves top
            let spot_2: u64 = location >> 8;    // king moves bot

            let spot_3: u64 = king_clip_file_a << 7;   // if king not on file A, moves topleft
            let spot_4: u64 = king_clip_file_a >> 1;   // if king not on file A, moves left
            let spot_5: u64 = king_clip_file_a >> 9;   // if king not on file A, moves bottomleft

            let spot_6: u64 = king_clip_file_h << 9;   // if king not on file H, moves topright
            let spot_7: u64 = king_clip_file_h << 1;   // if king not on file H, moves right
            let spot_8: u64 = king_clip_file_h >> 7;   // if king not on file H, moves bottomright

            king.king_move_masks[i] = spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;
        }

        king
    })
}
