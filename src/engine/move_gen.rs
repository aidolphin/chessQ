// src/engine/move_gen.rs
use crate::engine::bitboard::Square;
use crate::engine::magic::MagicTables;
use crate::state::game_state::GameState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn, Knight, Bishop, Rook, Queen, King
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub piece: PieceType,
    pub capture: Option<PieceType>,
    pub promotion: Option<PieceType>,
    pub is_castle: bool,
    pub is_en_passant: bool,
}

impl Move {
    pub fn new(from: Square, to: Square, piece: PieceType) -> Self {
        Self {
            from,
            to,
            piece,
            capture: None,
            promotion: None,
            is_castle: false,
            is_en_passant: false,
        }
    }
    
    pub fn algebraic(&self) -> String {
        let mut notation = self.from_to_str();
        if let Some(promotion) = self.promotion {
            notation.push('=');
            notation.push(match promotion {
                PieceType::Queen => 'Q',
                PieceType::Rook => 'R',
                PieceType::Bishop => 'B',
                PieceType::Knight => 'N',
                PieceType::Pawn => 'P',
                PieceType::King => 'K',
            });
        }
        notation
    }
    
    fn from_to_str(&self) -> String {
        format!("{}{}", self.square_to_str(self.from), 
                self.square_to_str(self.to))
    }
    
    fn square_to_str(&self, square: Square) -> String {
        let file = match square.file() {
            0 => 'a', 1 => 'b', 2 => 'c', 3 => 'd',
            4 => 'e', 5 => 'f', 6 => 'g', 7 => 'h',
            _ => unreachable!(),
        };
        let rank = (square.rank() + 1) as u8;
        format!("{}{}", file, rank)
    }
}

/// Move generator using bitboards
pub struct MoveGenerator {
    magic_tables: MagicTables,
}

impl MoveGenerator {
    pub fn new() -> Self {
        Self {
            magic_tables: MagicTables::init(),
        }
    }
    
    /// Generate all pseudo-legal moves for a position
    pub fn generate_moves(&self, state: &GameState) -> Vec<Move> {
        let mut moves = Vec::new();
        let side = state.side_to_move;
        
        // Generate moves for each piece type
        self.generate_pawn_moves(state, side, &mut moves);
        self.generate_knight_moves(state, side, &mut moves);
        self.generate_bishop_moves(state, side, &mut moves);
        self.generate_rook_moves(state, side, &mut moves);
        self.generate_queen_moves(state, side, &mut moves);
        self.generate_king_moves(state, side, &mut moves);
        
        // Filter to only legal moves (not leaving king in check)
        moves.retain(|mv| self.is_legal_move(state, mv));
        
        moves
    }
    
    fn generate_pawn_moves(&self, state: &GameState, color: Color, moves: &mut Vec<Move>) {
        let pawns = if color == Color::White { state.white_pawns } else { state.black_pawns };
        let occupied = state.all_pieces();
        let direction = if color == Color::White { 8 } else { -8 };
        let start_rank = if color == Color::White { 1 } else { 6 };
        let en_passant_rank = if color == Color::White { 4 } else { 3 };
        
        for from_sq in pawns.iter() {
            let from = Square::from_index(from_sq).unwrap();
            let to_idx = from_sq as i32 + direction;
            
            // Single push
            if (0..64).contains(&to_idx) && !occupied.has_piece(to_idx as usize) {
                self.add_pawn_move(from, Square::from_index(to_idx as usize).unwrap(), color, moves);
                
                // Double push
                if from.rank() == start_rank {
                    let to2_idx = from_sq as i32 + 2 * direction;
                    if (0..64).contains(&to2_idx) && !occupied.has_piece(to2_idx as usize) {
                        moves.push(Move::new(from, Square::from_index(to2_idx as usize).unwrap(), PieceType::Pawn));
                    }
                }
            }
            
            // Captures
            for offset in [-1, 1] {
                let target_file = from.file() as i32 + offset;
                if !(0..8).contains(&target_file) {
                    continue;
                }

                let capture_idx = from_sq as i32 + direction + offset;
                if (0..64).contains(&capture_idx) {
                    let capture_sq = Square::from_index(capture_idx as usize).unwrap();
                    let capture_piece = state.piece_at(capture_sq);
                    
                    if let Some((capture_color, _)) = capture_piece {
                        if capture_color != color {
                            self.add_pawn_capture(from, capture_sq, color, state, moves);
                        }
                    }
                    
                    // En passant
                    if let Some(ep_sq) = state.en_passant_square {
                        if capture_idx as usize == ep_sq.index() && from.rank() == en_passant_rank {
                            let mut mv = Move::new(from, capture_sq, PieceType::Pawn);
                            mv.is_en_passant = true;
                            mv.capture = Some(PieceType::Pawn);
                            moves.push(mv);
                        }
                    }
                }
            }
        }
    }
    
    fn add_pawn_move(&self, from: Square, to: Square, color: Color, moves: &mut Vec<Move>) {
        let promotion_rank = if color == Color::White { 7 } else { 0 };
        
        if to.rank() == promotion_rank {
            // Add all promotion options
            for promo in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                let mut mv = Move::new(from, to, PieceType::Pawn);
                mv.promotion = Some(promo);
                moves.push(mv);
            }
        } else {
            moves.push(Move::new(from, to, PieceType::Pawn));
        }
    }
    
    fn add_pawn_capture(
        &self,
        from: Square,
        to: Square,
        color: Color,
        state: &GameState,
        moves: &mut Vec<Move>,
    ) {
        let promotion_rank = if color == Color::White { 7 } else { 0 };
        let captured_piece = state.piece_at(to).map(|(_, piece)| piece);

        if to.rank() == promotion_rank {
            for promo in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                let mut mv = Move::new(from, to, PieceType::Pawn);
                mv.capture = captured_piece;
                mv.promotion = Some(promo);
                moves.push(mv);
            }
        } else {
            let mut mv = Move::new(from, to, PieceType::Pawn);
            mv.capture = captured_piece;
            moves.push(mv);
        }
    }

    fn generate_knight_moves(&self, state: &GameState, color: Color, moves: &mut Vec<Move>) {
        let knights = if color == Color::White { state.white_knights } else { state.black_knights };
        let friendly = state.pieces_of_color(color);
        let knight_offsets = [
            (1i32, 2i32),
            (2, 1),
            (2, -1),
            (1, -2),
            (-1, -2),
            (-2, -1),
            (-2, 1),
            (-1, 2),
        ];
        
        for from_sq in knights.iter() {
            let from = Square::from_index(from_sq).unwrap();
            let file = from.file() as i32;
            let rank = from.rank() as i32;
            
            for &(df, dr) in &knight_offsets {
                let new_file = file + df;
                let new_rank = rank + dr;
                if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
                    let to_idx = (new_file + new_rank * 8) as usize;
                    let to_sq = Square::from_index(to_idx as usize).unwrap();
                    if !friendly.has_piece(to_idx) {
                        let mut mv = Move::new(from, to_sq, PieceType::Knight);
                        if let Some((_, piece)) = state.piece_at(to_sq) {
                            mv.capture = Some(piece);
                        }
                        moves.push(mv);
                    }
                }
            }
        }
    }
    
    fn generate_sliding_moves(&self, state: &GameState, color: Color, 
                              piece_type: PieceType, moves: &mut Vec<Move>) {
        let (pieces, is_rook, is_bishop) = match piece_type {
            PieceType::Bishop => {
                let pieces = if color == Color::White { state.white_bishops } else { state.black_bishops };
                (pieces, false, true)
            },
            PieceType::Rook => {
                let pieces = if color == Color::White { state.white_rooks } else { state.black_rooks };
                (pieces, true, false)
            },
            PieceType::Queen => {
                let pieces = if color == Color::White { state.white_queens } else { state.black_queens };
                (pieces, true, true)
            },
            _ => return,
        };
        
        let friendly = state.pieces_of_color(color);
        let occupied = state.all_pieces();
        
        for from_sq in pieces.iter() {
            let from = Square::from_index(from_sq).unwrap();
            let attacks = if is_rook && is_bishop {
                self.magic_tables.queen_attacks(from, occupied)
            } else if is_rook {
                self.magic_tables.rook_attacks(from, occupied)
            } else {
                self.magic_tables.bishop_attacks(from, occupied)
            };
            
            for to_sq in attacks.iter() {
                if !friendly.has_piece(to_sq) {
                    let mut mv = Move::new(from, Square::from_index(to_sq).unwrap(), piece_type);
                    if let Some((_, piece)) = state.piece_at(Square::from_index(to_sq).unwrap()) {
                        mv.capture = Some(piece);
                    }
                    moves.push(mv);
                }
            }
        }
    }
    
    fn generate_bishop_moves(&self, state: &GameState, color: Color, moves: &mut Vec<Move>) {
        self.generate_sliding_moves(state, color, PieceType::Bishop, moves);
    }
    
    fn generate_rook_moves(&self, state: &GameState, color: Color, moves: &mut Vec<Move>) {
        self.generate_sliding_moves(state, color, PieceType::Rook, moves);
    }
    
    fn generate_queen_moves(&self, state: &GameState, color: Color, moves: &mut Vec<Move>) {
        self.generate_sliding_moves(state, color, PieceType::Queen, moves);
    }
    
    fn generate_king_moves(&self, state: &GameState, color: Color, moves: &mut Vec<Move>) {
        let king = if color == Color::White { state.white_king } else { state.black_king };
        let friendly = state.pieces_of_color(color);
        let king_offsets = [
            (-1i32, -1i32),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        
        for from_sq in king.iter() {
            let from = Square::from_index(from_sq).unwrap();
            let file = from.file() as i32;
            let rank = from.rank() as i32;
            
            for &(df, dr) in &king_offsets {
                let new_file = file + df;
                let new_rank = rank + dr;
                if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
                    let to_idx = (new_file + new_rank * 8) as usize;
                    let to_sq = Square::from_index(to_idx).unwrap();
                    if !friendly.has_piece(to_idx) {
                        let mut mv = Move::new(from, to_sq, PieceType::King);
                        if let Some((_, piece)) = state.piece_at(to_sq) {
                            mv.capture = Some(piece);
                        }
                        moves.push(mv);
                    }
                }
            }
            
            // Castling
            self.add_castling_moves(state, color, from, moves);
        }
    }
    
    fn add_castling_moves(&self, state: &GameState, color: Color, 
                          king_sq: Square, moves: &mut Vec<Move>) {
        let back_rank = if color == Color::White { 0 } else { 7 };
        let (kingside, queenside) = match color {
            Color::White => (state.castling_kingside_white, state.castling_queenside_white),
            Color::Black => (state.castling_kingside_black, state.castling_queenside_black),
        };
        
        if kingside {
            let f1 = Square::from_index(5 + back_rank * 8).unwrap();
            let g1 = Square::from_index(6 + back_rank * 8).unwrap();
            let h1 = Square::from_index(7 + back_rank * 8).unwrap();
            
            if !state.all_pieces().has_piece(f1.index()) && 
               !state.all_pieces().has_piece(g1.index()) &&
               matches!(state.piece_at(h1), Some((rook_color, PieceType::Rook)) if rook_color == color) &&
               !state.is_square_attacked(f1, color.opposite()) &&
               !state.is_square_attacked(g1, color.opposite()) &&
               !state.is_square_attacked(king_sq, color.opposite()) {
                let mut mv = Move::new(king_sq, g1, PieceType::King);
                mv.is_castle = true;
                moves.push(mv);
            }
        }
        
        if queenside {
            let b1 = Square::from_index(1 + back_rank * 8).unwrap();
            let c1 = Square::from_index(2 + back_rank * 8).unwrap();
            let d1 = Square::from_index(3 + back_rank * 8).unwrap();
            
            if !state.all_pieces().has_piece(b1.index()) && 
               !state.all_pieces().has_piece(c1.index()) &&
               !state.all_pieces().has_piece(d1.index()) &&
               matches!(state.piece_at(Square::from_index(back_rank * 8).unwrap()), Some((rook_color, PieceType::Rook)) if rook_color == color) &&
               !state.is_square_attacked(c1, color.opposite()) &&
               !state.is_square_attacked(d1, color.opposite()) &&
               !state.is_square_attacked(king_sq, color.opposite()) {
                let mut mv = Move::new(king_sq, c1, PieceType::King);
                mv.is_castle = true;
                moves.push(mv);
            }
        }
    }
    
    fn is_legal_move(&self, state: &GameState, mv: &Move) -> bool {
        // Try making the move and see if king is in check
        match GameState::make_move(state, mv) {
            Ok(new_state) => {
                let moved_side = state.side_to_move;
                !new_state.is_square_attacked(new_state.king_square(moved_side), moved_side.opposite())
            }
            Err(_) => false,
        }
    }
}
