// src/ai/evaluation.rs
use crate::engine::move_gen::Color;
use crate::state::game_state::GameState;

/// Piece values in centipawns
const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
/// Piece-square tables for positional evaluation
const PAWN_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10,-20,-20, 10, 10,  5,
    2,  5,  8, 12, 12,  8,  5,  2,
    0,  2,  4,  8,  8,  4,  2,  0,
    0,  0,  2,  4,  4,  2,  0,  0,
    0,  0,  1,  2,  2,  1,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,
];

const KNIGHT_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const BISHOP_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

const ROOK_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0,
];

const QUEEN_TABLE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -5,  0,  5,  5,  5,  5,  0, -5,
    0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];

const KING_MIDDLE_TABLE: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    20, 20,  0,  0,  0,  0, 20, 20,
    20, 30, 10,  0,  0, 10, 30, 20,
];

const KING_ENDGAME_TABLE: [i32; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50,
];

/// Personality weights for different play styles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Personality {
    Aggressive,
    Defensive,
    Chaotic,
}

impl Personality {
    fn material_weight(&self) -> f32 {
        match self {
            Personality::Aggressive => 0.8,
            Personality::Defensive => 1.2,
            Personality::Chaotic => 1.0,
        }
    }
    
    fn position_weight(&self) -> f32 {
        match self {
            Personality::Aggressive => 1.2,
            Personality::Defensive => 0.8,
            Personality::Chaotic => 1.0,
        }
    }
    
    fn king_safety_weight(&self) -> f32 {
        match self {
            Personality::Aggressive => 0.7,
            Personality::Defensive => 1.3,
            Personality::Chaotic => 1.0,
        }
    }
    
    fn chaos_factor(&self) -> f32 {
        match self {
            Personality::Chaotic => 0.3,
            _ => 0.0,
        }
    }
}

/// Evaluator for chess positions
pub struct Evaluator {
    personality: Personality,
}

impl Evaluator {
    pub fn new(personality: Personality) -> Self {
        Self { personality }
    }
    
    /// Evaluate position from perspective of current player
    pub fn evaluate(&mut self, state: &GameState) -> i32 {
        let mut score = 0;
        
        // Material evaluation
        let material = self.evaluate_material(state);
        score += (material as f32 * self.personality.material_weight()) as i32;
        
        // Positional evaluation
        let positional = self.evaluate_positional(state);
        score += (positional as f32 * self.personality.position_weight()) as i32;
        
        // King safety
        let king_safety = self.evaluate_king_safety(state);
        score += (king_safety as f32 * self.personality.king_safety_weight()) as i32;
        
        // Mobility
        let mobility = self.evaluate_mobility(state);
        score += mobility;
        
        // Pawn structure
        let pawn_structure = self.evaluate_pawn_structure(state);
        score += pawn_structure;
        
        // Add chaos if personality is Chaotic
        if self.personality == Personality::Chaotic {
            let swing = (self.personality.chaos_factor() * 100.0) as i32;
            let chaos = (rand::random::<f32>() * (swing * 2) as f32) as i32 - swing;
            score += chaos;
        }
        
        // Return score from perspective of side to move
        if state.side_to_move == Color::White {
            score
        } else {
            -score
        }
    }
    
    fn evaluate_material(&self, state: &GameState) -> i32 {
        let white_material = 
            state.white_pawns.count() as i32 * PAWN_VALUE +
            state.white_knights.count() as i32 * KNIGHT_VALUE +
            state.white_bishops.count() as i32 * BISHOP_VALUE +
            state.white_rooks.count() as i32 * ROOK_VALUE +
            state.white_queens.count() as i32 * QUEEN_VALUE;
            
        let black_material = 
            state.black_pawns.count() as i32 * PAWN_VALUE +
            state.black_knights.count() as i32 * KNIGHT_VALUE +
            state.black_bishops.count() as i32 * BISHOP_VALUE +
            state.black_rooks.count() as i32 * ROOK_VALUE +
            state.black_queens.count() as i32 * QUEEN_VALUE;
            
        white_material - black_material
    }
    
    fn evaluate_positional(&self, state: &GameState) -> i32 {
        let mut white_score = 0;
        let mut black_score = 0;
        
        // Pawns
        for square in state.white_pawns.iter() {
            white_score += PAWN_TABLE[square];
        }
        for square in state.black_pawns.iter() {
            black_score += PAWN_TABLE[63 - square];
        }
        
        // Knights
        for square in state.white_knights.iter() {
            white_score += KNIGHT_TABLE[square];
        }
        for square in state.black_knights.iter() {
            black_score += KNIGHT_TABLE[63 - square];
        }
        
        // Bishops
        for square in state.white_bishops.iter() {
            white_score += BISHOP_TABLE[square];
        }
        for square in state.black_bishops.iter() {
            black_score += BISHOP_TABLE[63 - square];
        }
        
        // Rooks
        for square in state.white_rooks.iter() {
            white_score += ROOK_TABLE[square];
        }
        for square in state.black_rooks.iter() {
            black_score += ROOK_TABLE[63 - square];
        }
        
        // Queens
        for square in state.white_queens.iter() {
            white_score += QUEEN_TABLE[square];
        }
        for square in state.black_queens.iter() {
            black_score += QUEEN_TABLE[63 - square];
        }
        
        // Kings (use endgame table if material is low)
        let total_material = self.evaluate_material(state).abs();
        let king_table = if total_material < 1500 {
            KING_ENDGAME_TABLE
        } else {
            KING_MIDDLE_TABLE
        };
        
        for square in state.white_king.iter() {
            white_score += king_table[square];
        }
        for square in state.black_king.iter() {
            black_score += king_table[63 - square];
        }
        
        white_score - black_score
    }
    
    fn evaluate_king_safety(&self, state: &GameState) -> i32 {
        let mut white_safety = 0;
        let mut black_safety = 0;
        
        // Pawn shield for king
        let white_pawn_shield = self.pawn_shield_score(state, Color::White);
        let black_pawn_shield = self.pawn_shield_score(state, Color::Black);
        
        white_safety += white_pawn_shield * 10;
        black_safety += black_pawn_shield * 10;
        
        // King exposure (penalty for being on open files)
        let white_exposure = self.king_exposure(state, Color::White);
        let black_exposure = self.king_exposure(state, Color::Black);
        
        white_safety -= white_exposure * 5;
        black_safety -= black_exposure * 5;
        
        white_safety - black_safety
    }
    
    fn pawn_shield_score(&self, state: &GameState, color: Color) -> i32 {
        let king_sq = state.king_square(color);
        let king_file = king_sq.file() as i32;
        let king_rank = king_sq.rank() as i32;
        
        let pawns = if color == Color::White { state.white_pawns } else { state.black_pawns };
        let direction = if color == Color::White { 1 } else { -1 };
        
        let mut score = 0;
        
        // Check squares in front of king
        for file_offset in -1..=1 {
            let file = king_file + file_offset;
            if file >= 0 && file < 8 {
                let front_rank = (king_rank + direction) as usize;
                let front_sq = file as usize + front_rank * 8;
                if pawns.has_piece(front_sq) {
                    score += 10;
                }
            }
        }
        
        score
    }
    
    fn king_exposure(&self, state: &GameState, color: Color) -> i32 {
        let king_sq = state.king_square(color);
        let king_file = king_sq.file();
        
        // Check if king is on open file (no pawns on that file)
        let pawns = if color == Color::White { state.white_pawns } else { state.black_pawns };
        let mut file_has_pawn = false;
        
        for rank in 0..8 {
            let sq = king_file + rank * 8;
            if pawns.has_piece(sq) {
                file_has_pawn = true;
                break;
            }
        }
        
        if !file_has_pawn {
            5 // Exposure penalty
        } else {
            0
        }
    }
    
    fn evaluate_mobility(&self, _state: &GameState) -> i32 {
        // Simplified mobility: count number of legal moves
        // In a full implementation, this would use the move generator
        // For now, return a placeholder
        0
    }
    
    fn evaluate_pawn_structure(&self, state: &GameState) -> i32 {
        let mut white_score = 0;
        let mut black_score = 0;
        
        // Detect doubled pawns
        for file in 0..8 {
            let mut white_count = 0;
            let mut black_count = 0;
            
            for rank in 0..8 {
                let sq = file + rank * 8;
                if state.white_pawns.has_piece(sq) {
                    white_count += 1;
                }
                if state.black_pawns.has_piece(sq) {
                    black_count += 1;
                }
            }
            
            if white_count > 1 {
                white_score -= (white_count - 1) * 20;
            }
            if black_count > 1 {
                black_score -= (black_count - 1) * 20;
            }
        }
        
        // Detect isolated pawns
        for file in 0..8 {
            let has_pawn_white = (0..8).any(|rank| state.white_pawns.has_piece(file + rank * 8));
            let has_pawn_black = (0..8).any(|rank| state.black_pawns.has_piece(file + rank * 8));
            
            let has_adjacent_white = (file > 0 && (0..8).any(|rank| state.white_pawns.has_piece(file - 1 + rank * 8))) ||
                                     (file < 7 && (0..8).any(|rank| state.white_pawns.has_piece(file + 1 + rank * 8)));
            
            let has_adjacent_black = (file > 0 && (0..8).any(|rank| state.black_pawns.has_piece(file - 1 + rank * 8))) ||
                                     (file < 7 && (0..8).any(|rank| state.black_pawns.has_piece(file + 1 + rank * 8)));
            
            if has_pawn_white && !has_adjacent_white {
                white_score -= 30;
            }
            if has_pawn_black && !has_adjacent_black {
                black_score -= 30;
            }
        }
        
        white_score - black_score
    }
}
