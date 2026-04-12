# Contributing to ChessQ

First off, thank you for considering contributing to ChessQ! It's people like you that make ChessQ such a great tool.

## 🎯 Ways to Contribute

### 🐛 Reporting Bugs

Before creating bug reports, please check existing issues. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce** the behavior
- **Expected behavior**
- **Actual behavior**
- **Screenshots** if applicable
- **Environment details** (OS, browser, Rust version)

**Example:**

```markdown
**Bug**: Pieces disappear after castling

**Steps to Reproduce**:
1. Start new game
2. Move pieces to enable castling
3. Castle kingside
4. King disappears from board

**Expected**: King should move to g1
**Actual**: King vanishes
**Browser**: Chrome 120.0
**OS**: macOS 14.0
```

### 💡 Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- **Clear title and description**
- **Use case** - why is this enhancement useful?
- **Proposed solution**
- **Alternative solutions** you've considered
- **Mockups or examples** if applicable

### 📝 Improving Documentation

Documentation improvements are always welcome:

- Fix typos or clarify existing docs
- Add examples or tutorials
- Translate documentation
- Add API documentation
- Create video tutorials

### 🔧 Code Contributions

We love pull requests! Here's how to contribute code:

## 🚀 Getting Started

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/chessQ.git
cd chessQ
git remote add upstream https://github.com/aidolphin/chessQ.git
```

### 2. Create a Branch

```bash
git checkout -b feature/amazing-feature
# or
git checkout -b fix/bug-description
```

**Branch naming conventions:**
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation changes
- `refactor/` - Code refactoring
- `test/` - Adding tests
- `perf/` - Performance improvements

### 3. Set Up Development Environment

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development tools
cargo install cargo-watch
cargo install cargo-edit

# Run the project
cargo run
```

### 4. Make Your Changes

Follow our coding standards (see below).

### 5. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific tests
cargo test engine::bitboard

# Run with output
cargo test -- --nocapture

# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings
```

### 6. Commit Your Changes

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
git commit -m "feat: add opening book support"
git commit -m "fix: resolve castling bug in FEN parser"
git commit -m "docs: update API documentation"
git commit -m "test: add tests for en passant"
```

**Commit types:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation only
- `style:` - Code style changes (formatting)
- `refactor:` - Code refactoring
- `test:` - Adding tests
- `perf:` - Performance improvement
- `chore:` - Maintenance tasks

### 7. Push and Create Pull Request

```bash
git push origin feature/amazing-feature
```

Then create a Pull Request on GitHub.

## 📋 Pull Request Guidelines

### PR Title

Use conventional commit format:

```
feat: add transposition table pruning
fix: resolve memory leak in search
docs: add contributing guidelines
```

### PR Description

Include:

```markdown
## Description
Brief description of changes

## Motivation
Why is this change needed?

## Changes Made
- Added X feature
- Fixed Y bug
- Updated Z documentation

## Testing
- [ ] All tests pass
- [ ] Added new tests
- [ ] Manual testing completed

## Screenshots (if applicable)
[Add screenshots here]

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings
```

### Review Process

1. **Automated checks** must pass (tests, linting)
2. **Code review** by maintainers
3. **Changes requested** if needed
4. **Approval** and merge

## 💻 Coding Standards

### Rust Code Style

```rust
// ✅ Good
pub fn calculate_mobility(board: &Board, color: Color) -> i32 {
    let moves = generate_legal_moves(board, color);
    moves.len() as i32
}

// ❌ Bad
pub fn calc_mob(b:&Board,c:Color)->i32{
    let m=gen_moves(b,c);
    m.len()as i32
}
```

**Guidelines:**
- Use `cargo fmt` for formatting
- Follow Rust naming conventions
- Add documentation comments for public APIs
- Keep functions small and focused
- Use descriptive variable names
- Avoid unwrap() - use proper error handling

### JavaScript Code Style

```javascript
// ✅ Good
async function makeMove(from, to, promotion = null) {
    try {
        const response = await fetch(buildMoveUrl(from, to, promotion));
        const data = await response.json();
        return handleMoveResponse(data);
    } catch (error) {
        console.error('Move failed:', error);
        return false;
    }
}

// ❌ Bad
async function mv(f,t,p){
    let r=await fetch(`/api/move?fen=${fen}&from=${f}&to=${t}`);
    return await r.json();
}
```

**Guidelines:**
- Use modern ES6+ syntax
- Use async/await over callbacks
- Add JSDoc comments for functions
- Use meaningful variable names
- Handle errors gracefully
- Keep functions pure when possible

### CSS Code Style

```css
/* ✅ Good */
.chess-board {
    display: grid;
    grid-template-columns: repeat(8, 1fr);
    aspect-ratio: 1;
    border: 3px solid var(--green-emerald);
}

/* ❌ Bad */
.cb{display:grid;grid-template-columns:repeat(8,1fr);aspect-ratio:1;border:3px solid #10b981}
```

**Guidelines:**
- Use CSS variables for colors
- Group related properties
- Use meaningful class names
- Add comments for complex styles
- Follow BEM naming when appropriate

## 🧪 Testing Guidelines

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_move_generation() {
        let board = Board::from_fen("8/8/8/8/8/8/P7/8 w - - 0 1").unwrap();
        let moves = generate_pawn_moves(&board, Square::A2);
        
        assert_eq!(moves.len(), 2); // a3 and a4
        assert!(moves.contains(&Square::A3));
        assert!(moves.contains(&Square::A4));
    }

    #[test]
    fn test_castling_blocked() {
        let board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        let moves = generate_king_moves(&board, Square::E1);
        
        // Castling should be blocked by pieces
        assert!(!moves.contains(&Square::G1));
        assert!(!moves.contains(&Square::C1));
    }
}
```

### Test Coverage

Aim for:
- **Unit tests**: Test individual functions
- **Integration tests**: Test API endpoints
- **Edge cases**: Test boundary conditions
- **Performance tests**: Benchmark critical paths

## 🎨 UI/UX Contributions

### Design Principles

1. **Consistency**: Follow existing design patterns
2. **Accessibility**: Support keyboard navigation and screen readers
3. **Responsiveness**: Test on mobile, tablet, and desktop
4. **Performance**: Optimize animations and rendering
5. **Clarity**: Clear visual hierarchy and feedback

### Adding UI Features

```javascript
// 1. Add HTML structure
// 2. Add CSS styling
// 3. Add JavaScript behavior
// 4. Test on multiple devices
// 5. Ensure accessibility
```

## 🐛 Debugging Tips

### Backend Debugging

```rust
// Add debug logging
use log::{debug, info, warn, error};

debug!("Generated {} moves", moves.len());
info!("Search completed at depth {}", depth);
warn!("Transposition table full");
error!("Invalid FEN: {}", fen);
```

### Frontend Debugging

```javascript
// Enable debug mode
const DEBUG = true;

function debugLog(...args) {
    if (DEBUG) console.log('[ChessQ]', ...args);
}

debugLog('Move made:', from, to);
debugLog('Game state:', gameState);
```

## 📚 Resources

### Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Chess Programming Wiki](https://www.chessprogramming.org/)
- [Bitboard Basics](https://www.chessprogramming.org/Bitboards)
- [Alpha-Beta Pruning](https://www.chessprogramming.org/Alpha-Beta)

### ChessQ Specific

- [Architecture Overview](docs/DOCUMENTATION.md#architecture-overview)
- [API Reference](docs/DOCUMENTATION.md#api-reference)
- [Engine Details](docs/DOCUMENTATION.md#chess-engine)

## 🏆 Recognition

Contributors will be:
- Listed in README.md
- Mentioned in release notes
- Given credit in commit history

## 📞 Getting Help

- **GitHub Issues**: For bugs and features
- **GitHub Discussions**: For questions and ideas
- **Code Review**: Tag maintainers in PRs

## 📜 Code of Conduct

### Our Pledge

We pledge to make participation in our project a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, gender identity, level of experience, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards

**Positive behavior:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards others

**Unacceptable behavior:**
- Trolling, insulting/derogatory comments
- Public or private harassment
- Publishing others' private information
- Other conduct which could reasonably be considered inappropriate

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported by contacting the project team. All complaints will be reviewed and investigated promptly and fairly.

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

## 🎉 Thank You!

Your contributions make ChessQ better for everyone. We appreciate your time and effort!

**Happy Coding! ♟️**
