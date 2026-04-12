# ChessQ Technical Documentation

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Chess Engine](#chess-engine)
3. [AI System](#ai-system)
4. [Frontend Application](#frontend-application)
5. [API Reference](#api-reference)
6. [Development Guide](#development-guide)
7. [Performance Optimization](#performance-optimization)

---

## Architecture Overview

ChessQ follows a clean separation between backend (Rust) and frontend (JavaScript):

```
┌─────────────────────────────────────────────────────────┐
│                     Web Browser                         │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Frontend (JavaScript)                            │  │
│  │  - UI Rendering                                   │  │
│  │  - User Input                                     │  │
│  │  - Game State Management                          │  │
│  │  - Time Controls                                  │  │
│  └───────────────┬───────────────────────────────────┘  │
└──────────────────┼──────────────────────────────────────┘
                   │ HTTP/JSON
                   │
┌──────────────────▼──────────────────────────────────────┐
│              Rust Backend                               │
│  ┌─────────────────────────────────────────────────┐   │
│  │  HTTP Server (src/server.rs)                    │   │
│  │  - Static file serving                          │   │
│  │  - API endpoints                                │   │
│  └─────────────────┬───────────────────────────────┘   │
│                    │                                    │
│  ┌─────────────────▼───────────────────────────────┐   │
│  │  Chess Engine (src/engine/)                     │   │
│  │  - Bitboard representation                      │   │
│  │  - Move generation                              │   │
│  │  - Game rules validation                        │   │
│  └─────────────────┬───────────────────────────────┘   │
│                    │                                    │
│  ┌─────────────────▼───────────────────────────────┐   │
│  │  AI System (src/ai/)                            │   │
│  │  - Alpha-beta search                            │   │
│  │  - Position evaluation                          │   │
│  │  - Transposition table                          │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### Design Principles

1. **Backend Authority**: All chess rules and move validation happen in Rust
2. **Frontend Presentation**: JavaScript handles UI, animations, and user interaction
3. **Stateless API**: Each request contains full game state (FEN notation)
4. **Type Safety**: Rust's type system prevents invalid game states
5. **Performance**: Bitboards and magic bitboards for fast move generation

---

## Chess Engine

### Bitboard Representation

ChessQ uses bitboards (64-bit integers) to represent the board state:

```rust
pub struct Bitboard {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,
    pub black_pawns: u64,
    // ... etc
}
```

**Advantages:**
- Fast bitwise operations
- Parallel processing of multiple squares
- Efficient move generation
- Low memory footprint

### Move Generation

#### Magic Bitboards

For sliding pieces (bishops, rooks, queens), ChessQ uses magic bitboards:

```rust
pub fn get_bishop_attacks(square: u8, occupancy: u64) -> u64 {
    let magic = BISHOP_MAGICS[square as usize];
    let index = magic.get_index(occupancy);
    BISHOP_ATTACKS[magic.offset + index]
}
```

**How it works:**
1. Pre-compute all possible attack patterns
2. Use magic numbers to hash occupancy patterns
3. Look up attacks in pre-computed table
4. Result: O(1) move generation

#### Pseudo-legal vs Legal Moves

```rust
// 1. Generate pseudo-legal moves (fast)
let pseudo_legal = generate_pseudo_legal_moves(&board);

// 2. Filter out moves that leave king in check
let legal = pseudo_legal.into_iter()
    .filter(|m| !leaves_king_in_check(&board, m))
    .collect();
```

### FEN Notation

ChessQ uses Forsyth-Edwards Notation (FEN) for board representation:

```
rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
│                                              │ │    │ │ │
│                                              │ │    │ │ └─ Move number
│                                              │ │    │ └─── Half-move clock
│                                              │ │    └───── En passant square
│                                              │ └────────── Castling rights
│                                              └──────────── Active color
└─────────────────────────────────────────────────────────── Board position
```

### Game State Detection

```rust
pub enum GameStatus {
    Ongoing,
    Check,
    Checkmate(Color),  // Winner
    Stalemate,
    Draw,              // 50-move rule, insufficient material
}
```

**Detection logic:**
1. Generate all legal moves
2. If no legal moves:
   - King in check → Checkmate
   - King not in check → Stalemate
3. Check 50-move rule
4. Check insufficient material

---

## AI System

### Search Algorithm

ChessQ uses **Alpha-Beta Pruning** with several enhancements:

```rust
pub fn alpha_beta(
    board: &Board,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    maximizing: bool,
) -> i32 {
    // 1. Check transposition table
    if let Some(score) = tt.probe(board.zobrist_hash, depth) {
        return score;
    }
    
    // 2. Terminal node or depth limit
    if depth == 0 || is_terminal(board) {
        return quiescence_search(board, alpha, beta);
    }
    
    // 3. Generate and order moves
    let moves = generate_moves(board);
    let ordered = order_moves(moves);
    
    // 4. Search recursively
    for mv in ordered {
        let score = -alpha_beta(
            &apply_move(board, mv),
            depth - 1,
            -beta,
            -alpha,
            !maximizing
        );
        
        alpha = max(alpha, score);
        if alpha >= beta {
            break;  // Beta cutoff
        }
    }
    
    // 5. Store in transposition table
    tt.store(board.zobrist_hash, depth, alpha);
    
    alpha
}
```

### Quiescence Search

Prevents horizon effect by searching tactical sequences:

```rust
pub fn quiescence_search(board: &Board, mut alpha: i32, beta: i32) -> i32 {
    let stand_pat = evaluate(board);
    
    if stand_pat >= beta {
        return beta;
    }
    
    alpha = max(alpha, stand_pat);
    
    // Only search captures and checks
    for mv in generate_tactical_moves(board) {
        let score = -quiescence_search(&apply_move(board, mv), -beta, -alpha);
        alpha = max(alpha, score);
        if alpha >= beta {
            break;
        }
    }
    
    alpha
}
```

### Position Evaluation

```rust
pub fn evaluate(board: &Board) -> i32 {
    let mut score = 0;
    
    // 1. Material count
    score += count_material(board);
    
    // 2. Piece-square tables
    score += piece_square_value(board);
    
    // 3. Mobility
    score += mobility_bonus(board);
    
    // 4. Pawn structure
    score += pawn_structure_score(board);
    
    // 5. King safety
    score += king_safety(board);
    
    // 6. Passed pawns
    score += passed_pawn_bonus(board);
    
    score
}
```

**Evaluation Components:**

| Component | Weight | Description |
|-----------|--------|-------------|
| Material | 100-900 | Piece values (P=100, N=320, B=330, R=500, Q=900) |
| Position | 0-50 | Piece-square tables |
| Mobility | 0-30 | Number of legal moves |
| Pawn Structure | 0-50 | Doubled, isolated, backward pawns |
| King Safety | 0-100 | Pawn shield, open files |
| Passed Pawns | 0-100 | Advancement bonus |

### Transposition Table

```rust
pub struct TranspositionTable {
    entries: Vec<TTEntry>,
    size: usize,
}

pub struct TTEntry {
    zobrist_hash: u64,
    depth: u8,
    score: i32,
    best_move: Option<Move>,
    flag: TTFlag,  // Exact, LowerBound, UpperBound
}
```

**Benefits:**
- Avoid re-searching identical positions
- 10-100x speedup in typical games
- 95%+ hit rate in middlegame

### AI Personalities

```rust
pub enum Personality {
    Aggressive,  // Prioritize attacks, sacrifices
    Defensive,   // Prioritize king safety, solid structure
    Chaotic,     // Random move selection with some evaluation
}
```

Implementation:
- **Aggressive**: Bonus for checks, captures, threats
- **Defensive**: Bonus for king safety, pawn structure
- **Chaotic**: Add random noise to evaluation

---

## Frontend Application

### File Structure

```
web/
├── play.html          # Main game interface
├── play.js            # Game logic and API integration
├── play.css           # Styling and animations
├── learn.js           # Learning mode chapters
├── landing-pro.html   # Landing page
├── landing-pro.js     # Landing page interactions
└── landing-pro.css    # Landing page styling
```

### Game State Management

```javascript
const gameState = {
    fen: 'starting position',
    board: [],              // 64-element array
    turn: 'w',              // 'w' or 'b'
    status: 'ongoing',
    selectedSquare: null,
    legalMoves: [],
    moveHistory: [],
    playerColor: 'w',       // For AI games
    opponentType: 'human',  // 'human' or 'ai'
    aiPersonality: 'aggressive'
};
```

### Board Rendering

```javascript
function renderBoard() {
    const board = document.getElementById('chessBoard');
    board.innerHTML = '';
    
    // Flip board if playing as black
    const ranks = gameState.playerColor === 'b' 
        ? [1,2,3,4,5,6,7,8] 
        : [8,7,6,5,4,3,2,1];
    
    for (const rank of ranks) {
        for (let file = 0; file < 8; file++) {
            const square = createSquare(rank, file);
            board.appendChild(square);
        }
    }
}
```

### API Integration

```javascript
async function makeMove(from, to, promotion = null) {
    const url = `/api/move?fen=${encodeURIComponent(gameState.fen)}`
              + `&from=${from}&to=${to}`
              + (promotion ? `&promotion=${promotion}` : '');
    
    const response = await fetch(url);
    const data = await response.json();
    
    if (data.error) {
        showError(data.error);
        return false;
    }
    
    updateGameState(data);
    return true;
}
```

### Time Controls

```javascript
class ChessClock {
    constructor(minutes, increment) {
        this.timeRemaining = minutes * 60 * 1000;
        this.increment = increment * 1000;
        this.isRunning = false;
    }
    
    start() {
        this.isRunning = true;
        this.interval = setInterval(() => {
            this.timeRemaining -= 100;
            this.updateDisplay();
            
            if (this.timeRemaining <= 0) {
                this.onTimeout();
            }
        }, 100);
    }
    
    stop() {
        this.isRunning = false;
        clearInterval(this.interval);
        this.timeRemaining += this.increment;
    }
}
```

### Learning Mode

```javascript
const LEARN_CHAPTERS = [
    {
        id: 'basics',
        title: 'Chess Basics',
        icon: '♟️',
        steps: [
            {
                title: 'The Chessboard',
                body: 'Chess is played on an 8×8 board...',
                fen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
                highlights: []
            },
            // ... more steps
        ]
    },
    // ... more chapters
];
```

---

## API Reference

### Request/Response Format

All API responses follow this structure:

```json
{
    "fen": "current position in FEN notation",
    "board": [/* 64-element array */],
    "status": "ongoing|check|checkmate|stalemate|draw",
    "turn": "w|b",
    "winner": "w|b|null",
    "error": "error message if any"
}
```

### Error Handling

```json
{
    "error": "Invalid move: e2e5",
    "details": "Pawn cannot move 3 squares"
}
```

### Rate Limiting

Currently no rate limiting. For production deployment, consider:
- 100 requests/minute per IP
- 1000 requests/hour per IP
- Exponential backoff for AI moves

---

## Development Guide

### Setting Up Development Environment

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/aidolphin/chessQ.git
cd chessQ

# Run in development mode
cargo run

# Run with auto-reload (install cargo-watch)
cargo install cargo-watch
cargo watch -x run
```

### Running Tests

```bash
# All tests
cargo test

# Specific module
cargo test engine::bitboard

# With output
cargo test -- --nocapture --test-threads=1

# Benchmarks
cargo bench
```

### Code Style

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Check without building
cargo check
```

### Adding New Features

1. **New Chess Rule**:
   - Update `src/engine/move_gen.rs`
   - Add tests in `tests/engine_tests.rs`
   - Update FEN parser if needed

2. **New AI Feature**:
   - Update `src/ai/evaluation.rs` or `src/ai/search.rs`
   - Add benchmark in `benches/`
   - Test with tactical positions

3. **New API Endpoint**:
   - Add route in `src/server.rs`
   - Update frontend in `web/play.js`
   - Document in this file

### Debugging

```rust
// Enable debug logging
env_logger::init();
log::debug!("Move generated: {:?}", mv);

// Print board state
println!("{}", board.to_string());

// Perft debugging
let nodes = perft(&board, depth);
println!("Depth {}: {} nodes", depth, nodes);
```

---

## Performance Optimization

### Profiling

```bash
# CPU profiling
cargo install flamegraph
cargo flamegraph --bin chessQ

# Memory profiling
valgrind --tool=massif target/release/chessQ
```

### Optimization Techniques

1. **Move Ordering**:
   - Try captures first (MVV-LVA)
   - Try killer moves
   - Try history heuristic

2. **Transposition Table Sizing**:
   - Default: 64MB
   - Adjust based on available RAM
   - Clear between games

3. **Search Depth**:
   - Adjust based on time control
   - Use iterative deepening
   - Aspiration windows

4. **Bitboard Operations**:
   - Use SIMD when available
   - Pre-compute attack tables
   - Minimize branching

### Benchmarks

Current performance on M1 MacBook Pro:

```
Move Generation:     10M nodes/sec
Perft(6):           2.5 seconds
Search (depth 6):    0.5 seconds
Transposition hits:  95%
```

---

## Deployment

### Production Build

```bash
# Build optimized binary
cargo build --release

# Binary location
./target/release/chessQ
```

### Docker Deployment

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/chessQ /usr/local/bin/
COPY --from=builder /app/web /app/web
WORKDIR /app
CMD ["chessQ"]
```

### Environment Variables

```bash
CHESSQ_PORT=4173          # Server port
CHESSQ_HOST=127.0.0.1     # Bind address
CHESSQ_WEB_DIR=./web      # Static files directory
```

---

## Troubleshooting

### Common Issues

**Issue**: Pieces not moving
- Check browser console for errors
- Verify API endpoints are responding
- Check FEN notation is valid

**Issue**: AI not responding
- Check server logs for panics
- Verify `/api/ai-move` endpoint
- Reduce search depth if timeout

**Issue**: Board rendering incorrectly
- Clear browser cache
- Check CSS is loading
- Verify board array has 64 elements

### Debug Mode

Enable debug output:

```javascript
// In play.js
const DEBUG = true;

function debugLog(...args) {
    if (DEBUG) console.log('[ChessQ]', ...args);
}
```

---

## Contributing

See [README.md](README.md#-contributing) for contribution guidelines.

---

## License

MIT License - see [LICENSE](LICENSE) file.

---

**Last Updated**: 2024
**Version**: 0.1.0
**Maintainer**: Quantum Leaf Automation
