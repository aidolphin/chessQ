# 🚀 ChessQ Quick Start Guide

Get up and running with ChessQ in 5 minutes!

## 📋 Prerequisites

- **Rust** 1.70 or higher ([Install Rust](https://www.rust-lang.org/tools/install))
- **Modern web browser** (Chrome, Firefox, Safari, Edge)

## ⚡ Installation

### Option 1: Quick Start (Recommended)

```bash
# Clone the repository
git clone https://github.com/aidolphin/chessQ.git

# Navigate to directory
cd chessQ

# Run the application
cargo run --release
```

That's it! Open your browser to `http://127.0.0.1:4173`

### Option 2: Development Mode

```bash
# Clone and navigate
git clone https://github.com/aidolphin/chessQ.git
cd chessQ

# Run in development mode (faster compilation)
cargo run
```

### Option 3: CLI Mode

```bash
# Run terminal chess interface
cargo run --bin chessq-cli
```

## 🎮 First Game

### 1. Open the Application

Navigate to `http://127.0.0.1:4173` in your browser.

### 2. Choose Game Mode

Click **"Standard Chess"** on the landing page.

### 3. Select Opponent

- **Play Against AI**: Choose this to play against the computer
- **Play Against Human**: Choose this for local 2-player

### 4. Choose Your Color (AI games only)

- **White**: You move first
- **Black**: AI moves first
- **Random**: Surprise!

### 5. Select Time Control

Choose from:
- ⚡ **Bullet**: 1-2 minutes (fast-paced)
- 🔥 **Blitz**: 3-5 minutes (quick games)
- ⏱️ **Rapid**: 10-30 minutes (standard)
- 📅 **Daily**: 1-7 days (correspondence)

### 6. Play!

- Click a piece to select it
- Click a highlighted square to move
- Your clock starts when it's your turn
- AI responds automatically

## 🎓 Learning Chess

### Access Learning Mode

1. Click **"Learning Mode"** from the main menu
2. Choose a chapter:
   - Chess Basics
   - The Pawn
   - The Knight
   - The Bishop
   - The Rook
   - The Queen
   - The King
   - Checkmate Patterns
   - Tactics

### Interactive Lessons

Each chapter includes:
- Live chess board demonstrations
- Step-by-step explanations
- Interactive quizzes
- Highlighted squares showing concepts

## 🎯 Key Features

### During a Game

**Move Pieces**
- Click piece → Click destination
- Legal moves are highlighted

**View Move History**
- Right sidebar shows all moves
- Scroll to review the game

**Chat**
- Click chat icon in navbar
- Send messages to opponent

**Game Controls**
- 🏳️ Resign
- 🤝 Offer Draw
- ↩️ Request Undo
- 🔗 Share Game
- ⛶ Fullscreen

### After a Game

**View Results**
- Winner announcement
- Rewards earned
- Rating change
- Game statistics

**Play Again**
- Click "Play Again" button
- Same settings, new game

**View Analysis**
- Review move history
- See game statistics
- Check accuracy

## 👤 Profile System

### Create Profile

1. Click user avatar in navbar
2. Enter username and email
3. Choose avatar style
4. Save profile

### Track Statistics

Your profile tracks:
- Total games played
- Wins / Losses / Draws
- Win rate percentage
- Current ELO rating
- Recent game history

## 🔔 Notifications

Click the bell icon to see:
- Game results
- Rewards earned
- System messages

## 🎨 Customization

### Board Orientation

When playing as Black, the board automatically flips to show your pieces at the bottom.

### Time Controls

Customize time controls:
- Base time (1 min to 7 days)
- Increment per move (0-10 seconds)

### AI Personality

Choose AI style:
- **Aggressive**: Attacks and sacrifices
- **Defensive**: Solid and safe
- **Chaotic**: Unpredictable moves

## 🔗 Sharing Games

### Share Current Position

1. Click share icon (🔗)
2. Copy the generated link
3. Send to friends
4. They open the same position

### Share on Social Media

- Discord
- Twitter/X
- WhatsApp

## 🐛 Troubleshooting

### Server Won't Start

```bash
# Check if port 4173 is in use
lsof -i :4173

# Kill the process if needed
kill -9 <PID>

# Try again
cargo run
```

### Pieces Not Moving

1. Check browser console (F12)
2. Refresh the page (Ctrl+R / Cmd+R)
3. Clear browser cache
4. Restart server

### AI Not Responding

1. Check server logs in terminal
2. Verify `/api/ai-move` endpoint
3. Try reducing search depth
4. Restart server

### Board Rendering Issues

1. Clear browser cache
2. Hard refresh (Ctrl+Shift+R / Cmd+Shift+R)
3. Try different browser
4. Check CSS is loading

## 📚 Next Steps

### Learn More

- Read [README.md](README.md) for full feature list
- Check [DOCUMENTATION.md](docs/DOCUMENTATION.md) for technical details
- See [CONTRIBUTING.md](CONTRIBUTING.md) to contribute

### Improve Your Chess

1. Complete all learning chapters
2. Play against AI on different difficulties
3. Review your game history
4. Practice tactics and patterns

### Contribute

- Report bugs on GitHub
- Suggest features
- Submit pull requests
- Improve documentation

## 🎯 Common Tasks

### Run Tests

```bash
cargo test
```

### Format Code

```bash
cargo fmt
```

### Check for Issues

```bash
cargo clippy
```

### Build for Production

```bash
cargo build --release
./target/release/chessQ
```

### Update Dependencies

```bash
cargo update
```

## 💡 Tips & Tricks

### Keyboard Shortcuts

- **Esc**: Close modals
- **F11**: Fullscreen mode
- **Ctrl+R**: Refresh board

### Performance

- Use `--release` flag for faster AI
- Close other browser tabs
- Increase search time for stronger AI

### Strategy

- Control the center
- Develop pieces early
- Castle for king safety
- Don't move same piece twice in opening
- Look for tactics (forks, pins, skewers)

## 🆘 Getting Help

### Resources

- **GitHub Issues**: [Report bugs](https://github.com/aidolphin/chessQ/issues)
- **Documentation**: [Technical docs](docs/DOCUMENTATION.md)
- **Contributing**: [How to contribute](CONTRIBUTING.md)

### Community

- Star the repository ⭐
- Watch for updates 👀
- Fork and experiment 🍴
- Share with friends 🔗

## 🎉 Have Fun!

ChessQ is designed to be:
- **Fast**: Rust-powered performance
- **Beautiful**: Premium UI design
- **Educational**: Learn while you play
- **Open**: Fully open-source

Enjoy playing chess! ♟️

---

**Made with ❤️ by Quantum Leaf Automation**

[Back to README](README.md) | [Technical Docs](docs/DOCUMENTATION.md) | [Contributing](CONTRIBUTING.md)
