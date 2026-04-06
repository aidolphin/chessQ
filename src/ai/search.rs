// src/ai/search.rs
use crate::ai::evaluation::{Evaluator, Personality};
use crate::engine::move_gen::{Move, MoveGenerator, PieceType};
use crate::engine::transposition::{TranspositionTable, NodeType};
use crate::engine::zobrist::ZobristKeys;
use crate::state::game_state::GameState;
use std::time::{Duration, Instant};

const SEARCH_INF: i32 = 1_000_000_000;
const MATE_SCORE: i32 = 900_000;

/// Search result containing best move and evaluation
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub best_move: Option<Move>,
    pub evaluation: i32,
    pub nodes_searched: u64,
    pub time_used: Duration,
    pub depth_reached: u32,
}

/// AI opponent with search capabilities
pub struct AI {
    move_gen: MoveGenerator,
    evaluator: Evaluator,
    transposition_table: TranspositionTable,
    zobrist_keys: ZobristKeys,
}

impl AI {
    pub fn new(personality: Personality) -> Self {
        Self {
            move_gen: MoveGenerator::new(),
            evaluator: Evaluator::new(personality),
            transposition_table: TranspositionTable::new(1_000_000), // 1M entries
            zobrist_keys: ZobristKeys::init(),
        }
    }
    
    /// Find best move for given position
    pub fn find_best_move(&mut self, state: &GameState, time_limit: Duration) -> SearchResult {
        let start_time = Instant::now();
        let mut best_move = None;
        let mut best_eval = -SEARCH_INF;
        let mut nodes_searched = 0;
        let mut depth_reached = 0;
        
        // Clear transposition table for new search
        self.transposition_table.clear();
        
        // Iterative deepening
        for depth in 1..=10 {
            let (mv, eval, nodes) =
                self.alpha_beta(state, depth, -SEARCH_INF, SEARCH_INF, start_time, time_limit);
            nodes_searched += nodes;
            
            if mv.is_some() {
                best_move = mv;
                best_eval = eval;
                depth_reached = depth;
            }
            
            // Check time limit
            if start_time.elapsed() >= time_limit {
                break;
            }
            
            // Stop if we found mate
            if eval.abs() > MATE_SCORE - 1000 {
                break;
            }
        }
        
        SearchResult {
            best_move,
            evaluation: best_eval,
            nodes_searched,
            time_used: start_time.elapsed(),
            depth_reached,
        }
    }
    
    fn alpha_beta(&mut self, state: &GameState, depth: u32, mut alpha: i32, beta: i32,
                  start_time: Instant, time_limit: Duration) -> (Option<Move>, i32, u64) {
        if start_time.elapsed() >= time_limit {
            return (None, 0, 0);
        }
        
        // Probe transposition table
        let hash = self.zobrist_keys.hash_position(state);
        if let Some((tt_score, tt_move)) = self.transposition_table.probe(hash, depth, alpha, beta) {
            if depth > 0 && tt_score != 0 {
                return (tt_move, tt_score, 1);
            }
        }
        
        // Quiescence search at leaf nodes
        if depth == 0 {
            let eval = self.quiescence_search(state, alpha, beta, 0);
            return (None, eval, 1);
        }
        
        let moves = self.move_gen.generate_moves(state);
        if moves.is_empty() {
            if state.is_check() {
                return (None, -MATE_SCORE + 1000, 1); // Checkmate
            } else {
                return (None, 0, 1); // Stalemate
            }
        }
        
        // Order moves for better pruning
        let ordered_moves = self.order_moves(state, moves);
        
        let mut best_move = None;
        let mut nodes_searched = 0;
        let original_alpha = alpha;
        
        for mv in ordered_moves.iter() {
            match GameState::make_move(state, mv) {
                Ok(new_state) => {
                    let (_, eval, nodes) = self.alpha_beta(&new_state, depth - 1, -beta, -alpha,
                                                           start_time, time_limit);
                    nodes_searched += nodes;
                    let current_eval = -eval;
                    
                    if current_eval > alpha {
                        alpha = current_eval;
                        best_move = Some(*mv);
                        
                        if alpha >= beta {
                            // Beta cutoff - store as lower bound
                            self.transposition_table.store(hash, depth, alpha, best_move, NodeType::LowerBound);
                            break;
                        }
                    }
                }
                Err(_) => continue,
            }
        }
        
        // Store in transposition table
        let node_type = if alpha <= original_alpha {
            NodeType::UpperBound // Failed low
        } else if alpha >= beta {
            NodeType::LowerBound // Failed high
        } else {
            NodeType::Exact // PV node
        };
        
        self.transposition_table.store(hash, depth, alpha, best_move, node_type);
        
        (best_move, alpha, nodes_searched)
    }
    
    /// Quiescence search to avoid horizon effect
    fn quiescence_search(&mut self, state: &GameState, mut alpha: i32, beta: i32, depth: u32) -> i32 {
        // Limit quiescence depth
        if depth > 8 {
            return self.evaluator.evaluate(state);
        }
        
        let stand_pat = self.evaluator.evaluate(state);
        
        // Beta cutoff
        if stand_pat >= beta {
            return beta;
        }
        
        // Update alpha
        if stand_pat > alpha {
            alpha = stand_pat;
        }
        
        // Generate only captures
        let moves = self.move_gen.generate_moves(state);
        let captures: Vec<Move> = moves.into_iter()
            .filter(|mv| mv.capture.is_some() || mv.promotion.is_some())
            .collect();
        
        // Order captures by MVV-LVA
        let ordered_captures = self.order_moves(state, captures);
        
        for mv in ordered_captures {
            match GameState::make_move(state, &mv) {
                Ok(new_state) => {
                    let score = -self.quiescence_search(&new_state, -beta, -alpha, depth + 1);
                    
                    if score >= beta {
                        return beta;
                    }
                    if score > alpha {
                        alpha = score;
                    }
                }
                Err(_) => continue,
            }
        }
        
        alpha
    }
    
    fn order_moves(&self, state: &GameState, moves: Vec<Move>) -> Vec<Move> {
        let mut moves_with_score: Vec<(Move, i32)> = moves.into_iter()
            .map(|mv| {
                let score = self.score_move(state, &mv);
                (mv, score)
            })
            .collect();
        
        moves_with_score.sort_by(|a, b| b.1.cmp(&a.1));
        moves_with_score.into_iter().map(|(mv, _)| mv).collect()
    }
    
    fn score_move(&self, state: &GameState, mv: &Move) -> i32 {
        let mut score = 0;
        
        // MVV-LVA (Most Valuable Victim - Least Valuable Aggressor)
        if let Some((_, captured)) = state.piece_at(mv.to) {
            let victim_value = match captured {
                PieceType::Queen => 900,
                PieceType::Rook => 500,
                PieceType::Bishop => 330,
                PieceType::Knight => 320,
                PieceType::Pawn => 100,
                PieceType::King => 20000,
            };
            
            let attacker_value = match mv.piece {
                PieceType::Queen => 900,
                PieceType::Rook => 500,
                PieceType::Bishop => 330,
                PieceType::Knight => 320,
                PieceType::Pawn => 100,
                PieceType::King => 20000,
            };
            
            score += victim_value - attacker_value;
        }
        
        // Promotion bonus
        if mv.promotion.is_some() {
            score += 800;
        }
        
        score
    }
}
