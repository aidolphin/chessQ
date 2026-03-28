// src/ai/search.rs
use crate::ai::evaluation::{Evaluator, Personality};
use crate::engine::move_gen::{Move, MoveGenerator, PieceType};
use crate::state::game_state::GameState;
use std::time::{Duration, Instant};

const SEARCH_INF: i32 = 1_000_000_000;

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
}

impl AI {
    pub fn new(personality: Personality) -> Self {
        Self {
            move_gen: MoveGenerator::new(),
            evaluator: Evaluator::new(personality),
        }
    }
    
    /// Find best move for given position
    pub fn find_best_move(&mut self, state: &GameState, time_limit: Duration) -> SearchResult {
        let start_time = Instant::now();
        let mut best_move = None;
        let mut best_eval = -SEARCH_INF;
        let mut nodes_searched = 0;
        let mut depth_reached = 0;
        
        // Iterative deepening
        for depth in 1..=6 {
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
        
        if depth == 0 {
            let eval = self.evaluator.evaluate(state);
            return (None, eval, 1);
        }
        
        let moves = self.move_gen.generate_moves(state);
        if moves.is_empty() {
            if state.is_check() {
                return (None, -SEARCH_INF + 1000, 1); // Checkmate
            } else {
                return (None, 0, 1); // Stalemate
            }
        }
        
        // Order moves for better pruning
        let ordered_moves = self.order_moves(state, moves);
        
        let mut best_move = None;
        let mut nodes_searched = 0;
        
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
                            break; // Beta cutoff
                        }
                    }
                }
                Err(_) => continue,
            }
        }
        
        (best_move, alpha, nodes_searched)
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
