// src/ui/renderer.rs
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color as UiColor, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crossterm::event::{self, Event, KeyCode};
use std::io;

use crate::ai::search::AI;
use crate::engine::bitboard::Square;
use crate::engine::move_gen::{Color as PieceColor, Move, PieceType};
use crate::state::game_state::GameState;

/// Main TUI application
pub struct ChessTUI {
    game_state: GameState,
    cursor_square: Square,
    selected_square: Option<Square>,
    legal_moves: Vec<Move>,
    move_history: Vec<Move>,
    captured_pieces: Vec<(PieceColor, PieceType)>,
    ai: Option<AI>,
    game_over: Option<String>,
    status_message: String,
}

impl ChessTUI {
    pub fn new() -> Self {
        let game_state = GameState::initial();
        let legal_moves = Vec::new();
        
        Self {
            game_state,
            cursor_square: Square::E2,
            selected_square: None,
            legal_moves,
            move_history: Vec::new(),
            captured_pieces: Vec::new(),
            ai: None,
            game_over: None,
            status_message: "Welcome to Rust Chess! Press 'a' for AI mode".to_string(),
        }
    }
    
    pub fn enable_ai(&mut self, personality: crate::ai::evaluation::Personality) {
        self.ai = Some(AI::new(personality));
        self.status_message = format!("AI enabled with {:?} personality", personality);
    }
    
    pub fn run<B: Backend>(&mut self, terminal: &mut ratatui::Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.draw(f))?;
            
            if let Some(result) = &self.game_over {
                self.status_message = result.clone();
                // Wait for user input to exit
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
                continue;
            }
            
            let should_ai_move = self.ai.is_some()
                && self.game_state.side_to_move == PieceColor::Black
                && self.game_over.is_none();
            if should_ai_move {
                self.status_message = "AI is thinking...".to_string();
                terminal.draw(|f| self.draw(f))?;

                let result = self
                    .ai
                    .as_mut()
                    .unwrap()
                    .find_best_move(&self.game_state, std::time::Duration::from_secs(2));

                if let Some(mv) = result.best_move {
                    match GameState::make_move(&self.game_state, &mv) {
                        Ok(new_state) => {
                            self.make_move(mv, new_state);
                            self.status_message =
                                format!("AI played {} (eval: {})", mv.algebraic(), result.evaluation);
                        }
                        Err(e) => {
                            self.status_message = format!("AI error: {}", e);
                        }
                    }
                }
                continue;
            }
            
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('a') => {
                        self.enable_ai(crate::ai::evaluation::Personality::Aggressive);
                    }
                    KeyCode::Char('d') => {
                        self.enable_ai(crate::ai::evaluation::Personality::Defensive);
                    }
                    KeyCode::Char('c') => {
                        self.enable_ai(crate::ai::evaluation::Personality::Chaotic);
                    }
                    KeyCode::Char('r') => {
                        *self = Self::new();
                    }
                    KeyCode::Char(' ') => {
                        self.selected_square = None;
                        self.legal_moves.clear();
                    }
                    KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                        self.handle_cursor(key.code);
                    }
                    KeyCode::Enter => {
                        self.handle_move(self.cursor_square);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
    
    fn draw(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(f.size());
        
        // Draw chessboard
        self.draw_board(f, chunks[0]);
        
        // Draw sidebar with game info
        self.draw_sidebar(f, chunks[1]);
    }
    
    fn draw_board(&self, f: &mut Frame, area: Rect) {
        let board_size = area.height.min(area.width);
        let square_size = board_size / 8;
        
        let board_area = Rect {
            x: area.x + (area.width - board_size) / 2,
            y: area.y,
            width: board_size,
            height: board_size,
        };
        
        for rank in 0..8 {
            for file in 0..8 {
                let square = Square::from_index(file + rank * 8).unwrap();
                let x = board_area.x + file as u16 * square_size;
                let y = board_area.y + (7 - rank) as u16 * square_size;
                
                let is_light = (file + rank) % 2 == 0;
                let bg_color = if is_light {
                    UiColor::Rgb(240, 217, 181)
                } else {
                    UiColor::Rgb(181, 136, 99)
                };
                
                let is_cursor = self.cursor_square == square;
                let is_selected = self.selected_square == Some(square);
                let is_legal_move = self.legal_moves.iter().any(|mv| mv.to == square);
                
                let bg_color = if is_selected {
                    UiColor::Rgb(100, 200, 100)
                } else if is_cursor {
                    UiColor::Rgb(120, 170, 220)
                } else if is_legal_move {
                    UiColor::Rgb(200, 200, 100)
                } else {
                    bg_color
                };
                
                let piece_char = self.get_piece_char(square);
                let piece_color = if self.game_state.piece_at(square).map(|(c, _)| c) == Some(PieceColor::White) {
                    UiColor::White
                } else {
                    UiColor::Black
                };
                
                let block = Block::default()
                    .style(Style::default().bg(bg_color));
                
                let paragraph = Paragraph::new(Span::styled(piece_char, Style::default().fg(piece_color)))
                    .block(block)
                    .alignment(Alignment::Center);
                
                f.render_widget(paragraph, Rect {
                    x,
                    y,
                    width: square_size,
                    height: square_size,
                });
            }
        }
    }
    
    fn draw_sidebar(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Length(10),
                Constraint::Min(0),
            ].as_ref())
            .split(area);
        
        // Status
        let status = Paragraph::new(self.status_message.as_str())
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(status, chunks[0]);
        
        // Game info
        let info_text = Text::from(vec![
            Line::from(vec![
                Span::styled("Turn: ", Style::default().fg(UiColor::Yellow)),
                Span::styled(
                    if self.game_state.side_to_move == PieceColor::White { "White" } else { "Black" },
                    Style::default().fg(UiColor::Green).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(vec![
                Span::styled("Full moves: ", Style::default().fg(UiColor::Yellow)),
                Span::styled(self.game_state.fullmove_number.to_string(), Style::default()),
            ]),
            Line::from(vec![
                Span::styled("Half moves: ", Style::default().fg(UiColor::Yellow)),
                Span::styled(self.game_state.halfmove_clock.to_string(), Style::default()),
            ]),
        ]);
        
        let info = Paragraph::new(info_text)
            .block(Block::default().borders(Borders::ALL).title("Game Info"));
        f.render_widget(info, chunks[1]);
        
        // Move history
        let history_text: Vec<Line> = self.move_history.iter().enumerate()
            .skip(self.move_history.len().saturating_sub(10))
            .map(|(i, mv)| {
                Line::from(vec![
                    Span::styled(format!("{}. ", i / 2 + 1), Style::default().fg(UiColor::DarkGray)),
                    Span::styled(mv.algebraic(), Style::default()),
                ])
            })
            .collect();
        
        let history = Paragraph::new(history_text)
            .block(Block::default().borders(Borders::ALL).title("Move History"));
        f.render_widget(history, chunks[2]);
        
        // Controls
        let controls_text = Text::from(vec![
            Line::from("Controls:"),
            Line::from("  Arrow keys - Move cursor"),
            Line::from("  Enter - Select/Move"),
            Line::from("  Space - Deselect"),
            Line::from("  a - Aggressive AI"),
            Line::from("  d - Defensive AI"),
            Line::from("  c - Chaotic AI"),
            Line::from("  r - Reset game"),
            Line::from("  q - Quit"),
        ]);
        
        let controls = Paragraph::new(controls_text)
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(controls, chunks[3]);
    }
    
    fn get_piece_char(&self, square: Square) -> String {
        match self.game_state.piece_at(square) {
            Some((PieceColor::White, PieceType::Pawn)) => "♙".to_string(),
            Some((PieceColor::White, PieceType::Knight)) => "♘".to_string(),
            Some((PieceColor::White, PieceType::Bishop)) => "♗".to_string(),
            Some((PieceColor::White, PieceType::Rook)) => "♖".to_string(),
            Some((PieceColor::White, PieceType::Queen)) => "♕".to_string(),
            Some((PieceColor::White, PieceType::King)) => "♔".to_string(),
            Some((PieceColor::Black, PieceType::Pawn)) => "♟".to_string(),
            Some((PieceColor::Black, PieceType::Knight)) => "♞".to_string(),
            Some((PieceColor::Black, PieceType::Bishop)) => "♝".to_string(),
            Some((PieceColor::Black, PieceType::Rook)) => "♜".to_string(),
            Some((PieceColor::Black, PieceType::Queen)) => "♛".to_string(),
            Some((PieceColor::Black, PieceType::King)) => "♚".to_string(),
            None => " ".to_string(),
        }
    }
    
    fn handle_cursor(&mut self, key: KeyCode) {
        let current = self.cursor_square;
        let mut file = current.file() as i32;
        let mut rank = current.rank() as i32;
        
        match key {
            KeyCode::Up => rank = (rank + 1).min(7),
            KeyCode::Down => rank = (rank - 1).max(0),
            KeyCode::Left => file = (file - 1).max(0),
            KeyCode::Right => file = (file + 1).min(7),
            _ => {}
        }
        
        self.cursor_square = Square::from_index((file + rank * 8) as usize).unwrap();
    }
    
    fn handle_move(&mut self, to: Square) {
        let from = if let Some(sq) = self.selected_square {
            sq
        } else {
            self.selected_square = Some(to);
            self.update_legal_moves();
            return;
        };
        
        // Find the move
        let move_gen = crate::engine::move_gen::MoveGenerator::new();
        let legal_moves = move_gen.generate_moves(&self.game_state);
        
        if let Some(mv) = legal_moves.iter().find(|mv| mv.from == from && mv.to == to) {
            match GameState::make_move(&self.game_state, mv) {
                Ok(new_state) => {
                    self.make_move(*mv, new_state);
                    self.status_message = format!("Played: {}", mv.algebraic());
                }
                Err(e) => {
                    self.status_message = format!("Illegal move: {}", e);
                }
            }
        } else {
            self.status_message = "Not a legal move".to_string();
        }
        
        self.selected_square = None;
        self.legal_moves.clear();
    }
    
    fn update_legal_moves(&mut self) {
        if let Some(selected) = self.selected_square {
            let move_gen = crate::engine::move_gen::MoveGenerator::new();
            self.legal_moves = move_gen.generate_moves(&self.game_state)
                .into_iter()
                .filter(|mv| mv.from == selected)
                .collect();
        } else {
            self.legal_moves.clear();
        }
    }
    
    fn make_move(&mut self, mv: Move, new_state: GameState) {
        if let Some(piece) = mv.capture {
            self.captured_pieces.push((self.game_state.side_to_move.opposite(), piece));
        }
        self.move_history.push(mv);
        self.game_state = new_state;
        self.cursor_square = mv.to;
        
        self.update_legal_moves();
        
        // Check game over
        let move_gen = crate::engine::move_gen::MoveGenerator::new();
        if let Some(result) = self.game_state.result(&move_gen) {
            self.game_over = Some(result);
        }
    }
}
