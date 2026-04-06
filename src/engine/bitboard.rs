// src/engine/bitboard.rs
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

/// Represents a 64-bit chessboard where each bit corresponds to a square
/// Square mapping: a1=0, b1=1, ..., h8=63
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Self = Self(0);
    
    /// Create a bitboard from a specific square index
    pub fn from_square(square: usize) -> Self {
        debug_assert!(square < 64);
        Self(1u64 << square)
    }
    
    /// Check if bitboard has a piece at given square
    pub fn has_piece(&self, square: usize) -> bool {
        debug_assert!(square < 64);
        (self.0 >> square) & 1 == 1
    }
    
    /// Count number of set bits (popcount)
    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    /// Check whether the bitboard contains no set bits.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    
    /// Get least significant bit index
    pub fn lsb(&self) -> Option<usize> {
        if self.0 == 0 {
            None
        } else {
            Some(self.0.trailing_zeros() as usize)
        }
    }
    
    /// Iterate over set bits
    pub fn iter(&self) -> BitboardIterator {
        BitboardIterator { bits: self.0 }
    }
    
    /// Set a specific square
    pub fn set_square(&self, square: usize) -> Self {
        Self(self.0 | (1u64 << square))
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl Shl<usize> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self {
        Self(self.0 << rhs)
    }
}

impl Shr<usize> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self {
        Self(self.0 >> rhs)
    }
}

/// Iterator over set bits in a bitboard
pub struct BitboardIterator {
    bits: u64,
}

impl Iterator for BitboardIterator {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            None
        } else {
            let lsb = self.bits.trailing_zeros() as usize;
            self.bits &= self.bits - 1;
            Some(lsb)
        }
    }
}

/// Square representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub const fn index(&self) -> usize {
        *self as usize
    }
    
    pub fn from_index(idx: usize) -> Option<Self> {
        if idx >= 64 {
            eprintln!("WARNING: Invalid square index {} (must be 0-63)", idx);
            return None;
        }
        // Safe: transmute is safe here because we're mapping valid indices
        Some(unsafe { std::mem::transmute(idx as u8) })
    }
    
    pub fn file(&self) -> usize {
        self.index() % 8
    }
    
    pub fn rank(&self) -> usize {
        self.index() / 8
    }
}
