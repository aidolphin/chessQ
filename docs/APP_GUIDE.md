# ChessQ Application Documentation

## Table of Contents

1. [Overview](#overview)
2. [Getting Started](#getting-started)
3. [User Guide](#user-guide)
4. [Features](#features)
5. [Technical Details](#technical-details)
6. [Troubleshooting](#troubleshooting)

---

## Overview

ChessQ is a modern chess application that combines a powerful Rust chess engine with an intuitive web interface. Whether you're learning chess or looking to improve your skills, ChessQ provides the tools you need.

### What Makes ChessQ Special

- **Fast & Reliable**: Rust-powered engine ensures quick, accurate move validation
- **Educational**: Built-in learning mode with 9 interactive chapters
- **Flexible**: Multiple game modes and time controls
- **Privacy-Focused**: All data stored locally in your browser
- **Open Source**: Transparent, community-driven development

---

## Getting Started

### System Requirements

- **Operating System**: Windows, macOS, or Linux
- **Rust**: Version 1.70 or higher
- **Browser**: Chrome 90+, Firefox 88+, Safari 14+, or Edge 90+
- **RAM**: 100MB minimum
- **Disk Space**: 50MB

### Installation Steps

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/aidolphin/chessQ.git
   cd chessQ
   ```

3. **Build and Run**:
   ```bash
   cargo run --release
   ```

4. **Open in Browser**:
   Navigate to `http://127.0.0.1:4173`

### First Time Setup

When you first open ChessQ:

1. Click the user avatar in the top-right corner
2. Enter your username (required)
3. Select your country (required)
4. Optionally add your email
5. Choose a 2-letter avatar
6. Click "Save Changes"

Your profile is saved locally and persists across sessions.

---

## User Guide

### Playing Your First Game

#### Against AI

1. Click **"Standard Chess"** on the home screen
2. Select **"Play Against AI"**
3. Choose your color:
   - **White**: You move first
   - **Black**: AI moves first
   - **Random**: Randomly assigned
4. Select a time control:
   - **Bullet**: 1-2 minutes (fast-paced)
   - **Blitz**: 3-5 minutes (quick)
   - **Rapid**: 10-30 minutes (standard)
   - **Classical**: 30+ minutes (slow)
5. Click **"Start Game"**

#### Against Human (Local)

1. Click **"Standard Chess"**
2. Select **"Play Against Human"**
3. Wait for opponent to join (simulated)
4. Select time control
5. Click **"Start Game"**

### Making Moves

1. **Select a Piece**: Click on one of your pieces
   - Legal moves will be highlighted in green
   - Selected piece will have a glowing border

2. **Move the Piece**: Click on a highlighted square
   - The piece will move to that square
   - Move notation appears in the history panel

3. **Deselect**: Click the same piece again to deselect

### Special Moves

#### Castling
- Click your king, then click two squares toward the rook
- Only available if:
  - King and rook haven't moved
  - No pieces between them
  - King not in check
  - King doesn't pass through check

#### En Passant
- Automatically available when opponent's pawn moves two squares
- Click your pawn, then click the diagonal square behind opponent's pawn

#### Pawn Promotion
- When your pawn reaches the last rank
- Automatically promotes to Queen (most common choice)

### Game Controls

Located below the chess board:

- **Resign** (🏳️): Forfeit the game
- **Offer Draw** (🤝): Propose a draw to opponent
- **Request Undo** (↩️): Ask to take back last move
- **Fullscreen** (⛶): Toggle fullscreen mode
- **Share** (🔗): Generate shareable game link

### Time Controls

Your clock is displayed below the board:
- **Green glow**: Your turn (clock running)
- **No glow**: Opponent's turn

Time runs out = automatic loss

### Move History

Right sidebar shows all moves in standard chess notation:
- **1. e4 e5**: Move number, White's move, Black's move
- **2. Nf3 Nc6**: Knight moves are prefixed with 'N'
- **+**: Check
- **#**: Checkmate

---

## Features

### Learning Mode

Access via **"Learning Mode"** on home screen.

#### Available Chapters

1. **Chess Basics**
   - Board setup
   - Piece names
   - Game objective

2. **The Pawn**
   - Movement rules
   - Capturing
   - Promotion

3. **The Knight**
   - L-shaped movement
   - Jumping ability
   - Fork tactics

4. **The Bishop**
   - Diagonal movement
   - Color binding
   - Long-range control

5. **The Rook**
   - Horizontal/vertical movement
   - Castling
   - Open files

6. **The Queen**
   - Combined bishop + rook movement
   - Most powerful piece
   - Positioning

7. **The King**
   - One square movement
   - Castling
   - Safety principles

8. **Checkmate Patterns**
   - Back rank mate
   - Smothered mate
   - Queen + King mate

9. **Tactics**
   - Forks
   - Pins
   - Skewers
   - Discovered attacks

#### Using Learning Mode

1. Select a chapter
2. Read the explanation
3. Observe the live board demonstration
4. Answer quiz questions
5. Navigate with **Previous** / **Next** buttons

### Profile & Statistics

Access via user avatar (top-right corner).

#### Tracked Statistics

- **Games Played**: Total games completed
- **Wins**: Games won
- **Losses**: Games lost
- **Draws**: Games drawn
- **Win Rate**: Percentage of games won
- **ELO Rating**: Skill rating (starts at 1500)

#### ELO Rating System

- **Win**: +15 points
- **Loss**: -15 points
- **Draw**: No change

Rating ranges:
- **1000-1200**: Beginner
- **1200-1400**: Novice
- **1400-1600**: Intermediate
- **1600-1800**: Advanced
- **1800-2000**: Expert
- **2000+**: Master

### Economy System

*Note: This is a frontend simulation for demonstration purposes.*

#### How It Works

- **Checkmate Win**: Earn $5.00
- **Draw**: Earn 2 gifts
- **Loss**: Earn 0 gifts
- **Conversion**: 5000 gifts = $1.00

#### Leveling System

- Start at Level 1
- Earn gifts to level up
- Each level requires more gifts
- Track progress in right sidebar

### Notifications

Bell icon (top-right) shows game events:
- Game results
- Achievements
- System messages

Click to view notification history.

### Chat System

Click chat icon (💬) to open chat panel:
- Send messages during games
- Receive opponent responses
- View message history

*Note: Currently simulated for local games*

---

## Technical Details

### Architecture

```
┌─────────────────────────────────────┐
│         Web Browser                 │
│  ┌───────────────────────────────┐  │
│  │  JavaScript (play.js)         │  │
│  │  - UI Rendering               │  │
│  │  - User Input                 │  │
│  │  - Local Storage              │  │
│  └───────────┬───────────────────┘  │
└──────────────┼──────────────────────┘
               │ HTTP/JSON
┌──────────────▼──────────────────────┐
│         Rust Server                 │
│  ┌───────────────────────────────┐  │
│  │  HTTP Server (server.rs)      │  │
│  │  - API Endpoints              │  │
│  │  - Static Files               │  │
│  └───────────┬───────────────────┘  │
│              │                       │
│  ┌───────────▼───────────────────┐  │
│  │  Chess Engine                 │  │
│  │  - Move Generation            │  │
│  │  - Rules Validation           │  │
│  │  - AI Search                  │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

### Data Storage

All data is stored locally in your browser using `localStorage`:

- **Profile**: `chessq_profile`
- **Economy**: `chessq_economy`
- **Notifications**: `chessq_notifications`

To reset all data:
```javascript
localStorage.clear();
```

### API Endpoints

#### Create New Game
```
GET /api/new
Response: { fen, board, turn, status }
```

#### Load Position
```
GET /api/state?fen={fen}
Response: { fen, board, turn, status }
```

#### Get Legal Moves
```
GET /api/legal-moves?fen={fen}&from={square}
Response: { moves: [{to, from, ...}] }
```

#### Make Move
```
GET /api/move?fen={fen}&from={from}&to={to}
Response: { snapshot: {...}, moveNotation }
```

#### AI Move
```
GET /api/ai-move?fen={fen}&personality={style}&ms={time}
Response: { snapshot: {...}, moveNotation, evaluation, depth }
```

### AI Personalities

#### Aggressive
- Prioritizes attacks
- Accepts sacrifices
- Seeks tactical complications
- Best for: Learning to defend

#### Defensive
- Prioritizes king safety
- Avoids risks
- Solid positional play
- Best for: Learning to attack

#### Chaotic
- Unpredictable moves
- Random evaluation noise
- Varied play style
- Best for: Fun, casual games

---

## Troubleshooting

### Server Won't Start

**Problem**: `cargo run` fails or shows errors

**Solutions**:
1. Check Rust version: `rustc --version` (need 1.70+)
2. Update Rust: `rustup update`
3. Clean build: `cargo clean && cargo build --release`
4. Check port 4173 is free: `lsof -i :4173`

### Board Not Displaying

**Problem**: Blank screen or no chess board

**Solutions**:
1. Hard refresh: `Ctrl+Shift+R` (Windows/Linux) or `Cmd+Shift+R` (Mac)
2. Clear browser cache
3. Check browser console (F12) for errors
4. Try different browser

### Pieces Won't Move

**Problem**: Clicking pieces doesn't work

**Solutions**:
1. Ensure it's your turn (check clock glow)
2. Select your own pieces (not opponent's)
3. Click legal move squares (highlighted in green)
4. Refresh page if stuck

### AI Not Responding

**Problem**: AI doesn't make moves

**Solutions**:
1. Wait 1-2 seconds (AI is thinking)
2. Check server console for errors
3. Restart server
4. Check browser console (F12)

### Time Not Running

**Problem**: Clock doesn't count down

**Solutions**:
1. Ensure game has started
2. Check if game is paused
3. Refresh page
4. Start new game

### Profile Not Saving

**Problem**: Statistics reset after refresh

**Solutions**:
1. Check browser allows localStorage
2. Don't use private/incognito mode
3. Check browser storage settings
4. Try different browser

### Performance Issues

**Problem**: Slow or laggy gameplay

**Solutions**:
1. Close other browser tabs
2. Use `--release` flag: `cargo run --release`
3. Reduce AI thinking time
4. Check system resources

---

## Keyboard Shortcuts

- **Esc**: Close modals
- **F11**: Toggle fullscreen
- **Ctrl+R**: Refresh board

---

## Tips & Best Practices

### For Beginners

1. **Complete Learning Mode**: Start with all 9 chapters
2. **Play Defensive AI**: Learn attacking patterns
3. **Use Longer Time Controls**: Rapid or Classical
4. **Review Move History**: Learn from mistakes
5. **Practice Tactics**: Focus on forks, pins, skewers

### For Intermediate Players

1. **Play Aggressive AI**: Improve defensive skills
2. **Use Shorter Time Controls**: Blitz or Bullet
3. **Analyze Games**: Review critical positions
4. **Practice Openings**: Learn 2-3 openings well
5. **Study Endgames**: King and pawn endings

### For Advanced Players

1. **Play Chaotic AI**: Adapt to unexpected moves
2. **Use Bullet Time**: Improve calculation speed
3. **Set Challenges**: Win without losing pieces
4. **Experiment**: Try unusual openings
5. **Teach Others**: Share knowledge

---

## Frequently Asked Questions

### Is ChessQ free?

Yes, ChessQ is completely free and open-source under the MIT License.

### Do I need an account?

No, ChessQ works entirely in your browser with local storage. No registration required.

### Can I play online against others?

Currently, ChessQ supports local play and AI opponents. Online multiplayer is planned for future releases.

### Is the economy system real?

No, the economy system is a frontend simulation for demonstration purposes. No real money is involved.

### How strong is the AI?

The AI plays at approximately 1800-2000 ELO strength, suitable for most players.

### Can I export my games?

PGN export is planned for a future release. Currently, you can share game links.

### Does ChessQ work offline?

The server must be running, but no internet connection is required.

### Can I contribute?

Yes! See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

---

## Support

### Getting Help

- **Documentation**: Check this guide first
- **GitHub Issues**: [Report bugs](https://github.com/aidolphin/chessQ/issues)
- **Discussions**: [Ask questions](https://github.com/aidolphin/chessQ/discussions)

### Reporting Bugs

When reporting bugs, include:
1. Operating system and version
2. Browser and version
3. Steps to reproduce
4. Expected vs actual behavior
5. Screenshots if applicable
6. Browser console errors (F12)

---

## Version History

See [CHANGELOG.md](../CHANGELOG.md) for detailed version history.

---

## Credits

- **Chess Engine**: Custom Rust implementation
- **UI Design**: Vanilla JavaScript and CSS
- **Chess Pieces**: Unicode characters
- **Inspiration**: Chess programming community

---

**Last Updated**: 2024
**Version**: 0.1.0
**Maintainer**: Quantum Leaf Automation
