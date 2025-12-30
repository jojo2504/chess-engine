#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use serde::Deserialize;
use crate::engine::models::{r#move::Move, piece::{Bishop, King, Knight, Pawn, Piece, Rook, SuperPiece}, state::State};

/// Represents a board rank, or horizontal line. `A1..H1`
#[allow(missing_docs)]
pub(crate) enum Rank {
    Rank1, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8
}

impl Rank {
    /// Returns the mask of a rank.
    pub(crate) fn mask(self) -> u64 {
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
    
    /// Returns the clear mask of a rank which is equivalent to `!mask`.
    pub(crate) fn clear(self) -> u64 {
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

    /// Convert an `i32` into a Rank, should only be used in a context where the conversion is infallible.
    pub(crate) fn from_i32_unchecked(value: i32) -> Self {
        match value {
            x if x == Rank::Rank1 as i32 => Rank::Rank1,
            x if x == Rank::Rank2 as i32 => Rank::Rank2,
            x if x == Rank::Rank3 as i32 => Rank::Rank3,
            x if x == Rank::Rank4 as i32 => Rank::Rank4,
            x if x == Rank::Rank5 as i32 => Rank::Rank5,
            x if x == Rank::Rank6 as i32 => Rank::Rank6,
            x if x == Rank::Rank7 as i32 => Rank::Rank7,
            x if x == Rank::Rank8 as i32 => Rank::Rank8,
            _ => unreachable!("Invalid file value")
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
pub(crate) enum File {
    FileA, FileB, FileC, FileD, FileE, FileF, FileG, FileH
}

impl File {
    /// Returns the mask of a file.
    pub(crate) fn mask(self) -> u64 {
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

    /// Returns the clear mask of a file which is equivalent to `!mask`.
    pub(crate) fn clear(self) -> u64 {
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

    /// Convert an `i32` into a File, should only be used in a context where the conversion is infallible.
    pub(crate) fn from_i32_unchecked(value: i32) -> Self {
        match value {
            x if x == File::FileA as i32 => File::FileA,
            x if x == File::FileB as i32 => File::FileB,
            x if x == File::FileC as i32 => File::FileC,
            x if x == File::FileD as i32 => File::FileD,
            x if x == File::FileE as i32 => File::FileE,
            x if x == File::FileF as i32 => File::FileF,
            x if x == File::FileG as i32 => File::FileG,
            x if x == File::FileH as i32 => File::FileH,
            _ => unreachable!("Invalid file value")
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
pub(crate) enum Color {
    /// Used to represent the white pieces or white turn.
    White,
    /// Used to represent the black pieces or black turn.
    Black
}

/// Constant values of a board state.
#[allow(clippy::upper_case_acronyms)]
#[repr(u64)]
pub(crate) enum Board {
    /// Constant value to a empty board (only `0`s).
    EMPTY = 0u64,
    /// Constant value to a full board (only `1`s).
    FULL = u64::MAX
}

impl Board {
    /// Full board minus the 4 corners.
    pub(crate) fn get_corner_clear() -> u64 {
        0x7EFFFFFFFFFFFF7E
    }
    
    /// Only the 4 corners of the chessboard.
    pub(crate) fn get_corner_mask() -> u64 {
        0x8100000000000081
    }
    
    /// Full board minus the 4 borders.
    pub(crate) fn get_all_border_clear() -> u64 {
        0x7E7E7E7E7E7E00
    } 
    
    /// Only the 4 borders of the chessboard.
    pub(crate) fn get_all_border_mask() -> u64 {
        0xFF818181818181FF
    }
}

#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) enum Square {
    A1 = 0, B1 = 1, C1 = 2, D1 = 3, E1 = 4, F1 = 5, G1 = 6, H1 = 7,
    A2 = 8, B2 = 9, C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
    A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
    A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
    A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
    A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
    A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
    A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
}

impl Square {
    /// Get bitboard mask for this square
    pub(crate) const fn bitboard(self) -> u64 {
        1u64 << (self as u64)
    }
    
    /// Get file
    pub(crate) fn file(self) -> File {
        File::from_i32_unchecked(self as i32 % 8)
    }
    
    /// Get rank
    pub(crate) fn rank(self) -> Rank {
        Rank::from_i32_unchecked(self as i32 % 8)
    }
}

impl TryFrom<u64> for Square {
    type Error = String;

    fn try_from(index: u64) -> Result<Self, Self::Error> {
        if index > 63 {
            return Err(format!("Index {} out of range (0-63)", index));
        }
        // Safe because we validated the range
        Ok(unsafe { std::mem::transmute(index as u8) })
    }
}

impl From<Square> for u64 {
    fn from(square: Square) -> u64 {
        square as u64
    }
}

impl FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(format!("Invalid square format: {}", s));
        }
        
        let chars: Vec<char> = s.to_uppercase().chars().collect();
        let file = match chars[0] {
            'A' => 0, 'B' => 1, 'C' => 2, 'D' => 3,
            'E' => 4, 'F' => 5, 'G' => 6, 'H' => 7,
            _ => return Err(format!("Invalid file: {}", chars[0])),
        };
        
        let rank = match chars[1] {
            '1' => 0, '2' => 1, '3' => 2, '4' => 3,
            '5' => 4, '6' => 5, '7' => 6, '8' => 7,
            _ => return Err(format!("Invalid rank: {}", chars[1])),
        };
        
        let index = rank * 8 + file;
        Square::try_from(index as u64)
    }
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
    /// The 12 bitboards for each piece, starting with white then black, same order as [Piece].
    pub(crate) pieces: [u64; 12],
    /// Bitboard representing the position of all white pieces.
    pub(crate) white_pieces: u64,
    /// Bitboard representing the position of all black pieces.
    pub(crate) black_pieces: u64,

    /// Current state of the chessboard.
    pub(crate) state: State,
    /// Used to keep track of all previous and current states of the chessboard. 
    pub(crate) state_stack: Vec<State>,
    /// Used to index the state_stack, representing the current ply, equivalent to a half-move.
    pub(crate) ply_index: usize
}

impl Chessboard {
    /// Default chessboard's constructor initialized with the default fen value, or classic starting position.
    /// 
    /// This will call [Chessboard::from_fen] with:
    /// ```rust
    /// let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// ```
    #[allow(clippy::unwrap_used, reason="The default fen will always works")]
    pub(crate) fn new() -> Self {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Self::from_fen(fen).unwrap()
    }
    
    /// Chessboard's constructor initialized with a custom fen value.
    pub(crate) fn from_fen(fen: &str) -> Result<Self, &str> {
        // Initialize variables
        let mut chessboard = Chessboard::default();

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
        chessboard.state.turn_color = turn_color;
        
        match half_moves {
            Ok(value) => {
                chessboard.state.half_move_clock = value;
            },
            Err(err) => {
                return Err(err);
            }
        }
        match full_moves {
            Ok(value) => {
                chessboard.state.full_move_number = value;
            },
            Err(err) => {
                return Err(err);
            }
        }
        
        for x in castling_ability.chars() {
            match x {
                'K' => {
                    chessboard.state.can_white_king_castle = true;
                },
                'Q' => {
                    chessboard.state.can_white_queen_castle = true;
                },
                'k' => {
                    chessboard.state.can_black_king_castle = true;
                },
                'q' => {
                    chessboard.state.can_black_queen_castle = true;
                },
                _ => {}
            }
        }
        
        // Parse En Passant part
        chessboard.state.en_passant_square = None;
        if let Ok(ep_square) = Square::from_str(&en_passant_square) {
            chessboard.state.en_passant_square = Some(ep_square);
        }

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
                    let index: usize = 63_usize - overall_index_square as usize;
                    chessboard.toggle_piece(&mut chessboard.get_piece(*_color, *_piece), 1 << index, *_color);
                    overall_index_square += 1;
                }
            }
        }

        Ok(chessboard)
    }

    /// Returns the bitboard corresponding to the searched piece.
    /// 
    /// For example, `self.get_piece(Color::White, Piece::Pawn)` returns the bitboard with all white pawns.
    #[inline]
    pub(crate) fn get_piece(&self, color: Color, piece: Piece) -> u64 {
        self.pieces[color as usize * 6 + piece as usize]
    }
    
    /// Returns a bitboard with all pieces on chessboard.
    #[inline]
    pub(crate) fn get_all_pieces(&self) -> u64 {
        self.white_pieces | self.black_pieces
    }

    /// Returns a bitboard with all pieces from a color.
    /// 
    /// For Example, calling `self.get_color_pieces(Color::White)` returns the bitboard with all white pieces.
    #[inline]
    pub(crate) fn get_color_pieces(&self, turn_color: Color) -> u64 {
        match turn_color {
            Color::White => self.white_pieces,
            Color::Black => self.black_pieces,
        }
    }
    
    // ? Not sure if we keep it
    /// Quick checks before expensive castling computation
    pub(crate) fn should_check_castling(&self) -> bool {
        (self.state.turn_color == Color::White && (self.state.can_white_king_castle || self.state.can_white_queen_castle))
            || (self.state.turn_color == Color::Black && (self.state.can_black_king_castle || self.state.can_black_queen_castle))
    }

    /// Checks if a given square is attacked by any other pieces of the opponant color.
    /// 
    /// Used by [Chessboard::any_attacked_squared_by_side].
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
        if ((SuperPiece::rook_rays()[square_index] & rooks_queens) != 0)
            && (Rook::rays(square, self) & rooks_queens) != 0 {
            return true;
        }
            
        // Check bishop and queen attacks (diagonal)
        let bishops_queens = self.get_piece(attacking_side, Piece::Queen) |
                            self.get_piece(attacking_side, Piece::Bishop);
        if ((SuperPiece::bishop_rays()[square_index] & bishops_queens) != 0)
            && ((Bishop::rays(square, self) & bishops_queens) != 0) {
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
    pub(crate) fn any_attacked_squared_by_side(&self, mut squares: u64, attacking_side: Color) -> bool {
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
    pub(crate) fn any_occupied_square(&self, squares: u64) -> bool {
        (squares & self.get_all_pieces()) != 0
    }

    /// Use this method when required to "slide" a piece, meaning a piece leaving its starting square and ending on its destination square.
    /// 
    /// You should also combine it with [Chessboard::toggle_piece] when capturing pieces.
    /// 
    /// # Exemple 
    /// ```rust
    /// use chess_engine::engine::models::board::{Chessboard, Square, Color};
    /// use chess_engine::engine::models::piece::Piece;
    /// 
    /// let mut chessboard = Chessboard::new();
    /// // Move a pawn from A2 to A3
    /// chessboard.slide_piece(&mut chessboard.get_piece(Color::White, Piece::Pawn), Square::A2.bitboard(), Square::A3.bitboard(), Color::White);
    /// // Remove a captured piece (random square in this example)
    /// chessboard.toggle_piece(&mut chessboard.get_piece(Color::White, Piece::Pawn), Square::A2.bitboard(), Color::White);
    /// ```
    #[inline]
    pub(crate) fn slide_piece(&mut self, piece_bitboard: &mut u64, from: u64, to: u64, side: Color) {
        
        *piece_bitboard ^= from ^ to;
        match side {
            Color::White => self.white_pieces ^= from ^ to,
            Color::Black => self.black_pieces ^= from ^ to,
        }
    }

    /// Use this method when required to put a piece without moving one or removing a piece, like during game initialization, captures or promotions.
    pub(crate) fn toggle_piece(&mut self, piece_bitboard: &mut u64, square: u64, side: Color) {
        *piece_bitboard ^= square;
        match side {
            Color::White => self.white_pieces ^= square,
            Color::Black => self.black_pieces ^= square,
        }
    }

    /// Make a move on the chessboard itself.
    pub(crate) fn make(&mut self, r#move: &Move) {
        todo!()
    }
    
    /// Unmake a move on the chessboard itself.
    pub(crate) fn unmake(&mut self, r#move: &Move) {
        todo!()
    }
    
    /// Checks if the current tested side king is in check or not
    pub(crate) fn is_in_check(&self, side: Color) -> bool {
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

        move_count
    }
    
    /// Performs a `perft` performance and debugging test returning the total number of positions at the end
    pub fn perft(&mut self, depth: u8) -> u64 {
        if depth == 0 {
            return 1u64;
        }

        let mut all_pseudo_legal_moves: Vec<Move> = Vec::with_capacity(256);
        let mut nodes = 0;
        

        let n_moves: usize = self.generate_moves(&mut all_pseudo_legal_moves);
        for i in 0..n_moves {
            self.make(&all_pseudo_legal_moves[i]);
            if !self.is_in_check(self.state_stack[self.ply_index].turn_color) {
                nodes += self.perft(depth - 1);
            }
            self.unmake(&all_pseudo_legal_moves[i]);
        }

        nodes
    }
}

impl Default for Chessboard {
    fn default() -> Self {
        Self {
            pieces: [0; 12],
            white_pieces: 0u64,
            black_pieces: 0u64,
            state: State::default(),
            state_stack: Vec::with_capacity(8191),
            ply_index: 0,
        }
    }
}

impl fmt::Display for Chessboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}