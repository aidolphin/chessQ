# ♟️ ChessQ

<div align="center">

**A modern chess application built with Rust and vanilla JavaScript**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

[Features](#features) • [Quick Start](#quick-start) • [Documentation](#documentation) • [Contributing](#contributing)

</div>

---

## Overview

ChessQ is a full-featured chess application combining a Rust-powered chess engine with a modern web interface. Play against an AI opponent, practice with interactive lessons, or challenge friends locally.

### Key Features

- 🎮 **Play Chess**: Standard chess with multiple time controls
- 🤖 **AI Opponent**: Play against computer with different personalities
- 🎓 **Learn Chess**: 9 interactive chapters teaching fundamentals
- ⏱️ **Time Controls**: Bullet, Blitz, Rapid, and Classical modes
- 📊 **Track Progress**: Local statistics and ELO rating system
- 📱 **Responsive Design**: Works on desktop, tablet, and mobile

---

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or higher
- Modern web browser (Chrome, Firefox, Safari, Edge)

### Installation

```bash
# Clone the repository
git clone https://github.com/aidolphin/chessQ.git
cd chessQ

# Build and run
cargo run --release
```

The application will start on `http://127.0.0.1:4173`

### First Game

1. Open `http://127.0.0.1:4173` in your browser
2. Click **"Standard Chess"**
3. Choose **"Play Against AI"**
4. Select your color (White/Black/Random)
5. Pick a time control
6. Click **"Start Game"** and play!

---

## Features

### Chess Engine

- **Bitboard Representation**: Efficient board state management
- **Magic Bitboards**: Fast move generation for sliding pieces
- **Full Rules**: Castling, en passant, promotion, check/checkmate
- **FEN Support**: Import/export positions

### AI System

- **Alpha-Beta Search**: Intelligent move selection
- **Transposition Table**: Zobrist hashing for performance
- **Quiescence Search**: Tactical accuracy
- **Three Personalities**:
  - Aggressive: Attacks and sacrifices
  - Defensive: Solid and safe play
  - Chaotic: Unpredictable moves

### Game Modes

- **Standard Chess**: Classic 2-player game
- **vs AI**: Play against computer opponent
- **Learning Mode**: 9 interactive tutorials
- **Time Controls**: Bullet (1-2min), Blitz (3-5min), Rapid (10-30min), Classical (30min+)

### User Interface

- **Premium Design**: Black & green theme with 3D-style pieces
- **Board Flip**: Automatic orientation for black player
- **Move Highlighting**: Visual feedback for legal moves
- **Move History**: Review all moves in notation
- **Notifications**: Game events and updates
- **Profile System**: Track your statistics locally

### Learning Mode

9 interactive chapters covering:
1. Chess Basics
2. The Pawn
3. The Knight
4. The Bishop
5. The Rook
6. The Queen
7. The King
8. Checkmate Patterns
9. Tactics

---

## Project Structure

```
chessQ/
├── src/
│   ├── engine/          # Chess engine (bitboards, move generation)
│   ├── ai/              # AI opponent (search, evaluation)
│   ├── state/           # Game state management
│   ├── server.rs        # HTTP server and API
│   └── main.rs          # Application entry point
├── web/
│   ├── play.html        # Main game interface
│   ├── play.js          # Game logic and UI
│   ├── play.css         # Styling
│   ├── learn.js         # Learning mode
│   └── landing-pro.html # Landing page
├── tests/
│   ├── engine_tests.rs      # Engine unit tests
│   ├── integration_tests.rs # API integration tests
│   └── e2e_tests.rs         # End-to-end tests
└── docs/
    └── DOCUMENTATION.md     # Technical documentation
```

---

## API Endpoints

### `GET /api/new`
Create a new game with starting position.

### `GET /api/state?fen={fen}`
Load a board state from FEN notation.

### `GET /api/legal-moves?fen={fen}&from={square}`
Get legal moves for a piece at the specified square.

### `GET /api/move?fen={fen}&from={from}&to={to}`
Apply a move to the board.

### `GET /api/ai-move?fen={fen}&personality={style}&ms={time}`
Request an AI move with specified personality and time limit.

---

## Testing

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test engine_tests
cargo test --test integration_tests -- --ignored
cargo test --test e2e_tests

# Run benchmarks
cargo bench
```

---

## Documentation

- **[Quick Start Guide](QUICKSTART.md)** - Get started in 5 minutes
- **[Technical Documentation](docs/DOCUMENTATION.md)** - Architecture and implementation details
- **[Contributing Guide](CONTRIBUTING.md)** - How to contribute
- **[Roadmap](ROADMAP.md)** - Future development plans
- **[Changelog](CHANGELOG.md)** - Version history

---

## Development

### Running in Development Mode

```bash
# Start server with auto-reload
cargo watch -x run

# Run tests on file changes
cargo watch -x test
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check without building
cargo check
```

---

## Performance

- **Move Generation**: ~10M nodes/second
- **Search Depth**: 6-8 ply in middlegame
- **Transposition Table**: 95%+ hit rate
- **API Response**: <100ms typical
- **Memory Usage**: ~50MB

---

## Known Limitations

ChessQ is transparent about current limitations:

- ❌ **No Real-time Multiplayer**: WebSocket implementation planned
- ❌ **Local Storage Only**: Server-side profiles not yet implemented
- ❌ **Economy System**: Frontend simulation only (not real money)
- ❌ **4-Player Chess**: UI exists but logic not implemented

See [ROADMAP.md](ROADMAP.md) for planned features and implementation timeline.

---

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Ways to Contribute

- 🐛 Report bugs
- 💡 Suggest features
- 📝 Improve documentation
- 🔧 Submit pull requests
- ⭐ Star the repository

---

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

---

## Acknowledgments

- Chess programming community for algorithms and techniques
- Rust community for excellent tooling
- Contributors and testers

---

## Contact

- **Repository**: [github.com/aidolphin/chessQ](https://github.com/aidolphin/chessQ)
- **Issues**: [Report a bug](https://github.com/aidolphin/chessQ/issues)

---

<div align="center">

**Made with ❤️ by Quantum Leaf Automation**

[⬆ Back to Top](#️-chessq)

</div>
