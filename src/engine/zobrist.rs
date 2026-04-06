// src/engine/zobrist.rs
use crate::engine::bitboard::Square;
use crate::engine::move_gen::{Color, PieceType};
use crate::state::game_state::GameState;
use rand::Rng;

/// Zobrist hashing for position identification
pub struct ZobristKeys {
    pieces: [[u64; 64]; 12], // 6 piece types * 2 colors * 64 squares
    castling: [u64; 4],       // White KS, White QS, Black KS, Black QS
    en_passant: [u64; 8],     // Files a-h
    side_to_move: u64,
}

impl ZobristKeys {
    /// Initialize Zobrist keys with random values
    pub fn init() -> Self {
        let mut rng = rand::thread_rng();
        
        let mut pieces = [[0u64; 64]; 12];
        for piece_type in 0..12 {
            for square in 0..64 {
                pieces[piece_type][square] = rng.gen();
            }
        }
        
        let castling = [rng.gen(), rng.gen(), rng.gen(), rng.gen()];
        let en_passant = [
            rng.gen(), rng.gen(), rng.gen(), rng.gen(),
            rng.gen(), rng.gen(), rng.gen(), rng.gen(),
        ];
        let side_to_move = rng.gen();
        
        Self {
            pieces,
            castling,
            en_passant,
            side_to_move,
        }
    }
    
    /// Hash a complete position
    pub fn hash_position(&self, state: &GameState) -> u64 {
        let mut hash = 0u64;
        
        // Hash all pieces
        for square in 0..64 {
            if let Some((color, piece)) = state.piece_at(Square::from_index(square).unwrap()) {
                let piece_index = self.piece_to_index(color, piece);
                hash ^= self.pieces[piece_index][square];
            }
        }
        
        // Hash castling rights
        if state.castling_kingside_white {
            hash ^= self.castling[0];
        }
        if state.castling_queenside_white {
            hash ^= self.castling[1];
        }
        if state.castling_kingside_black {
            hash ^= self.castling[2];
        }
        if state.castling_queenside_black {
            hash ^= self.castling[3];
        }
        
        // Hash en passant square
        if let Some(ep_sq) = state.en_passant_square {
            hash ^= self.en_passant[ep_sq.file()];
        }
        
        // Hash side to move
        if state.side_to_move == Color::Black {
            hash ^= self.side_to_move;
        }
        
        hash
    }
    
    /// Convert piece to index (0-11)
    fn piece_to_index(&self, color: Color, piece: PieceType) -> usize {
        let color_offset = match color {
            Color::White => 0,
            Color::Black => 6,
        };
        
        let piece_offset = match piece {
            PieceType::Pawn => 0,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 4,
            PieceType::King => 5,
        };
        
        color_offset + piece_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_zobrist_deterministic() {
        let keys = ZobristKeys::init();
        let state = GameState::initial();
        
        let hash1 = keys.hash_position(&state);
        let hash2 = keys.hash_position(&state);
        
        assert_eq!(hash1, hash2, "Same position should have same hash");
    }
    
    #[test]
    fn test_zobrist_different_positions() {
        let keys = ZobristKeys::init();
        let state1 = GameState::initial();
        let state2 = GameState::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
        
        let hash1 = keys.hash_position(&state1);
        let hash2 = keys.hash_position(&state2);
        
        assert_ne!(hash1, hash2, "Different positions should have different hashes");
    }
}
