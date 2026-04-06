// src/engine/transposition.rs
use crate::engine::move_gen::Move;
use std::collections::HashMap;

/// Type of node in the search tree
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Exact,      // PV node - exact score
    LowerBound, // Beta cutoff - score >= beta
    UpperBound, // Alpha failed - score <= alpha
}

/// Entry in the transposition table
#[derive(Debug, Clone)]
pub struct TranspositionEntry {
    pub zobrist_hash: u64,
    pub depth: u32,
    pub score: i32,
    pub best_move: Option<Move>,
    pub node_type: NodeType,
}

/// Transposition table for caching search results
pub struct TranspositionTable {
    table: HashMap<u64, TranspositionEntry>,
    max_size: usize,
    hits: u64,
    misses: u64,
}

impl TranspositionTable {
    /// Create new transposition table with given size
    pub fn new(max_size: usize) -> Self {
        Self {
            table: HashMap::with_capacity(max_size),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    
    /// Probe the transposition table
    pub fn probe(&mut self, hash: u64, depth: u32, alpha: i32, beta: i32) 
        -> Option<(i32, Option<Move>)> {
        if let Some(entry) = self.table.get(&hash) {
            self.hits += 1;
            
            // Only use entry if depth is sufficient
            if entry.depth >= depth {
                match entry.node_type {
                    NodeType::Exact => {
                        // Exact score - use it
                        return Some((entry.score, entry.best_move));
                    }
                    NodeType::LowerBound => {
                        // Score is at least this high
                        if entry.score >= beta {
                            return Some((entry.score, entry.best_move));
                        }
                    }
                    NodeType::UpperBound => {
                        // Score is at most this high
                        if entry.score <= alpha {
                            return Some((entry.score, entry.best_move));
                        }
                    }
                }
            }
            
            // Return best move for move ordering even if depth insufficient
            return Some((0, entry.best_move));
        }
        
        self.misses += 1;
        None
    }
    
    /// Store entry in transposition table
    pub fn store(&mut self, hash: u64, depth: u32, score: i32, 
                 best_move: Option<Move>, node_type: NodeType) {
        // Check if we need to replace
        if self.table.len() >= self.max_size {
            if let Some(entry) = self.table.get(&hash) {
                // Always replace scheme: replace if new depth is greater or equal
                if entry.depth > depth {
                    return; // Don't replace deeper entry
                }
            } else {
                // Table full, need to evict something
                // Simple strategy: remove first entry (could be improved)
                if let Some(&key) = self.table.keys().next() {
                    self.table.remove(&key);
                }
            }
        }
        
        self.table.insert(hash, TranspositionEntry {
            zobrist_hash: hash,
            depth,
            score,
            best_move,
            node_type,
        });
    }
    
    /// Clear the transposition table
    pub fn clear(&mut self) {
        self.table.clear();
        self.hits = 0;
        self.misses = 0;
    }
    
    /// Get statistics
    pub fn stats(&self) -> (u64, u64, f64) {
        let total = self.hits + self.misses;
        let hit_rate = if total > 0 {
            self.hits as f64 / total as f64
        } else {
            0.0
        };
        (self.hits, self.misses, hit_rate)
    }
    
    /// Get table size
    pub fn size(&self) -> usize {
        self.table.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::bitboard::Square;
    use crate::engine::move_gen::{Move, PieceType};
    
    #[test]
    fn test_transposition_table_store_probe() {
        let mut tt = TranspositionTable::new(1000);
        let hash = 12345u64;
        let mv = Move::new(
            Square::from_index(12).unwrap(),
            Square::from_index(20).unwrap(),
            PieceType::Pawn
        );
        
        // Store entry
        tt.store(hash, 5, 100, Some(mv), NodeType::Exact);
        
        // Probe should find it
        let result = tt.probe(hash, 5, -1000, 1000);
        assert!(result.is_some());
        
        let (score, best_move) = result.unwrap();
        assert_eq!(score, 100);
        assert!(best_move.is_some());
    }
    
    #[test]
    fn test_transposition_table_depth_check() {
        let mut tt = TranspositionTable::new(1000);
        let hash = 12345u64;
        
        // Store at depth 3
        tt.store(hash, 3, 100, None, NodeType::Exact);
        
        // Probe at depth 5 should not use score (insufficient depth)
        let result = tt.probe(hash, 5, -1000, 1000);
        assert!(result.is_some());
        let (score, _) = result.unwrap();
        assert_eq!(score, 0); // Returns 0 when depth insufficient
    }
    
    #[test]
    fn test_transposition_table_stats() {
        let mut tt = TranspositionTable::new(1000);
        
        tt.probe(123, 5, -1000, 1000); // Miss
        tt.store(123, 5, 100, None, NodeType::Exact);
        tt.probe(123, 5, -1000, 1000); // Hit
        
        let (hits, misses, hit_rate) = tt.stats();
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
        assert_eq!(hit_rate, 0.5);
    }
}
