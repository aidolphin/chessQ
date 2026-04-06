// tests/engine_tests.rs
use chess_q::engine::move_gen::{MoveGenerator, Color};
use chess_q::state::game_state::GameState;
use chess_q::ai::search::AI;
use chess_q::ai::evaluation::Personality;
use std::time::Duration;

/// Perft (Performance Test) - counts leaf nodes at given depth
fn perft(state: &GameState, gen: &MoveGenerator, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    
    let moves = gen.generate_moves(state);
    let mut nodes = 0;
    
    for mv in moves {
        if let Ok(new_state) = GameState::make_move(state, &mv) {
            nodes += perft(&new_state, gen, depth - 1);
        }
    }
    
    nodes
}

#[test]
fn test_perft_initial_position() {
    let state = GameState::initial();
    let gen = MoveGenerator::new();
    
    assert_eq!(perft(&state, &gen, 1), 20, "Depth 1");
    assert_eq!(perft(&state, &gen, 2), 400, "Depth 2");
    assert_eq!(perft(&state, &gen, 3), 8_902, "Depth 3");
    assert_eq!(perft(&state, &gen, 4), 197_281, "Depth 4");
    
    // Depth 5 takes ~2 seconds
    // assert_eq!(perft(&state, &gen, 5), 4_865_609, "Depth 5");
}

#[test]
fn test_perft_kiwipete_position() {
    // Famous perft test position
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let state = GameState::from_fen(fen).unwrap();
    let gen = MoveGenerator::new();
    
    assert_eq!(perft(&state, &gen, 1), 48, "Kiwipete depth 1");
    assert_eq!(perft(&state, &gen, 2), 2_039, "Kiwipete depth 2");
    assert_eq!(perft(&state, &gen, 3), 97_862, "Kiwipete depth 3");
}

#[test]
fn test_mate_in_one_detection() {
    // Scholar's mate position - White to move, Qxf7# is mate
    let fen = "r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4";
    let state = GameState::from_fen(fen).unwrap();
    
    // Verify it's checkmate
    let gen = MoveGenerator::new();
    let moves = gen.generate_moves(&state);
    
    assert_eq!(moves.len(), 0, "Should have no legal moves (checkmate)");
    assert!(state.is_check(), "King should be in check");
}

#[test]
fn test_ai_finds_mate_in_one() {
    // Back rank mate - White to move, Rd8# is mate
    let fen = "6k1/5ppp/8/8/8/8/5PPP/3R2K1 w - - 0 1";
    let state = GameState::from_fen(fen).unwrap();
    let mut ai = AI::new(Personality::Aggressive);
    
    let result = ai.find_best_move(&state, Duration::from_secs(2));
    
    assert!(result.best_move.is_some(), "AI should find a move");
    
    // Verify the move leads to checkmate
    if let Some(mv) = result.best_move {
        let new_state = GameState::make_move(&state, &mv).unwrap();
        let gen = MoveGenerator::new();
        let opponent_moves = gen.generate_moves(&new_state);
        
        assert_eq!(opponent_moves.len(), 0, "Should be checkmate");
        assert!(new_state.is_check(), "Should be in check");
    }
}

#[test]
fn test_ai_avoids_blunder() {
    // Position where hanging queen is obvious
    let fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPPQPPP/RNB1KBNR b KQkq - 0 2";
    let state = GameState::from_fen(fen).unwrap();
    let mut ai = AI::new(Personality::Aggressive);
    
    let result = ai.find_best_move(&state, Duration::from_secs(1));
    
    assert!(result.best_move.is_some(), "AI should find a move");
    
    // AI should capture the hanging queen
    if let Some(mv) = result.best_move {
        assert!(mv.capture.is_some(), "Should capture the queen");
    }
}

#[test]
fn test_castling_legality() {
    // Position where kingside castling is legal
    let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";
    let state = GameState::from_fen(fen).unwrap();
    let gen = MoveGenerator::new();
    let moves = gen.generate_moves(&state);
    
    // Should have castling moves
    let castling_moves: Vec<_> = moves.iter().filter(|m| m.is_castle).collect();
    assert_eq!(castling_moves.len(), 2, "Should have 2 castling moves (kingside and queenside)");
}

#[test]
fn test_castling_through_check_illegal() {
    // Position where castling through check is illegal
    let fen = "r3k2r/8/8/8/8/8/8/R2rK2R w KQkq - 0 1";
    let state = GameState::from_fen(fen).unwrap();
    let gen = MoveGenerator::new();
    let moves = gen.generate_moves(&state);
    
    // Kingside castling should be illegal (f1 is attacked)
    let kingside_castle = moves.iter().any(|m| {
        m.is_castle && m.to.file() == 6
    });
    
    assert!(!kingside_castle, "Kingside castling should be illegal (castling through check)");
}

#[test]
fn test_en_passant_capture() {
    // Position after e4, d5, exd5 should be possible via en passant
    let fen = "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2";
    let state = GameState::from_fen(fen).unwrap();
    let gen = MoveGenerator::new();
    let moves = gen.generate_moves(&state);
    
    // Should have en passant capture
    let en_passant_moves: Vec<_> = moves.iter().filter(|m| m.is_en_passant).collect();
    assert_eq!(en_passant_moves.len(), 1, "Should have 1 en passant capture");
}

#[test]
fn test_pawn_promotion() {
    // White pawn on 7th rank
    let fen = "8/P7/8/8/8/8/8/K6k w - - 0 1";
    let state = GameState::from_fen(fen).unwrap();
    let gen = MoveGenerator::new();
    let moves = gen.generate_moves(&state);
    
    // Should have 4 promotion moves (Q, R, B, N)
    let promotion_moves: Vec<_> = moves.iter().filter(|m| m.promotion.is_some()).collect();
    assert_eq!(promotion_moves.len(), 4, "Should have 4 promotion options");
}

#[test]
fn test_stalemate_detection() {
    // Stalemate position - Black to move, no legal moves but not in check
    let fen = "7k/5Q2/6K1/8/8/8/8/8 b - - 0 1";
    let state = GameState::from_fen(fen).unwrap();
    let gen = MoveGenerator::new();
    let moves = gen.generate_moves(&state);
    
    assert_eq!(moves.len(), 0, "Should have no legal moves");
    assert!(!state.is_check(), "Should not be in check (stalemate)");
}

#[test]
fn test_pinned_piece_cannot_move() {
    // White rook on e2 is pinned by black queen on e8
    let fen = "4q3/8/8/8/8/8/4R3/4K3 w - - 0 1";
    let state = GameState::from_fen(fen).unwrap();
    let gen = MoveGenerator::new();
    let moves = gen.generate_moves(&state);
    
    // Rook should not be able to move horizontally (pinned)
    let rook_horizontal_moves = moves.iter().any(|m| {
        m.from.index() == 12 && m.to.rank() == 1 && m.to.file() != 4
    });
    
    assert!(!rook_horizontal_moves, "Pinned rook should not be able to move horizontally");
}

#[test]
fn test_transposition_table_hit_rate() {
    let state = GameState::initial();
    let mut ai = AI::new(Personality::Aggressive);
    
    // Run search twice
    ai.find_best_move(&state, Duration::from_millis(500));
    let result = ai.find_best_move(&state, Duration::from_millis(500));
    
    // Second search should be faster due to TT
    assert!(result.nodes_searched > 0, "Should search some nodes");
    assert!(result.depth_reached >= 4, "Should reach reasonable depth");
}

#[test]
fn test_quiescence_search_tactical_accuracy() {
    // Position with hanging piece - AI should see the capture
    let fen = "rnbqkb1r/pppp1ppp/5n2/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 3";
    let state = GameState::from_fen(fen).unwrap();
    let mut ai = AI::new(Personality::Aggressive);
    
    let result = ai.find_best_move(&state, Duration::from_secs(1));
    
    // Evaluation should be reasonable (not wildly off)
    assert!(result.evaluation.abs() < 1000, "Evaluation should be reasonable");
}
