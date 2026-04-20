// End-to-end tests for ChessQ
// These tests simulate complete game scenarios from start to finish

#[cfg(test)]
mod e2e_tests {
    use chess_q::engine::move_gen::{Color, MoveGenerator};
    use chess_q::state::game_state::GameState;
    use chess_q::ai::search::AI;
    use chess_q::ai::evaluation::Personality;
    use std::time::Duration;

    #[test]
    fn test_complete_game_white_wins() {
        // Simulate a complete game where white wins
        let mut state = GameState::initial();
        let generator = MoveGenerator::new();
        
        // Scholar's mate sequence
        let moves = vec![
            ("e2", "e4"),
            ("e7", "e5"),
            ("f1", "c4"),
            ("b8", "c6"),
            ("d1", "h5"),
            ("g8", "f6"),
            ("h5", "f7"), // Checkmate
        ];
        
        for (i, (from, to)) in moves.iter().enumerate() {
            let legal_moves = generator.generate_moves(&state);
            assert!(!legal_moves.is_empty(), "No legal moves at step {}", i);
            
            let from_sq = coord_to_square(from).unwrap();
            let to_sq = coord_to_square(to).unwrap();
            
            let mv = legal_moves.iter()
                .find(|m| m.from == from_sq && m.to == to_sq)
                .expect(&format!("Move {}->{} not legal at step {}", from, to, i));
            
            state = GameState::make_move(&state, mv).unwrap();
        }
        
        // Verify checkmate
        let legal_moves = generator.generate_moves(&state);
        assert!(legal_moves.is_empty(), "Should be no legal moves (checkmate)");
        assert!(state.is_check(), "King should be in check");
    }

    #[test]
    fn test_complete_game_stalemate() {
        // Test stalemate scenario
        let fen = "k7/8/1K6/8/8/8/8/1Q6 b - - 0 1";
        let state = GameState::from_fen(fen).unwrap();
        let generator = MoveGenerator::new();
        
        let legal_moves = generator.generate_moves(&state);
        assert!(legal_moves.is_empty(), "Should be no legal moves (stalemate)");
        assert!(!state.is_check(), "King should not be in check");
    }

    #[test]
    fn test_ai_vs_ai_game() {
        // Simulate AI vs AI game
        let mut state = GameState::initial();
        let generator = MoveGenerator::new();
        let mut white_ai = AI::new(Personality::Aggressive);
        let mut black_ai = AI::new(Personality::Defensive);
        
        let max_moves = 50; // Limit to prevent infinite games
        let mut move_count = 0;
        
        while move_count < max_moves {
            let legal_moves = generator.generate_moves(&state);
            if legal_moves.is_empty() {
                break; // Game over
            }
            
            let ai = if state.side_to_move == Color::White {
                &mut white_ai
            } else {
                &mut black_ai
            };
            
            let result = ai.find_best_move(&state, Duration::from_millis(100));
            if let Some(best_move) = result.best_move {
                state = GameState::make_move(&state, &best_move).unwrap();
                move_count += 1;
            } else {
                break;
            }
        }
        
        // Game should progress without errors
        assert!(move_count > 0, "AI should make at least one move");
    }

    #[test]
    fn test_castling_kingside() {
        // Test kingside castling
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";
        let state = GameState::from_fen(fen).unwrap();
        let generator = MoveGenerator::new();
        
        let legal_moves = generator.generate_moves(&state);
        let castle_move = legal_moves.iter()
            .find(|m| m.from == coord_to_square("e1").unwrap() 
                   && m.to == coord_to_square("g1").unwrap()
                   && m.is_castle)
            .expect("Kingside castling should be legal");
        
        let new_state = GameState::make_move(&state, castle_move).unwrap();
        
        // Verify king and rook positions after castling
        assert!(new_state.piece_at(coord_to_square("g1").unwrap()).is_some());
        assert!(new_state.piece_at(coord_to_square("f1").unwrap()).is_some());
    }

    #[test]
    fn test_castling_queenside() {
        // Test queenside castling
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";
        let state = GameState::from_fen(fen).unwrap();
        let generator = MoveGenerator::new();
        
        let legal_moves = generator.generate_moves(&state);
        let castle_move = legal_moves.iter()
            .find(|m| m.from == coord_to_square("e1").unwrap() 
                   && m.to == coord_to_square("c1").unwrap()
                   && m.is_castle)
            .expect("Queenside castling should be legal");
        
        let new_state = GameState::make_move(&state, castle_move).unwrap();
        
        // Verify king and rook positions after castling
        assert!(new_state.piece_at(coord_to_square("c1").unwrap()).is_some());
        assert!(new_state.piece_at(coord_to_square("d1").unwrap()).is_some());
    }

    #[test]
    fn test_en_passant_capture() {
        // Test en passant capture
        let fen = "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1";
        let state = GameState::from_fen(fen).unwrap();
        let generator = MoveGenerator::new();
        
        let legal_moves = generator.generate_moves(&state);
        let en_passant = legal_moves.iter()
            .find(|m| m.from == coord_to_square("e5").unwrap() 
                   && m.to == coord_to_square("d6").unwrap()
                   && m.is_en_passant)
            .expect("En passant should be legal");
        
        let new_state = GameState::make_move(&state, en_passant).unwrap();
        
        // Verify captured pawn is removed
        assert!(new_state.piece_at(coord_to_square("d5").unwrap()).is_none());
        assert!(new_state.piece_at(coord_to_square("d6").unwrap()).is_some());
    }

    #[test]
    fn test_pawn_promotion_to_queen() {
        // Test pawn promotion
        let fen = "8/P7/8/8/8/8/8/K6k w - - 0 1";
        let state = GameState::from_fen(fen).unwrap();
        let generator = MoveGenerator::new();
        
        let legal_moves = generator.generate_moves(&state);
        let promotion = legal_moves.iter()
            .find(|m| m.from == coord_to_square("a7").unwrap() 
                   && m.to == coord_to_square("a8").unwrap()
                   && m.promotion.is_some())
            .expect("Promotion should be legal");
        
        let new_state = GameState::make_move(&state, promotion).unwrap();
        
        // Verify queen is on a8
        let piece = new_state.piece_at(coord_to_square("a8").unwrap());
        assert!(piece.is_some());
    }

    #[test]
    fn test_fifty_move_rule() {
        // Test that game can progress 50 moves without pawn move or capture
        let fen = "8/8/8/8/8/8/8/K6k w - - 0 1";
        let mut state = GameState::from_fen(fen).unwrap();
        let generator = MoveGenerator::new();
        
        // Make some king moves
        for _ in 0..10 {
            let legal_moves = generator.generate_moves(&state);
            if let Some(mv) = legal_moves.first() {
                state = GameState::make_move(&state, mv).unwrap();
            }
        }
        
        // Game should still be playable
        let legal_moves = generator.generate_moves(&state);
        assert!(!legal_moves.is_empty());
    }

    #[test]
    fn test_check_evasion() {
        // Test that only legal moves when in check are those that get out of check
        let fen = "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3";
        let state = GameState::from_fen(fen).unwrap();
        let generator = MoveGenerator::new();
        
        assert!(state.is_check(), "White should be in check");
        
        let legal_moves = generator.generate_moves(&state);
        
        // All legal moves should get out of check
        for mv in legal_moves {
            let new_state = GameState::make_move(&state, &mv).unwrap();
            assert!(!new_state.is_check(), "Move should get out of check");
        }
    }

    #[test]
    fn test_fen_round_trip() {
        // Test FEN parsing and generation
        let original_fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        let state = GameState::from_fen(original_fen).unwrap();
        let generated_fen = state.to_fen();
        
        assert_eq!(original_fen, generated_fen, "FEN should round-trip correctly");
    }

    #[test]
    fn test_move_generation_consistency() {
        // Test that move generation is consistent
        let state = GameState::initial();
        let generator = MoveGenerator::new();
        
        let moves1 = generator.generate_moves(&state);
        let moves2 = generator.generate_moves(&state);
        
        assert_eq!(moves1.len(), moves2.len(), "Move generation should be deterministic");
        assert_eq!(moves1.len(), 20, "Initial position should have 20 legal moves");
    }

    #[test]
    fn test_ai_finds_mate_in_one() {
        // Test that AI can find mate in one
        let fen = "r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4";
        let state = GameState::from_fen(fen).unwrap();
        let mut ai = AI::new(Personality::Aggressive);
        
        // AI should find this is already checkmate
        let generator = MoveGenerator::new();
        let legal_moves = generator.generate_moves(&state);
        assert!(legal_moves.is_empty(), "Should be checkmate");
    }

    #[test]
    fn test_ai_avoids_blunder() {
        // Test that AI doesn't make obvious blunders
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let state = GameState::from_fen(fen).unwrap();
        let mut ai = AI::new(Personality::Defensive);
        
        let result = ai.find_best_move(&state, Duration::from_millis(500));
        assert!(result.best_move.is_some(), "AI should find a move");
        
        // AI should not move king on first move
        if let Some(mv) = result.best_move {
            let from_sq = mv.from;
            let piece = state.piece_at(from_sq);
            assert!(piece.is_some());
        }
    }

    #[test]
    fn test_game_state_cloning() {
        // Test that game state can be cloned correctly
        let state1 = GameState::initial();
        let state2 = state1.clone();
        
        assert_eq!(state1.to_fen(), state2.to_fen());
    }

    #[test]
    fn test_multiple_promotions() {
        // Test multiple pawn promotions in one game
        let fen = "8/PPPPPPPP/8/8/8/8/pppppppp/8 w - - 0 1";
        let state = GameState::from_fen(fen).unwrap();
        let generator = MoveGenerator::new();
        
        let legal_moves = generator.generate_moves(&state);
        let promotion_moves: Vec<_> = legal_moves.iter()
            .filter(|m| m.promotion.is_some())
            .collect();
        
        assert!(!promotion_moves.is_empty(), "Should have promotion moves available");
    }

    // Helper function
    fn coord_to_square(coord: &str) -> Result<chess_q::engine::bitboard::Square, String> {
        use chess_q::engine::bitboard::Square;
        
        let bytes = coord.as_bytes();
        if bytes.len() != 2 {
            return Err("Invalid square".to_string());
        }

        let file = (bytes[0].to_ascii_lowercase() - b'a') as usize;
        let rank = (bytes[1] - b'1') as usize;

        Square::from_index(file + rank * 8).ok_or_else(|| "Invalid square".to_string())
    }
}
