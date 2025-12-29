#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]

use std::collections::HashMap;
use std::{fmt};
use std::str::FromStr;
use serde::Deserialize;
use crate::engine::models::{r#move::Move, piece::{Bishop, King, Knight, Pawn, Piece, Rook, SuperPiece}, state::State};

/// Represents a board rank, or horizontal line. `A1..H1`
#[allow(missing_docs)]
pub enum Rank {
    Rank1, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8
}

impl Rank {
    pub fn mask(self) -> u64 {
        use Rank::*;
        match self {
            Rank1 => 0xFF,
            Rank2 => 0xFF00,
            Rank3 => 0xFF0000,
            Rank4 => 0xFF000000,
            Rank5 => 0xFF00000000,
            Rank6 => 0xFF0000000000,
            Rank7 => 0xFF000000000000,
            Rank8 => 0xFF00000000000000,
        }
    }
        
    pub fn clear(self) -> u64 {
        use Rank::*;
        match self {
            Rank1 => 0xFFFFFFFFFFFFFF00,
            Rank2 => 0xFFFFFFFFFFFF00FF,
            Rank3 => 0xFFFFFFFFFF00FFFF,
            Rank4 => 0xFFFFFFFF00FFFFFF,
            Rank5 => 0xFFFFFF00FFFFFFFF,
            Rank6 => 0xFFFF00FFFFFFFFFF,
            Rank7 => 0xFF00FFFFFFFFFFFF,
            Rank8 => 0x00FFFFFFFFFFFFFF,
        }
    }
}

impl TryFrom<i32> for Rank {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == Rank::Rank1 as i32 => Ok(Rank::Rank1),
            x if x == Rank::Rank2 as i32 => Ok(Rank::Rank2),
            x if x == Rank::Rank3 as i32 => Ok(Rank::Rank3),
            x if x == Rank::Rank4 as i32 => Ok(Rank::Rank4),
            x if x == Rank::Rank5 as i32 => Ok(Rank::Rank5),
            x if x == Rank::Rank6 as i32 => Ok(Rank::Rank6),
            x if x == Rank::Rank7 as i32 => Ok(Rank::Rank7),
            x if x == Rank::Rank8 as i32 => Ok(Rank::Rank8),
            _ => Err("Invalid file value".to_owned())
        }
    }
}

/// Represents a board file, or vertical line. `A1..A8`
#[allow(missing_docs)]
pub enum File {
    FileA, FileB, FileC, FileD, FileE, FileF, FileG, FileH
}

impl File {
    pub fn mask(self) -> u64 {
        use File::*;
        match self {
            FileA => 0x0101010101010101,
            FileB => 0x0202020202020202,
            FileC => 0x0404040404040404,
            FileD => 0x0808080808080808,
            FileE => 0x1010101010101010,
            FileF => 0x2020202020202020,
            FileG => 0x4040404040404040,
            FileH => 0x8080808080808080,
        }
    }

    pub fn clear(self) -> u64 {
        use File::*;
        match self {
            FileA => 0xFEFEFEFEFEFEFEFE,
            FileB => 0xFDFDFDFDFDFDFDFD,
            FileC => 0xFBFBFBFBFBFBFBFB,
            FileD => 0xF7F7F7F7F7F7F7F7,
            FileE => 0xEFEFEFEFEFEFEFEF,
            FileF => 0xDFDFDFDFDFDFDFDF,
            FileG => 0xBFBFBFBFBFBFBFBF,
            FileH => 0x7F7F7F7F7F7F7F7F,
        }
    }
}

impl TryFrom<i32> for File {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == File::FileA as i32 => Ok(File::FileA),
            x if x == File::FileB as i32 => Ok(File::FileB),
            x if x == File::FileC as i32 => Ok(File::FileC),
            x if x == File::FileD as i32 => Ok(File::FileD),
            x if x == File::FileE as i32 => Ok(File::FileE),
            x if x == File::FileF as i32 => Ok(File::FileF),
            x if x == File::FileG as i32 => Ok(File::FileG),
            x if x == File::FileH as i32 => Ok(File::FileH),
            _ => Err("Invalid file value".to_owned())
        }
    }
}

/// Reprensents one of the two piece's color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black
}

/// Constant values of a board state.
#[repr(u64)]
pub enum Board {
    EMPTY = 0u64,
    FULL = u64::MAX
}

impl Board {
    pub fn get_corner_clear() -> u64 {
        0x7EFFFFFFFFFFFF7E
    }

    pub fn get_corner_mask() -> u64 {
        0x8100000000000081
    }

    pub fn get_all_border_clear() -> u64 {
        0x7E7E7E7E7E7E00
    } 

    pub fn get_all_border_mask() -> u64 {
        0xFF818181818181FF
    }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Deserialize)]
#[repr(u64)]
pub enum Square {
    A1 = 1u64 << 0, B1 = 1u64 << 1, C1 = 1u64 << 2, D1 = 1u64 << 3, E1 = 1u64 << 4, F1 = 1u64 << 5, G1 = 1u64 << 6, H1 = 1u64 << 7,
    A2 = 1u64 << 8, B2 = 1u64 << 9, C2 = 1u64 << 10, D2 = 1u64 << 11, E2 = 1u64 << 12, F2 = 1u64 << 13, G2 = 1u64 << 14, H2 = 1u64 << 15, 
    A3 = 1u64 << 16, B3 = 1u64 << 17, C3 = 1u64 << 18, D3 = 1u64 << 19, E3 = 1u64 << 20, F3 = 1u64 << 21, G3 = 1u64 << 22, H3 = 1u64 << 23,
    A4 = 1u64 << 24, B4 = 1u64 << 25, C4 = 1u64 << 26, D4 = 1u64 << 27, E4 = 1u64 << 28, F4 = 1u64 << 29, G4 = 1u64 << 30, H4 = 1u64 << 31,
    A5 = 1u64 << 32, B5 = 1u64 << 33, C5 = 1u64 << 34, D5 = 1u64 << 35, E5 = 1u64 << 36, F5 = 1u64 << 37, G5 = 1u64 << 38, H5 = 1u64 << 39,
    A6 = 1u64 << 40, B6 = 1u64 << 41, C6 = 1u64 << 42, D6 = 1u64 << 43, E6 = 1u64 << 44, F6 = 1u64 << 45, G6 = 1u64 << 46, H6 = 1u64 << 47,
    A7 = 1u64 << 48, B7 = 1u64 << 49, C7 = 1u64 << 50, D7 = 1u64 << 51, E7 = 1u64 << 52, F7 = 1u64 << 53, G7 = 1u64 << 54, H7 = 1u64 << 55,
    A8 = 1u64 << 56, B8 = 1u64 << 57, C8 = 1u64 << 58, D8 = 1u64 << 59, E8 = 1u64 << 60, F8 = 1u64 << 61, G8 = 1u64 << 62, H8 = 1u64 << 63,
}

impl From<u64> for Square {
    fn from(index: u64) -> Self {
        unsafe { std::mem::transmute(index) }
    }
}

fn square_from_str(square: &str) -> Result<Square, &str> {
    if square.len() != 2 {
        return Err("Invalid length for square.");
    }

    let file = square.chars().nth(0).expect("Cannot parse row."); // 'A'..'H'
    let rank = square.chars().nth(1).expect("Cannot parse column."); // '1'..'8'

    let file_index = match file {
        'A'..='H' => (file as u8 - b'A') as u64,
        _ => return Err("Unknown row detected."),
    };

    let rank_index = match rank {
        '1'..='8' => (rank as u8 - b'1') as u64,
        _ => return Err("Unknown column detected."),
    };

    let bit_index = rank_index * 8 + file_index;

    Ok(unsafe { std::mem::transmute(1u64 << bit_index) })
}

/// ```txt
/// |r|n|b|q|k|b|n|r|
/// |p|p|p|p|p|p|p|p|
/// | | | | | | | | |
/// | | | | | | | | |
/// | | | | | | | | |
/// | | | | | | | | |
/// |P|P|P|P|P|P|P|P|
/// |R|N|B|Q|K|B|N|R|
/// ```
pub struct Chessboard {
    pub pieces: [u64; 12],
    pub white_pieces: u64,
    pub black_pieces: u64,

    pub state: State,
    pub state_stack: Vec<State>,
    pub ply_index: usize
}

impl Chessboard {
    /// Default chessboard's constructor initilized with the default fen value, or classic starting position 
    pub fn new() -> Self {
        let pieces = [
            // White pieces (indices 0-5)
            0b11111111 << 8,           // White pawns
            0b10000001,                // White rooks
            0b01000010,                // White knights
            0b00100100,                // White bishops
            0b00010000,                // White queens
            0b00001000,                // White king
            // Black pieces (indices 6-11)
            0b11111111 << 48,          // Black pawns
            0b10000001 << 56,          // Black rooks
            0b01000010 << 56,          // Black knights
            0b00100100 << 56,          // Black bishops
            0b00010000 << 56,          // Black queens
            0b00001000 << 56,          // Black king
        ];
        
        let white_pieces = pieces[0] | pieces[1] | pieces[2] | pieces[3] | pieces[4] | pieces[5];
        let black_pieces = pieces[6] | pieces[7] | pieces[8] | pieces[9] | pieces[10] | pieces[11];
        
        let state = State::default();
        let mut state_stack = Vec::with_capacity(8191); // 8192 - 1
        state_stack[0] = state.clone();

        Chessboard { 
            pieces, 
            white_pieces, 
            black_pieces, 
            state: state,
            state_stack: state_stack,
            ply_index: 0
        }
    }
    
    pub fn from_fen(fen: &str) -> Result<Self, &str> {
        // Initialize variables
        let mut state = State::default();
        let mut chessboard = Chessboard::new();

        // Logic
        let parts: Vec<&str> = fen.split(" ").collect();
        if parts.len() != 6 {
            return Err("Malformed FEN string.");
        }

        // Raw values
        let positions_pieces = parts[0];
        let turn_color = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid turn color detected.")
        };
        let castling_ability = parts[2];
        let en_passant_square = parts[3].to_uppercase();
        let half_moves = u32::from_str(parts[4]).map_err(|_| "Invalid half-moves detected.");
        let full_moves = u32::from_str(parts[5]).map_err(|_| "Invalid full-moves detected.");

        // Save into state
        state.turn_color = turn_color;

        match half_moves {
            Ok(value) => {
                state.half_move_clock = value;
            },
            Err(err) => {
                return Err(err);
            }
        }
        match full_moves {
            Ok(value) => {
                state.full_move_number = value;
            },
            Err(err) => {
                return Err(err);
            }
        }

        for x in castling_ability.chars() {
            match x {
                'K' => {
                    state.can_white_king_castle = true;
                },
                'Q' => {
                    state.can_white_queen_castle = true;
                },
                'k' => {
                    state.can_black_king_castle = true;
                },
                'q' => {
                    state.can_black_queen_castle = true;
                },
                _ => {}
            }
        }

        // Parse En Passant part
        //state.en_passant_square = Square::from(en_passant_square)

        // Parsing piece positions
        let mut raw_piece_to_type: HashMap<char, (Color, Piece)> = HashMap::new();
        raw_piece_to_type.insert('P',(Color::White, Piece::Pawn));
        raw_piece_to_type.insert('N',(Color::White, Piece::Knight));
        raw_piece_to_type.insert('B',(Color::White, Piece::Bishop));
        raw_piece_to_type.insert('R',(Color::White, Piece::Rook));
        raw_piece_to_type.insert('Q',(Color::White, Piece::Queen));
        raw_piece_to_type.insert('K',(Color::White, Piece::King));

        raw_piece_to_type.insert('p',(Color::Black, Piece::Pawn));
        raw_piece_to_type.insert('n',(Color::Black, Piece::Knight));
        raw_piece_to_type.insert('b',(Color::Black, Piece::Bishop));
        raw_piece_to_type.insert('r',(Color::Black, Piece::Rook));
        raw_piece_to_type.insert('q',(Color::Black, Piece::Queen));
        raw_piece_to_type.insert('k',(Color::Black, Piece::King));

        let ranks: Vec<&str> = positions_pieces.split("/").collect();
        let mut overall_index_square = 0;
        if ranks.len() != 8 {
            return Err("Invalid position information detected.");
        }
        for rank in ranks {
            for i in 0..rank.len() - 1 {
                let letter = rank.chars().nth(i).expect("Out-of-bounds error when parsing rank.");
                if letter.is_numeric() {
                    let n_letter = letter.to_digit(10).unwrap();
                    overall_index_square += n_letter;
                } else if let Some((_color, _piece)) = raw_piece_to_type.get(&letter) {
                    let index: usize = 63 as usize - overall_index_square as usize;
                    chessboard.toggle_piece(&mut chessboard.get_piece(*_color, *_piece), 1 << index, *_color);
                    overall_index_square += 1;
                }
            }
        }

        // Set all attributes of chessboard
        chessboard.state = state.clone();

        Ok(chessboard)
    }

    #[inline]
    pub fn get_piece(&self, color: Color, piece: Piece) -> u64 {
        self.pieces[color as usize * 6 + piece as usize]
    }
    
    #[inline]
    pub fn get_all_pieces(&self) -> u64 {
        self.white_pieces | self.black_pieces
    }

    #[inline]
    pub fn get_color_pieces(&self, turn_color: Color) -> u64 {
        match turn_color {
            Color::White => self.white_pieces,
            Color::Black => self.black_pieces,
        }
    }
  
    pub fn should_check_castling(&self) -> bool {
        // Quick checks before expensive castling computation
        return (self.state.turn_color == Color::White && (self.state.can_white_king_castle || self.state.can_black_queen_castle))
            || (self.state.turn_color == Color::Black && (self.state.can_white_king_castle || self.state.can_black_queen_castle));
    }

    /// Checks if a given square is attacked by any other pieces of the opponant color.
    /// 
    /// Used by [Chessboard::any_attacked_squared_by_side]
    fn is_square_attacked_by_color(&self, square: u64, attacking_side: Color) -> bool {
        let square_index: usize = square.trailing_zeros() as usize;
        
        // Check knight attacks
        let knights = self.get_piece(attacking_side, Piece::Knight);
        if (Knight::get_move_masks()[square_index] & knights) != 0 {
            return true;
        }

        // Check rook and queen attacks (straight lines)
        let rooks_queens = self.get_piece(attacking_side, Piece::Queen) | 
                          self.get_piece(attacking_side, Piece::Rook);
        if ((SuperPiece::get_rook_attacks()[square_index] & rooks_queens) != 0)
            && (Rook::compute_possible_attacks(square, self) & rooks_queens) != 0 {
            return true;
        }
            
        // Check bishop and queen attacks (diagonal)
        let bishops_queens = self.get_piece(attacking_side, Piece::Queen) |
                            self.get_piece(attacking_side, Piece::Bishop);
        if ((SuperPiece::get_bishop_attacks()[square_index] & bishops_queens) != 0)
            && ((Bishop::compute_possible_attacks(square, self) & bishops_queens) != 0) {
            return true;
        }

        // Check pawn attacks
        let pawns = self.get_piece(attacking_side, Piece::Pawn);
        let opponent_color = match attacking_side {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        if (Pawn::get_attack_mask()[(opponent_color as usize + 1) * square_index] & pawns) != 0 {
            return true;
        }

        // Check king attacks
        let king = self.get_piece(attacking_side, Piece::King);
        if (King::get_move_masks()[square_index] & king) != 0 {
            return true;
        }

        false
    }

    /// Checks if any given squares (reprensented as 1s in the `squares: u64`) is attacked by the `attacking_side` pieces.
    ///
    /// This is particularly useful for checking castle rights.
    #[inline]
    pub fn any_attacked_squared_by_side(&self, mut squares: u64, attacking_side: Color) -> bool {
        while squares != 0 {
            let square = 1u64 << squares.trailing_zeros();
            squares &= squares - 1;
            if self.is_square_attacked_by_color(square, attacking_side) {
                return true;
            }
        }
        false
    }

    /// Checks is any of the squares (reprensented as 1s in the `squares: u64`) has a piece on it.
    #[inline]
    pub fn any_occupied_square(&self, squares: u64) -> bool {
        (squares & self.get_all_pieces()) != 0
    }

    /// Use this method when required to "slide" a piece, meaning a piece leaving its starting square and ending on its destination square.
    /// 
    /// You should also combine it with [Move::toggle_piece()] when capturing pieces.
    /// 
    /// # Exemple 
    /// ```rust
    /// use chess_engine::engine::models::r#move::Move;
    /// 
    /// let chessboard = Chessboard::new();
    /// // Move a bishop to e4
    /// Move::slide_piece();
    /// // Remove the captured piece
    /// Move::toggle_piece(...);
    /// ```
    #[inline]
    pub fn slide_piece(&mut self, piece_bitboard: &mut u64, from: u64, to: u64, side: Color) {
        *piece_bitboard ^= from ^ to;
        match side {
            Color::White => self.white_pieces ^= from ^ to,
            Color::Black => self.black_pieces ^= from ^ to,
        }
    }

    /// Use this method when required to put a piece without moving one or removing a piece, like during game initialization, captures or promotions.
    pub fn toggle_piece(&mut self, piece_bitboard: &mut u64, square: u64, side: Color) {
        *piece_bitboard ^= square;
        match side {
            Color::White => self.white_pieces ^= square,
            Color::Black => self.black_pieces ^= square,
        }
    }

    /// Make a move on the chessboard itself.
    pub fn make(&mut self, r#move: &Move) {
        todo!()
    }
    
    /// Unmake a move on the chessboard itself.
    pub fn unmake(&mut self, r#move: &Move) {
        todo!()
    }
    
    /// Checks if the current tested side king is in check or not
    pub fn is_in_check(&self, side: Color) -> bool {
        todo!()
    }

    /// Generate all **SPEUDO LEGAL** moves for a given piece and color, updating the `all_pseudo_legal_moves` vector at the same time.
    fn get_all_possible_piece_moves(&self, side: Color, piece: Piece, all_pseudo_legal_moves: &mut Vec<Move>, move_count: &mut usize) {
        todo!()
    }
    
    /// Generate all **SPEUDO LEGAL** moves, updating the `all_pseudo_legal_moves` vector at the same time and returning the number of distinct **SPEUDO LEGAL** moves.
    fn generate_moves(&self, all_pseudo_legal_moves: &mut Vec<Move>) -> usize {
        let mut move_count: usize = 0;
        for i in 0..6 {
            self.get_all_possible_piece_moves(self.state.turn_color, Piece::try_from(i).unwrap(), all_pseudo_legal_moves, &mut move_count);
        }

        return move_count;
    }
    
    /// Performs a `perft` performance and debugging test returning the total number of positions at the end
    pub fn perft(&mut self, depth: u8) -> u64 {
        if depth == 0 {
            return 1u64;
        }

        let mut all_pseudo_legal_moves: Vec<Move> = Vec::with_capacity(256);
        let mut nodes = 0;
        let n_moves: usize;

        n_moves = self.generate_moves(&mut all_pseudo_legal_moves);
        for i in 0..n_moves {
            self.make(&all_pseudo_legal_moves[i]);
            if !self.is_in_check(self.state_stack[self.ply_index].turn_color) {
                nodes += self.perft(depth - 1);
            }
            self.unmake(&all_pseudo_legal_moves[i]);
        }

        return nodes;
    }
}

impl fmt::Display for Chessboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}