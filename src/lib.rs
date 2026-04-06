// src/lib.rs
pub mod engine {
    pub mod bitboard;
    pub mod magic;
    pub mod move_gen;
    pub mod zobrist;
    pub mod transposition;
}

pub mod state {
    pub mod game_state;
}

pub mod ai {
    pub mod evaluation;
    pub mod search;
}

pub mod ui {
    pub mod renderer;
}

pub mod server;

#[cfg(test)]
mod tests {
    use crate::engine::move_gen::{Color, MoveGenerator};
    use crate::state::game_state::GameState;

    #[test]
    fn test_initial_position_move_count() {
        let move_gen = MoveGenerator::new();
        let state = GameState::initial();
        let moves = move_gen.generate_moves(&state);

        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn test_perft_position_1() {
        let move_gen = MoveGenerator::new();
        let state = GameState::initial();

        let depth1_moves = move_gen.generate_moves(&state);
        assert_eq!(depth1_moves.len(), 20);
    }

    #[test]
    fn test_make_move_immutable() {
        let state = GameState::initial();
        let move_gen = MoveGenerator::new();
        let moves = move_gen.generate_moves(&state);

        if let Some(first_move) = moves.first() {
            let new_state = GameState::make_move(&state, first_move).unwrap();
            assert_ne!(state, new_state);
            assert_eq!(state.side_to_move, Color::White);
            assert_eq!(new_state.side_to_move, Color::Black);
        }
    }

    #[test]
    fn test_fen_parsing_and_serialization() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let state = GameState::from_fen(fen).unwrap();
        let initial = GameState::initial();

        assert_eq!(state.white_pawns, initial.white_pawns);
        assert_eq!(state.black_knights, initial.black_knights);
        assert_eq!(state.to_fen(), fen);
    }
    
    #[test]
    fn test_en_passant_square_calculation() {
        use crate::engine::bitboard::Square;
        use crate::engine::move_gen::{Move, PieceType};
        
        // Set up position: white pawn at e2, black pawn at e7
        let mut state = GameState::initial();
        
        // White plays e4 - double push
        let e4_move = Move {
            from: Square::E2,
            to: Square::E4,
            piece: PieceType::Pawn,
            capture: None,
            promotion: None,
            is_castle: false,
            is_en_passant: false,
        };
        
        state = GameState::make_move(&state, &e4_move).unwrap();
        
        // After e4, en passant square should be e3
        assert_eq!(state.en_passant_square, Some(Square::E3));
        
        // Switch to black's turn and play e5 - double push
        let e5_move = Move {
            from: Square::E7,
            to: Square::E5,
            piece: PieceType::Pawn,
            capture: None,
            promotion: None,
            is_castle: false,
            is_en_passant: false,
        };
        
        state = GameState::make_move(&state, &e5_move).unwrap();
        
        // After e5, en passant square should be e6
        assert_eq!(state.en_passant_square, Some(Square::E6));
    }
}
