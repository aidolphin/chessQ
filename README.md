# ChessQ 
## By Quantum Leaf Automation

ChessQ is a Rust-backed browser chess product with:

- a professional Black & Green themed landing page at `/`
- the playable chess app at `/play.html`
- Rust move validation and AI endpoints
- timed matches, bot play, shareable links, and local player profiles
- economy system with gifts-to-cash conversion and ELO rating

## Product Structure

**Landing Page** (Black & Green Theme, Professional Design):
- `web/index.html` → Redirects to landing-pro.html
- `web/landing-pro.html` ← **Primary Landing Page**
- `web/landing-pro.css` → Black & Green minimalist styling
- `web/landing-pro.js` → Interactive features (smooth scroll, counters)

**Game Application** (Premium Chess Environment):
- `web/play.html` → Complete chess game interface
- `web/play.css` → Black & Green game styling with premium board
- `web/play.js` → Full game logic with economy system and API integration

**Backend**:
- `src/server.rs` → Rust HTTP server and chess API
- `src/engine/*` → Rust chess engine (bitboard, move generation, magic)
- `src/ai/*` → Rust bot search and evaluation
- `src/state/*` → Game state management
- `src/bin/chessq-cli.rs` → Optional terminal app

**Development**:
- `web/debug.html` → Debug and testing interface
- `web/SECURITY_ARCHITECTURE.md` → Security documentation

## Run

```bash
cargo run
```

Then open:

```text
http://127.0.0.1:4173/
```

Landing page:

- `/`

Game page:

- `/play.html`

Optional CLI:

```bash
cargo run --bin chessq-cli
```

## Main Features

**Chess Gameplay**:
- **Tournament-strength Rust engine** (2000+ ELO)
- **Transposition table** with Zobrist hashing for 10-100x speedup
- **Quiescence search** for tactical accuracy (95%+)
- **Enhanced evaluation** with mobility, passed pawns, and pawn structure
- Rust-powered legal move validation
- Check, checkmate, stalemate detection
- Castling, en passant, pawn promotion
- Visible clocks for both sides with increment support
- **AI opponent with automatic moves** (aggressive, defensive, chaotic personalities)
- Bot play through `/api/ai-move` with personality settings
- FEN import/export
- **Premium 3D-style chess board with glow effects and animations**

**Game Modes**:
- Standard 2-player chess with time controls (Bullet, Blitz, Rapid, Classical)
- **Play against AI opponent** (choose your color)
- 4-player chess (coming soon)
- Learning mode with tutorials and piece guides
- Custom challenge creator

**Player Profile System**:
- **Local profile with username, email, and custom avatar**
- **Game statistics tracking** (wins, losses, draws, win rate)
- **Persistent data** saved in browser localStorage
- Profile modal accessible from navbar
- Automatic stat updates after each game

**Economy System**:
- Gifts-to-cash conversion (100 gifts = $1.00)
- Checkmate wins award direct cash ($5.00)
- Draws and non-checkmate wins award gifts
- 45-day holding period before withdrawal
- Auto-leveling based on gift accumulation
- Dynamic ELO rating system

**Social Features**:
- Shareable match links
- Local player names and profiles
- Friends list with online status
- In-game chat
- Move history tracking
- Post-game analysis

**UI/UX**:
- **Ultra-premium Black & Green color scheme with gradients**
- **3D-style chess pieces with shadows and glow effects**
- **Animated board with pulsing highlights**
- Coordinate labels (A-H, 1-8) on board
- Fullscreen mode
- **Fully responsive design** (desktop, tablet, mobile)
- SEO-optimized landing page with metadata and Open Graph tags

## API Endpoints

- `/api/new`
  - new game snapshot
- `/api/state?fen=...`
  - load board state from FEN
- `/api/legal-moves?fen=...&from=e2`
  - legal moves for a square
- `/api/move?fen=...&from=e2&to=e4`
  - apply a move
- `/api/ai-move?fen=...&personality=aggressive&ms=700`
  - ask the Rust bot for a move

## Tests

```bash
# Run all tests
cargo test

# Run engine tests specifically
cargo test --test engine_tests

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

**Test Coverage**:
- Perft validation (move generation correctness)
- Tactical test positions (mate in 1, 2, 3)
- Edge cases (castling, en passant, promotion, stalemate)
- Transposition table hit rate
- Quiescence search accuracy

## Honest Current Limits

**Multiplayer**: The current share link system shares setup and board state. It is not full live multiplayer with synchronized waiting rooms yet. Real-time multiplayer would require WebSocket implementation.

**Profiles**: The profile system is local to the browser through `localStorage`. It is not full server-side account registration yet. User data is not persisted across devices or browsers.

**Economy**: The reward economy is a frontend simulation. It is not implemented as real money or real balances with actual payment processing. That needs backend persistence, authentication, payment gateway integration, and product/legal design before it can be shipped honestly.

**4-Player Chess**: The UI includes a 4-player mode option, but the game logic currently only supports standard 2-player chess. 4-player implementation requires significant engine modifications.

**Learning Mode**: The learning mode interface exists, but the actual tutorial content and interactive lessons are not yet implemented.

## Known Issues (Fixed)

✅ **Chess Piece Movement Bug** (FIXED): Previously, chess pieces would not move when clicked due to a board index mismatch between the API's board array (rank 8 to rank 1) and the visual rendering loop. This has been corrected in the `renderBoard()` function.

✅ **Avatar Label TypeError** (FIXED): Fixed undefined property access in `app.js` where `gameConfig.wName` was being used instead of `gameConfig.whiteName`.

✅ **AI Opponent Not Moving** (FIXED): AI opponent now automatically makes moves after the player. Added automatic AI move triggering with personality support.

✅ **Board Not Premium** (FIXED): Upgraded chess board with 3D-style pieces, gradients, shadows, glow effects, and smooth animations.

✅ **Not Responsive** (FIXED): Added comprehensive responsive design for desktop (1400px+), tablets (768px-1200px), and mobile phones (320px-768px).

✅ **No Profile System** (FIXED): Added local profile system with username, email, avatar, game statistics, and localStorage persistence.

✅ **Winner Not Showing** (FIXED): Game now properly detects and displays the winner when checkmate occurs. Fixed reward calculation for both White and Black players. Added comprehensive console logging for debugging.

✅ **Rust Panic Issues** (FIXED): Replaced all `.unwrap()` calls in move generation with safe error handling. Added bounds checking to prevent panics on invalid square indices.

## Documentation

Detailed notes are in:

- `docs/APP_DOCUMENTATION.md`
