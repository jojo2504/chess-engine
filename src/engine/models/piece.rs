#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::{collections::HashMap, sync::OnceLock};

use crate::{engine::{magic::Magic, models::{board::{Board, Chessboard, Color, File, Rank}, r#move::MoveKind}}, utils::bit_operations::{index_to_bitboard, transform}};

/// Quick enum to match pieces
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    pub const ALL: [Piece; 6] = [
        Piece::Pawn,
        Piece::Rook,
        Piece::Knight,
        Piece::Bishop,
        Piece::Queen,
        Piece::King,
    ];
}

impl TryFrom<i32> for Piece {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == Piece::Pawn as i32 => Ok(Piece::Pawn),
            x if x == Piece::Rook as i32 => Ok(Piece::Rook),
            x if x == Piece::Knight as i32 => Ok(Piece::Knight),
            x if x == Piece::Bishop as i32 => Ok(Piece::Bishop),
            x if x == Piece::Queen as i32 => Ok(Piece::Queen),
            x if x == Piece::King as i32 => Ok(Piece::King),
            _ => Err("Invalid piece value".to_owned())
        }
    }
}

impl From<Piece> for char {
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        }
    }
}

/*
    TELEPORTING PIECES
*/

/// Represents a pawn piece in chess with precomputed attack patterns.
///
/// This struct contains precomputed bitboards for pawn attacks from all possible
/// square positions on the chessboard. The attack masks are stored in an array
/// indexed by square position (0-63 for valid squares, with the extra positions
/// potentially used for padding or to handle both white and black pawns).
///
/// # Fields
///
/// * `pawn_attack_masks` - An array of 128 bitboards representing the attack patterns
///   for pawns from different positions. Each bitboard represents the squares that
///   a pawn can attack from a given position.
pub struct Pawn {
    /// Precomputed attack mask for every pawn position for both side, white then black.
    pawn_attack_masks: [u64; 128],
    promotion_map: HashMap<char, u8>
}

impl Pawn {
    /// Returns the precomputed attack masks for pawns.
    pub fn get_attack_mask() -> [u64; 128] {
        pawn().pawn_attack_masks
    }

    /// Returns a precalculated char to byte move code hashmap.
    pub(crate) fn get_promotion_map() -> HashMap<char, u8> {
        pawn().promotion_map.clone()
    }

    /// Compute possible moves for a given pawn and its color.
    pub(crate) fn compute_possible_moves(location: u64, chessboard: &Chessboard, turn_color: Color) -> u64 {
        let moves = match turn_color {
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
                pawn_valid_moves | pawn_valid_attacks
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
                pawn_valid_moves | pawn_valid_attacks
            }
        };
        let own_side = chessboard.get_color_pieces(turn_color);
        moves & !own_side
    }

    pub(crate) fn compute_possible_attacks(location: u64, chessboard: &Chessboard, turn_color: Color) -> u64 {
        let own_side = chessboard.get_color_pieces(turn_color);
        pawn().pawn_attack_masks[(turn_color as usize + 1) * location.trailing_zeros() as usize] & !own_side
    }
}

/// Lazy static initializer for [Pawn].
fn pawn() -> &'static Pawn {
    static PAWN: OnceLock<Pawn> = OnceLock::new();
    PAWN.get_or_init(|| {
        let mut pawn = Pawn { 
            pawn_attack_masks: [0; 128] ,
            promotion_map: HashMap::new()
        };
        
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

        pawn.promotion_map.insert('n', MoveKind::KnightPromotion as u8);
        pawn.promotion_map.insert('b', MoveKind::BishopPromotion as u8);
        pawn.promotion_map.insert('r', MoveKind::RookPromotion as u8);
        pawn.promotion_map.insert('q', MoveKind::QueenPromotion as u8);

        pawn
    })
}

/// Represents a knight piece in chess with precomputed move generation data.
///
/// The knight moves in an L-shape: two squares in one direction and one square perpendicular,
/// or one square in one direction and two squares perpendicular. This struct stores
/// precomputed attack masks for all 64 possible knight positions on the board for
/// efficient move generation.
pub(crate) struct Knight {
    /// Precomputed attack mask for every knight position.
    knight_move_masks: [u64; 64]
}

impl Knight {
    /// Returns the precomputed move masks for all squares.
    pub(crate) fn get_move_masks() -> [u64; 64] {
        knight().knight_move_masks
    }

    /// Computes the possible moves for a knight at a given location,
    /// taking into account the current board state and own pieces.
    ///
    /// # Arguments
    /// * `location` - A bitboard with a single bit set for the knight's position.
    /// * `chessboard` - The current chessboard state.
    /// * `turn_color` - The color of the player whose turn it is.
    ///
    /// # Returns
    /// A bitboard representing all legal destination squares for the knight.
    pub(crate) fn compute_possible_moves(location: u64, chessboard: &Chessboard, turn_color: Color) -> u64 {
        let own_side = chessboard.get_color_pieces(turn_color);
        knight().knight_move_masks[location.trailing_zeros() as usize] & !own_side
    }
}

/// Lazy static initializer for [Knight].
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

/// Masks used to facilitate the castling right check process.
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

/// Represents a King piece in chess with precomputed move generation data.
///
/// The knight moves 1 square in every direction, but can also do `Castle` moves.
pub(crate) struct King {
    /// Precomputed attack mask for every king position.
    king_move_masks: [u64; 64],
}

impl King {
    /// Returns the king move mask.
    pub(crate) fn get_move_masks() -> [u64; 64] {
        king().king_move_masks
    }

    /// Compute and returns the possible king moves without overlapping its own pieces.
    pub(crate) fn compute_possible_moves(location: u64, chessboard: &Chessboard, turn_color: Color) -> u64 {
        let own_side = chessboard.get_color_pieces(turn_color);
        king().king_move_masks[location.trailing_zeros() as usize] & !own_side
    }

    /// Compute the available castling square for a king depending on the chessboard's context and state.
    pub(crate) fn compute_possible_castling_moves(location: u64, chessboard: &Chessboard, turn_color: Color) -> u64 {
        let mut castle_king: u64 = 0;
        let mut castle_queen: u64 = 0;

        match turn_color {
            Color::White => {
                if chessboard.state.can_white_king_castle &&
                !chessboard.any_occupied_square(CastlingMasks::WhiteKingSideEmpty as u64) &&
                !chessboard.any_attacked_squared_by_side(CastlingMasks::WhiteKingSideAttack as u64, Color::Black) {
                    castle_king = location << 2;
                }

                if chessboard.state.can_white_queen_castle &&
                !chessboard.any_occupied_square(CastlingMasks::WhiteQueenSideEmpty as u64) &&
                !chessboard.any_attacked_squared_by_side(CastlingMasks::WhiteQueenSideAttack as u64, Color::Black) {
                    castle_queen = location >> 2;
                }
            },
            Color::Black => {
                if chessboard.state.can_black_king_castle &&
                !chessboard.any_occupied_square(CastlingMasks::BlackKingSideEmpty as u64) &&
                !chessboard.any_attacked_squared_by_side(CastlingMasks::BlackKingSideAttack as u64, Color::White) {
                    castle_king = location << 2;
                }
                
                if chessboard.state.can_black_queen_castle &&
                !chessboard.any_occupied_square(CastlingMasks::BlackQueenSideEmpty as u64) &&
                !chessboard.any_attacked_squared_by_side(CastlingMasks::BlackQueenSideAttack as u64, Color::White) {
                    castle_queen = location >> 2;
                }
            }
        }

        castle_king | castle_queen
    }
}

/// Lazy static initializer for [King].
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

/*
    SLIDING PIECES
*/

/// Represents a bishop piece in chess with precomputed attack patterns using magic bitboards.
///
/// The bishop moves diagonally any number of squares. This struct uses magic bitboards
/// for efficient move generation, storing precomputed attack patterns for all possible
/// blocker configurations.
pub(crate) struct Bishop {
    /// Precomputed blocker mask for every bishop position.
    bishop_blocker_mask: [u64; 64],
    /// Magic bitboard table for transforming occupancy to attack indices.
    bishop_magic_table: Vec<Magic>,
    /// Precomputed attack patterns indexed by square and magic-transformed occupancy.
    magic_bishop_attacks: Box<[[u64; 4096]; 64]> 
}
    
impl Bishop {
    /// From a given bishop `location`, `turn_color` and `chessboard`, returns a bitboard representing all the valid squares it can move to
    /// without checking if it's really valid (is pseudolegal move) or capturing a piece yet. Note that it may overlap with the opponant
    /// all pieces bitboard because of possible captures.
    pub(crate) fn compute_possible_moves(location: u64, chessboard: &Chessboard, turn_color: Color) -> u64 {
        let own_side = chessboard.get_color_pieces(turn_color);
        Bishop::rays(location, chessboard) & !own_side
    }

    /// Using the magic bitboard `table` and very precise operations, returns all the `rays` of the bishop. Note that it's not checking for any side yet. 
    pub(crate) fn rays(location: u64, chessboard: &Chessboard) -> u64 {
        let sq: usize = location.trailing_zeros() as usize;
        let mut occ = chessboard.get_all_pieces();

        occ &= bishop().bishop_magic_table[sq].mask;
        occ = occ.wrapping_mul(bishop().bishop_magic_table[sq].magic_number);
        occ >>= 55; //64-9

        bishop().magic_bishop_attacks[sq][occ as usize]
    }
    
    /// Compute the rays of the bishop from a given square and block mask
    fn batt(square: i32, block: u64) -> u64 {
        let mut result: u64 = 0u64;
        let rk = square / 8;
        let fl = square % 8;
        let mut r: i32;
        let mut f: i32;

        r = rk + 1;
        f = fl + 1;
        while r <= 7 && f <= 7 {
            result |= 1u64 << (f + r * 8);
            if (block & (1u64 << (f + r * 8))) != 0 { break; }
            r += 1;
            f += 1;
        }

        r = rk + 1;
        f = fl - 1;
        while r <= 7 && f >= 0 {
            result |= 1u64 << (f + r * 8);
            if (block & (1u64 << (f + r * 8))) != 0 { break; }
            r += 1;
            f -= 1;
        }

        r = rk - 1;
        f = fl + 1;
        while r >= 0 && f <= 7 {
            result |= 1u64 << (f + r * 8);
            if (block & (1u64 << (f + r * 8))) != 0 { break; }
            r -= 1;
            f += 1;
        }

        r = rk - 1;
        f = fl - 1;
        while r >= 0 && f >= 0 {
            result |= 1u64 << (f + r * 8);
            if (block & (1u64 << (f + r * 8))) != 0 { break; }
            r -= 1;
            f -= 1;
        }

        result
    }
}

/// Lazy static initializer for [Bishop].
fn bishop() -> &'static Bishop {
    static BISHOP: OnceLock<Bishop> = OnceLock::new();
    BISHOP.get_or_init(|| {
        let mut bishop = Bishop {
            bishop_blocker_mask: [0; 64],
            bishop_magic_table: Magic::load_magic_table("src/engine/magic/BMagicTable.json").expect("bishop magic table should be found here"),
            magic_bishop_attacks: Box::new([[0; 4096]; 64]) 
        };

        // init blocker mask
        for sq in 0..64 {
            let mut result = 0u64;
            let rk = sq / 8;
            let fl = sq % 8;

            for d in 1..7 {
                if rk + d <= 6 && fl + d <= 6 {
                    result |= 1u64 << ((rk + d) * 8 + fl + d);
                }
                if rk + d <= 6 && fl > d {
                    result |= 1u64 << ((rk + d) * 8 + fl - d);
                }
                if rk > d && fl + d <= 6 {
                    result |= 1u64 << ((rk - d) * 8 + fl + d);
                }
                if rk > d && fl > d {
                    result |= 1u64 << ((rk - d) * 8 + fl - d);
                }
            }
            bishop.bishop_blocker_mask[sq] = result;
        }

        // init bishop attacks
        for sq in 0..64 {
            let mask = bishop.bishop_magic_table[sq].mask;
            let relevant_bits_number = mask.count_ones();

            for i in 0..(1 << relevant_bits_number) {
                let occupancy = index_to_bitboard(i, relevant_bits_number, mask);
                let attacks = Bishop::batt(sq as i32, occupancy);

                // Transform occupancy to magic index
                let masked_occ = occupancy & mask;
                let magic_index = transform(masked_occ, bishop.bishop_magic_table[sq].magic_number, 9);

                // Store the attacks in your lookup table
                bishop.magic_bishop_attacks[sq][magic_index as usize] = attacks;
            }
        }

        bishop
    })
}

/// Represents a rook piece in chess with precomputed attack patterns using magic bitboards.
///
/// The rook moves horizontally or vertically any number of squares. This struct uses magic bitboards
/// for efficient move generation, storing precomputed attack patterns for all possible
/// blocker configurations.
pub(crate) struct Rook {
    /// Precomputed blocker mask for every rook position.
    rook_blocker_mask: [u64; 64],
    /// Magic bitboard table for transforming occupancy to attack indices.
    rook_magic_table: Vec<Magic>,
    /// Precomputed attack patterns indexed by square and magic-transformed occupancy.
    magic_rook_attacks: Box<[[u64; 4096]; 64]> 
}
    
impl Rook {
    pub(crate) const WHITE_CASTLING_MASK: u64 = 0x81;  // A1 | H1
    pub(crate) const BLACK_CASTLING_MASK: u64 = 0x8100000000000000; // A8 | H8

    /// From a given rook `location`, `turn_color` and `chessboard`, returns a bitboard representing all the valid squares it can move to
    /// without checking if it's really valid (is pseudolegal move) or capturing a piece yet. Note that it may overlap with the opponant
    /// all pieces bitboard because of possible captures.
    pub(crate) fn compute_possible_moves(location: u64, chessboard: &Chessboard, turn_color: Color) -> u64 {
        let own_side = chessboard.get_color_pieces(turn_color);
        Rook::rays(location, chessboard) & !own_side
    }

    /// Using the magic bitboard `table` and very precise operations, returns all the `rays` of the rook. Note that it's not checking for any side yet. 
    pub(crate) fn rays(location: u64, chessboard: &Chessboard) -> u64 {
        let sq: usize = location.trailing_zeros() as usize;
        let mut occ = chessboard.get_all_pieces();

        occ &= rook().rook_magic_table[sq].mask;
        occ = occ.wrapping_mul(rook().rook_magic_table[sq].magic_number);
        occ >>= 52; //64-12

        rook().magic_rook_attacks[sq][occ as usize]
    }
    
    /// Compute the rays of the rook from a given square and block mask
    fn ratt(square: i32, block: u64) -> u64 {
        let mut result: u64 = 0u64;
        let rk = square / 8;
        let fl = square % 8;
        let mut r: i32;
        let mut f: i32;

        r = rk + 1;
        while r <= 7 {
            result |= 1u64 << (fl + r * 8);
            if (block & (1u64 << (fl + r * 8))) != 0 { break; }
            r += 1;
        }
        r = rk - 1;
        while r >= 0 {
            result |= 1u64 << (fl + r * 8);
            if (block & (1u64 << (fl + r * 8))) != 0 { break; }
            r -= 1;
        }
        f = fl + 1;
        while f <= 7 {
            result |= 1u64 << (f + rk * 8);
            if (block & (1u64 << (f + rk * 8))) != 0 { break; }
            f += 1;
        }
        f = fl - 1;
        while f >= 0 {
            result |= 1u64 << (f + rk * 8);
            if (block & (1u64 << (f + rk * 8))) != 0 { break; }
            f -= 1;
        }
        result
    }
}

/// Lazy static initializer for [Rook].
fn rook() -> &'static Rook {
    static ROOK: OnceLock<Rook> = OnceLock::new();
    ROOK.get_or_init(|| {
        let mut rook = Rook {
            rook_blocker_mask: [0; 64],
            rook_magic_table: Magic::load_magic_table("src/engine/magic/RMagicTable.json").expect("rook magic table should be found here"),
            magic_rook_attacks: Box::new([[0; 4096]; 64]) 
        };

        // init blocker mask
        for i in 0..64 {
            let file_index: i32 = i % 8;
            let rank_index: i32 = i / 8;

            let mut blocker_mask: u64 = 0u64;
            blocker_mask |= File::try_from(file_index).unwrap().mask();
            blocker_mask ^= Rank::try_from(rank_index).unwrap().mask();

            // remove the 4 corners
            blocker_mask &= Board::get_corner_clear();

            // checks if not on border
            if ((1u64 << i) & Board::get_all_border_mask()) != 0 {
                blocker_mask &= Board::get_all_border_clear();
            }

            rook.rook_blocker_mask[i as usize] = blocker_mask;
        }

        // init rook attacks
        for sq in 0..64 {
            let mask = rook.rook_magic_table[sq].mask;
            let relevant_bits_number = mask.count_ones();

            for i in 0..(1 << relevant_bits_number) {
                let occupancy = index_to_bitboard(i, relevant_bits_number, mask);
                let attacks = Rook::ratt(sq as i32, occupancy);

                // Transform occupancy to magic index
                let masked_occ = occupancy & mask;
                let magic_index = transform(masked_occ, rook.rook_magic_table[sq].magic_number, 12);

                // Store the attacks in your lookup table
                rook.magic_rook_attacks[sq][magic_index as usize] = attacks;
            }
        }

        rook
    })
}

// Represents a queen piece in chess with precomputed attack patterns using magic bitboards.
///
/// The queen merges the moves of both the [Rook] and the [Bishop].
pub(crate) struct Queen;

impl Queen {
    /// Compute the possible moves for the [Queen].
    pub(crate) fn compute_possible_moves(location: u64, chessboard: &Chessboard, turncolor: Color) -> u64 {
        Rook::compute_possible_moves(location, chessboard, turncolor) | 
        Bishop::compute_possible_moves(location, chessboard, turncolor)
    }
}

/// This special piece is used to help checking if a square is attacked by any piece, by casting its attack directly to them
pub(crate) struct SuperPiece {
    /// Rook-like rays for each square.
    rook_rays: [u64; 64],
    /// Bishop-like rays for each square.
    bishop_rays: [u64; 64],
    /// Knight attack masks for each square.
    knight_rays: [u64; 64],
}

impl SuperPiece {
    /// Returns an array of all rook rays for each square, simulating an empty board.
    pub(crate) fn rook_rays() -> [u64; 64] {
        super_piece().rook_rays
    }
    
    /// Returns an array of all bishop rays for each square, simulating an empty board.
    pub(crate) fn bishop_rays() -> [u64; 64] {
        super_piece().bishop_rays
    }

    /// Returns an array of all knight attack masks for each square.
    pub(crate) fn knight_rays() -> [u64; 64] {
        super_piece().knight_rays
    }
}

/// Lazy static initializer for [SuperPiece].
fn super_piece() -> &'static SuperPiece {
    static SUPER_PIECE: OnceLock<SuperPiece> = OnceLock::new();
    SUPER_PIECE.get_or_init(|| {
        let mut super_piece = SuperPiece {
            rook_rays: [0; 64],
            bishop_rays: [0; 64],
            knight_rays: [0; 64],
        };

        for i in 0..64 {
            super_piece.rook_rays[i] = Rook::ratt(i as i32, 0u64);
            super_piece.bishop_rays[i] = Bishop::batt(i as i32, 0u64);
            super_piece.knight_rays[i] = Knight::get_move_masks()[i];
        }

        super_piece
    })
}