// src/state/game_state.rs
use crate::engine::bitboard::{Bitboard, Square};
use crate::engine::move_gen::{Move, PieceType, Color};

/// Represents the complete game state
#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    // Piece bitboards
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_rooks: Bitboard,
    pub white_queens: Bitboard,
    pub white_king: Bitboard,
    
    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_rooks: Bitboard,
    pub black_queens: Bitboard,
    pub black_king: Bitboard,
    
    // Game state
    pub side_to_move: Color,
    pub en_passant_square: Option<Square>,
    pub castling_kingside_white: bool,
    pub castling_queenside_white: bool,
    pub castling_kingside_black: bool,
    pub castling_queenside_black: bool,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl GameState {
    /// Create initial board position
    pub fn initial() -> Self {
        Self {
            // White pieces on rank 1 (indices 0-7) - BACK RANK
            white_pawns: Bitboard(0x000000000000FF00),  // Rank 2 (indices 8-15)
            white_knights: Bitboard(0x42),               // b1, g1 (indices 1, 6)
            white_bishops: Bitboard(0x24),               // c1, f1 (indices 2, 5)
            white_rooks: Bitboard(0x81),                  // a1, h1 (indices 0, 7)
            white_queens: Bitboard(0x08),                 // d1 (index 3)
            white_king: Bitboard(0x10),                   // e1 (index 4)
            
            // Black pieces on rank 8 (indices 56-63) - BACK RANK
            black_pawns: Bitboard(0x00FF000000000000),    // Rank 7 (indices 48-55)
            black_knights: Bitboard(0x4200000000000000),  // b8, g8 (indices 57, 62)
            black_bishops: Bitboard(0x2400000000000000),  // c8, f8 (indices 58, 61)
            black_rooks: Bitboard(0x8100000000000000),    // a8, h8 (indices 56, 63)
            black_queens: Bitboard(0x0800000000000000),   // d8 (index 59)
            black_king: Bitboard(0x1000000000000000),     // e8 (index 60)
            
            side_to_move: Color::White,
            en_passant_square: None,
            castling_kingside_white: true,
            castling_queenside_white: true,
            castling_kingside_black: true,
            castling_queenside_black: true,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
    
    /// Parse FEN string to create game state
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            return Err("Invalid FEN".to_string());
        }
        
        let mut state = Self::empty();
        state.parse_board(parts[0])?;
        state.parse_side_to_move(parts[1])?;
        state.parse_castling(parts[2])?;
        state.parse_en_passant(parts[3])?;
        
        if parts.len() > 4 {
            state.halfmove_clock = parts[4].parse().map_err(|_| "Invalid halfmove clock")?;
        }
        if parts.len() > 5 {
            state.fullmove_number = parts[5].parse().map_err(|_| "Invalid fullmove number")?;
        }
        
        Ok(state)
    }

    /// Serialize the current state to FEN.
    pub fn to_fen(&self) -> String {
        let mut rows = Vec::with_capacity(8);

        for rank in (0..8).rev() {
            let mut empty = 0usize;
            let mut row = String::new();

            for file in 0..8 {
                let square = Square::from_index(file + rank * 8).unwrap();
                match self.piece_at(square) {
                    Some((color, piece)) => {
                        if empty > 0 {
                            row.push_str(&empty.to_string());
                            empty = 0;
                        }
                        row.push(piece_to_fen_char(color, piece));
                    }
                    None => empty += 1,
                }
            }

            if empty > 0 {
                row.push_str(&empty.to_string());
            }

            rows.push(row);
        }

        let mut castling = String::new();
        if self.castling_kingside_white {
            castling.push('K');
        }
        if self.castling_queenside_white {
            castling.push('Q');
        }
        if self.castling_kingside_black {
            castling.push('k');
        }
        if self.castling_queenside_black {
            castling.push('q');
        }
        if castling.is_empty() {
            castling.push('-');
        }

        let en_passant = self
            .en_passant_square
            .map(square_to_coord)
            .unwrap_or_else(|| "-".to_string());

        format!(
            "{} {} {} {} {} {}",
            rows.join("/"),
            if self.side_to_move == Color::White { "w" } else { "b" },
            castling,
            en_passant,
            self.halfmove_clock,
            self.fullmove_number
        )
    }
    
    fn empty() -> Self {
        Self {
            white_pawns: Bitboard::EMPTY,
            white_knights: Bitboard::EMPTY,
            white_bishops: Bitboard::EMPTY,
            white_rooks: Bitboard::EMPTY,
            white_queens: Bitboard::EMPTY,
            white_king: Bitboard::EMPTY,
            black_pawns: Bitboard::EMPTY,
            black_knights: Bitboard::EMPTY,
            black_bishops: Bitboard::EMPTY,
            black_rooks: Bitboard::EMPTY,
            black_queens: Bitboard::EMPTY,
            black_king: Bitboard::EMPTY,
            side_to_move: Color::White,
            en_passant_square: None,
            castling_kingside_white: false,
            castling_queenside_white: false,
            castling_kingside_black: false,
            castling_queenside_black: false,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
    
    fn parse_board(&mut self, board_str: &str) -> Result<(), String> {
        let mut rank = 7;
        let mut file = 0;
        
        for ch in board_str.chars() {
            if ch == '/' {
                rank -= 1;
                file = 0;
            } else if let Some(num) = ch.to_digit(10) {
                file += num as usize;
            } else {
                let square = file + rank * 8;
                let bit = Bitboard::from_square(square);
                
                match ch {
                    'P' => self.white_pawns = self.white_pawns | bit,
                    'N' => self.white_knights = self.white_knights | bit,
                    'B' => self.white_bishops = self.white_bishops | bit,
                    'R' => self.white_rooks = self.white_rooks | bit,
                    'Q' => self.white_queens = self.white_queens | bit,
                    'K' => self.white_king = self.white_king | bit,
                    'p' => self.black_pawns = self.black_pawns | bit,
                    'n' => self.black_knights = self.black_knights | bit,
                    'b' => self.black_bishops = self.black_bishops | bit,
                    'r' => self.black_rooks = self.black_rooks | bit,
                    'q' => self.black_queens = self.black_queens | bit,
                    'k' => self.black_king = self.black_king | bit,
                    _ => return Err(format!("Invalid piece character: {}", ch)),
                }
                file += 1;
            }
        }
        
        Ok(())
    }
    
    fn parse_side_to_move(&mut self, side_str: &str) -> Result<(), String> {
        self.side_to_move = match side_str {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid side to move".to_string()),
        };
        Ok(())
    }
    
    fn parse_castling(&mut self, castling_str: &str) -> Result<(), String> {
        if castling_str == "-" {
            return Ok(());
        }
        
        self.castling_kingside_white = castling_str.contains('K');
        self.castling_queenside_white = castling_str.contains('Q');
        self.castling_kingside_black = castling_str.contains('k');
        self.castling_queenside_black = castling_str.contains('q');
        
        Ok(())
    }
    
    fn parse_en_passant(&mut self, ep_str: &str) -> Result<(), String> {
        if ep_str == "-" {
            self.en_passant_square = None;
            return Ok(());
        }
        
        let chars: Vec<char> = ep_str.chars().collect();
        if chars.len() != 2 {
            return Err("Invalid en passant square".to_string());
        }
        
        let file = match chars[0] {
            'a' => 0, 'b' => 1, 'c' => 2, 'd' => 3,
            'e' => 4, 'f' => 5, 'g' => 6, 'h' => 7,
            _ => return Err("Invalid file".to_string()),
        };
        
        let rank = match chars[1] {
            '1' => 0, '2' => 1, '3' => 2, '4' => 3,
            '5' => 4, '6' => 5, '7' => 6, '8' => 7,
            _ => return Err("Invalid rank".to_string()),
        };
        
        self.en_passant_square = Some(Square::from_index(file + rank * 8).unwrap());
        Ok(())
    }
    
    /// Make a move, returning new game state or error
    pub fn make_move(state: &GameState, mv: &Move) -> Result<GameState, String> {
        let mut new_state = state.clone();
        
        // Verify piece belongs to current player
        let piece_color = if state.piece_at(mv.from).is_some() {
            state.piece_at(mv.from).unwrap().0
        } else {
            return Err("No piece at from square".to_string());
        };
        
        if piece_color != state.side_to_move {
            return Err("Wrong side to move".to_string());
        }
        
        // Clear from square
        new_state = new_state.remove_piece(mv.from);
        
        // Handle capture
        if let Some((_, _piece)) = state.piece_at(mv.to) {
            new_state = new_state.remove_piece(mv.to);
        }
        
        // Handle en passant capture
        if mv.is_en_passant {
            let capture_square = if state.side_to_move == Color::White {
                Square::from_index(mv.to.index() - 8).unwrap()
            } else {
                Square::from_index(mv.to.index() + 8).unwrap()
            };
            new_state = new_state.remove_piece(capture_square);
        }
        
        // Add piece to new square
        new_state = new_state.add_piece(mv.to, state.side_to_move, mv.piece);
        
        // Handle promotion
        if let Some(promo_piece) = mv.promotion {
            new_state = new_state.remove_piece(mv.to);
            new_state = new_state.add_piece(mv.to, state.side_to_move, promo_piece);
        }
        
        // Handle castling (move rook)
        if mv.is_castle {
            let back_rank = if state.side_to_move == Color::White { 0 } else { 7 };
            let (rook_from, rook_to) = if mv.to.file() == 6 {
                // Kingside
                (Square::from_index(7 + back_rank * 8).unwrap(),
                 Square::from_index(5 + back_rank * 8).unwrap())
            } else {
                // Queenside
                (Square::from_index(0 + back_rank * 8).unwrap(),
                 Square::from_index(3 + back_rank * 8).unwrap())
            };
            
            new_state = new_state.remove_piece(rook_from);
            new_state = new_state.add_piece(rook_to, state.side_to_move, PieceType::Rook);
        }
        
        // Update castling rights
        new_state = new_state.update_castling_rights(mv);
        
        // Update en passant square
        new_state.en_passant_square = if mv.piece == PieceType::Pawn && 
                                         (mv.to.rank() as i32 - mv.from.rank() as i32).abs() == 2 {
            // The en passant square is the square the pawn passed through
            // For white (moving up ranks): the square one rank up from start
            // For black (moving down ranks): rank 5 (the only rank where black EP makes sense)
            let ep_rank = if state.side_to_move == Color::White { 2 } else { 5 };
            Some(Square::from_index(mv.from.file() + ep_rank * 8).unwrap())
        } else {
            None
        };
        
        // Update clocks
        new_state.halfmove_clock = if mv.capture.is_some() || mv.piece == PieceType::Pawn {
            0
        } else {
            state.halfmove_clock + 1
        };
        
        new_state.fullmove_number = if state.side_to_move == Color::Black {
            state.fullmove_number + 1
        } else {
            state.fullmove_number
        };
        
        // Change side to move
        new_state.side_to_move = state.side_to_move.opposite();
        
        Ok(new_state)
    }
    
    fn remove_piece(&self, square: Square) -> Self {
        let mut new = self.clone();
        let mask = !Bitboard::from_square(square.index());
        
        new.white_pawns = new.white_pawns & mask;
        new.white_knights = new.white_knights & mask;
        new.white_bishops = new.white_bishops & mask;
        new.white_rooks = new.white_rooks & mask;
        new.white_queens = new.white_queens & mask;
        new.white_king = new.white_king & mask;
        new.black_pawns = new.black_pawns & mask;
        new.black_knights = new.black_knights & mask;
        new.black_bishops = new.black_bishops & mask;
        new.black_rooks = new.black_rooks & mask;
        new.black_queens = new.black_queens & mask;
        new.black_king = new.black_king & mask;
        
        new
    }
    
    fn add_piece(&self, square: Square, color: Color, piece: PieceType) -> Self {
        let mut new = self.clone();
        let bit = Bitboard::from_square(square.index());
        
        match (color, piece) {
            (Color::White, PieceType::Pawn) => new.white_pawns = new.white_pawns | bit,
            (Color::White, PieceType::Knight) => new.white_knights = new.white_knights | bit,
            (Color::White, PieceType::Bishop) => new.white_bishops = new.white_bishops | bit,
            (Color::White, PieceType::Rook) => new.white_rooks = new.white_rooks | bit,
            (Color::White, PieceType::Queen) => new.white_queens = new.white_queens | bit,
            (Color::White, PieceType::King) => new.white_king = new.white_king | bit,
            (Color::Black, PieceType::Pawn) => new.black_pawns = new.black_pawns | bit,
            (Color::Black, PieceType::Knight) => new.black_knights = new.black_knights | bit,
            (Color::Black, PieceType::Bishop) => new.black_bishops = new.black_bishops | bit,
            (Color::Black, PieceType::Rook) => new.black_rooks = new.black_rooks | bit,
            (Color::Black, PieceType::Queen) => new.black_queens = new.black_queens | bit,
            (Color::Black, PieceType::King) => new.black_king = new.black_king | bit,
        }
        
        new
    }
    
    fn update_castling_rights(&self, mv: &Move) -> Self {
        let mut new = self.clone();
        
        // Moving king removes castling rights
        if mv.piece == PieceType::King {
            match self.side_to_move {
                Color::White => {
                    new.castling_kingside_white = false;
                    new.castling_queenside_white = false;
                }
                Color::Black => {
                    new.castling_kingside_black = false;
                    new.castling_queenside_black = false;
                }
            }
        }
        
        // Moving rook removes specific castling rights
        if mv.piece == PieceType::Rook {
            match (self.side_to_move, mv.from.file()) {
                (Color::White, 0) => new.castling_queenside_white = false,
                (Color::White, 7) => new.castling_kingside_white = false,
                (Color::Black, 0) => new.castling_queenside_black = false,
                (Color::Black, 7) => new.castling_kingside_black = false,
                _ => {}
            }
        }
        
        // Capturing rook removes opponent's castling rights
        if let Some((_, piece)) = self.piece_at(mv.to) {
            if piece == PieceType::Rook {
                match (self.side_to_move.opposite(), mv.to.file()) {
                    (Color::White, 0) => new.castling_queenside_white = false,
                    (Color::White, 7) => new.castling_kingside_white = false,
                    (Color::Black, 0) => new.castling_queenside_black = false,
                    (Color::Black, 7) => new.castling_kingside_black = false,
                    _ => {}
                }
            }
        }
        
        new
    }
    
    /// Get piece at square
    pub fn piece_at(&self, square: Square) -> Option<(Color, PieceType)> {
        let bit = Bitboard::from_square(square.index());
        
        if (self.white_pawns & bit).0 != 0 { return Some((Color::White, PieceType::Pawn)); }
        if (self.white_knights & bit).0 != 0 { return Some((Color::White, PieceType::Knight)); }
        if (self.white_bishops & bit).0 != 0 { return Some((Color::White, PieceType::Bishop)); }
        if (self.white_rooks & bit).0 != 0 { return Some((Color::White, PieceType::Rook)); }
        if (self.white_queens & bit).0 != 0 { return Some((Color::White, PieceType::Queen)); }
        if (self.white_king & bit).0 != 0 { return Some((Color::White, PieceType::King)); }
        if (self.black_pawns & bit).0 != 0 { return Some((Color::Black, PieceType::Pawn)); }
        if (self.black_knights & bit).0 != 0 { return Some((Color::Black, PieceType::Knight)); }
        if (self.black_bishops & bit).0 != 0 { return Some((Color::Black, PieceType::Bishop)); }
        if (self.black_rooks & bit).0 != 0 { return Some((Color::Black, PieceType::Rook)); }
        if (self.black_queens & bit).0 != 0 { return Some((Color::Black, PieceType::Queen)); }
        if (self.black_king & bit).0 != 0 { return Some((Color::Black, PieceType::King)); }
        
        None
    }
    
    /// Get all pieces of a color
    pub fn pieces_of_color(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white_pawns | self.white_knights | self.white_bishops |
                           self.white_rooks | self.white_queens | self.white_king,
            Color::Black => self.black_pawns | self.black_knights | self.black_bishops |
                           self.black_rooks | self.black_queens | self.black_king,
        }
    }
    
    /// Get all pieces on board
    pub fn all_pieces(&self) -> Bitboard {
        self.pieces_of_color(Color::White) | self.pieces_of_color(Color::Black)
    }
    
    /// Check if king is in check
    pub fn is_check(&self) -> bool {
        self.is_square_attacked(self.king_square(self.side_to_move), self.side_to_move.opposite())
    }
    
    /// Get king square for a color
    pub fn king_square(&self, color: Color) -> Square {
        let king_bitboard = match color {
            Color::White => self.white_king,
            Color::Black => self.black_king,
        };
        Square::from_index(king_bitboard.lsb().unwrap()).unwrap()
    }
    
    /// Check if a square is attacked by a color
    pub fn is_square_attacked(&self, square: Square, by_color: Color) -> bool {
        !self.attackers_to_square(square, by_color).is_empty()
    }
    
    fn attackers_to_square(&self, square: Square, color: Color) -> Bitboard {
        let mut attackers = Bitboard::EMPTY;
        let target_file = square.file() as i32;
        let target_rank = square.rank() as i32;

        let pawn_offsets = if color == Color::White {
            [(-1i32, -1i32), (1, -1)]
        } else {
            [(-1i32, 1i32), (1, 1)]
        };
        for (df, dr) in pawn_offsets {
            let file = target_file + df;
            let rank = target_rank + dr;
            if (0..8).contains(&file) && (0..8).contains(&rank) {
                let idx = (file + rank * 8) as usize;
                if matches!(self.piece_at(Square::from_index(idx).unwrap()), Some((piece_color, PieceType::Pawn)) if piece_color == color) {
                    attackers = attackers.set_square(idx);
                }
            }
        }

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
        for (df, dr) in knight_offsets {
            let file = target_file + df;
            let rank = target_rank + dr;
            if (0..8).contains(&file) && (0..8).contains(&rank) {
                let idx = (file + rank * 8) as usize;
                if matches!(self.piece_at(Square::from_index(idx).unwrap()), Some((piece_color, PieceType::Knight)) if piece_color == color) {
                    attackers = attackers.set_square(idx);
                }
            }
        }

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
        for (df, dr) in king_offsets {
            let file = target_file + df;
            let rank = target_rank + dr;
            if (0..8).contains(&file) && (0..8).contains(&rank) {
                let idx = (file + rank * 8) as usize;
                if matches!(self.piece_at(Square::from_index(idx).unwrap()), Some((piece_color, PieceType::King)) if piece_color == color) {
                    attackers = attackers.set_square(idx);
                }
            }
        }

        for (df, dr, rook_like, bishop_like) in [
            (1i32, 0i32, true, false),
            (-1, 0, true, false),
            (0, 1, true, false),
            (0, -1, true, false),
            (1, 1, false, true),
            (1, -1, false, true),
            (-1, 1, false, true),
            (-1, -1, false, true),
        ] {
            let mut file = target_file + df;
            let mut rank = target_rank + dr;
            while (0..8).contains(&file) && (0..8).contains(&rank) {
                let idx = (file + rank * 8) as usize;
                let from_sq = Square::from_index(idx).unwrap();
                if let Some((piece_color, piece_type)) = self.piece_at(from_sq) {
                    if piece_color == color {
                        let attacks_on_ray =
                            (rook_like && matches!(piece_type, PieceType::Rook | PieceType::Queen))
                                || (bishop_like && matches!(piece_type, PieceType::Bishop | PieceType::Queen));
                        if attacks_on_ray {
                            attackers = attackers.set_square(idx);
                        }
                    }
                    break;
                }
                file += df;
                rank += dr;
            }
        }
        
        attackers
    }
    
    /// Check if game is over
    pub fn is_game_over(&self, generator: &crate::engine::move_gen::MoveGenerator) -> bool {
        let moves = generator.generate_moves(self);
        if moves.is_empty() {
            return true; // Checkmate or stalemate
        }
        false
    }
    
    /// Get game result
    pub fn result(&self, generator: &crate::engine::move_gen::MoveGenerator) -> Option<String> {
        let moves = generator.generate_moves(self);
        if moves.is_empty() {
            if self.is_check() {
                Some(format!("{} wins by checkmate", 
                           if self.side_to_move == Color::White { "Black" } else { "White" }))
            } else {
                Some("Stalemate".to_string())
            }
        } else {
            None
        }
    }
}

fn piece_to_fen_char(color: Color, piece: PieceType) -> char {
    let symbol = match piece {
        PieceType::Pawn => 'p',
        PieceType::Knight => 'n',
        PieceType::Bishop => 'b',
        PieceType::Rook => 'r',
        PieceType::Queen => 'q',
        PieceType::King => 'k',
    };

    if color == Color::White {
        symbol.to_ascii_uppercase()
    } else {
        symbol
    }
}

fn square_to_coord(square: Square) -> String {
    let file = match square.file() {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => unreachable!(),
    };
    let rank = char::from_digit((square.rank() + 1) as u32, 10).unwrap();
    format!("{file}{rank}")
}
