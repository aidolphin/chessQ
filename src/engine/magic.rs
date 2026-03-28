// src/engine/magic.rs
use crate::engine::bitboard::{Bitboard, Square};

/// Sliding attack helper. This keeps the existing interface without relying on
/// partially generated magic tables.
pub struct MagicTables;

impl MagicTables {
    /// Initialize magic bitboard tables
    pub fn init() -> Self {
        Self
    }
    
    /// Get rook attacks from a square with blockers
    pub fn rook_attacks(&self, square: Square, blockers: Bitboard) -> Bitboard {
        Self::compute_rook_attacks(square, blockers)
    }
    
    /// Get bishop attacks from a square with blockers
    pub fn bishop_attacks(&self, square: Square, blockers: Bitboard) -> Bitboard {
        Self::compute_bishop_attacks(square, blockers)
    }
    
    /// Get queen attacks (rook + bishop)
    pub fn queen_attacks(&self, square: Square, blockers: Bitboard) -> Bitboard {
        self.rook_attacks(square, blockers) | self.bishop_attacks(square, blockers)
    }
    
    fn compute_rook_attacks(square: Square, blockers: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let (file, rank) = (square.file() as i32, square.rank() as i32);
        
        // Positive X
        for x in (file + 1)..8 {
            let idx = (x + rank * 8) as usize;
            attacks = attacks.set_square(idx);
            if blockers.has_piece(idx) { break; }
        }
        
        // Negative X
        for x in (0..file).rev() {
            let idx = (x + rank * 8) as usize;
            attacks = attacks.set_square(idx);
            if blockers.has_piece(idx) { break; }
        }
        
        // Positive Y
        for y in (rank + 1)..8 {
            let idx = (file + y * 8) as usize;
            attacks = attacks.set_square(idx);
            if blockers.has_piece(idx) { break; }
        }
        
        // Negative Y
        for y in (0..rank).rev() {
            let idx = (file + y * 8) as usize;
            attacks = attacks.set_square(idx);
            if blockers.has_piece(idx) { break; }
        }
        
        attacks
    }
    
    fn compute_bishop_attacks(square: Square, blockers: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let (file, rank) = (square.file() as i32, square.rank() as i32);
        
        // Four diagonals
        for i in 1..8 {
            let x = file + i;
            let y = rank + i;
            if x >= 8 || y >= 8 { break; }
            let idx = (x + y * 8) as usize;
            attacks = attacks.set_square(idx);
            if blockers.has_piece(idx) { break; }
        }
        
        for i in 1..8 {
            let x = file - i;
            let y = rank + i;
            if x < 0 || y >= 8 { break; }
            let idx = (x + y * 8) as usize;
            attacks = attacks.set_square(idx);
            if blockers.has_piece(idx) { break; }
        }
        
        for i in 1..8 {
            let x = file + i;
            let y = rank - i;
            if x >= 8 || y < 0 { break; }
            let idx = (x + y * 8) as usize;
            attacks = attacks.set_square(idx);
            if blockers.has_piece(idx) { break; }
        }
        
        for i in 1..8 {
            let x = file - i;
            let y = rank - i;
            if x < 0 || y < 0 { break; }
            let idx = (x + y * 8) as usize;
            attacks = attacks.set_square(idx);
            if blockers.has_piece(idx) { break; }
        }
        
        attacks
    }
}
