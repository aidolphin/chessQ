# ChessQ 
## By Quantum Leaf Automation

ChessQ is a Rust-backed browser chess product with:

- an SEO landing page at `/`
- the playable chess app at `/play.html`
- Rust move validation and AI endpoints
- timed matches, bot play, shareable links, and local player profiles

## Product Structure

- `web/index.html`
  - landing page for SEO and product messaging
- `web/play.html`
  - actual chess application
- `web/app.js`
  - browser game client, clocks, setup modal, profiles, bot flow, history
- `web/styles.css`
  - game UI styling
- `web/landing.css`
  - landing page styling
- `src/server.rs`
  - Rust HTTP server and chess API
- `src/engine/*`
  - Rust chess engine
- `src/ai/*`
  - Rust bot search and evaluation
- `src/bin/chessq-cli.rs`
  - optional terminal app

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

- Rust-powered legal move validation
- check, checkmate, stalemate, castling, en passant, promotion
- new game setup modal with time controls
- visible clocks for both sides
- bot play through `/api/ai-move`
- shareable match links
- local player names and recent match history
- FEN import/export
- SEO-oriented landing page with metadata, Open Graph tags, `robots.txt`, and `sitemap.xml`

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
cargo test
```

Frontend syntax check:

```bash
node --check web/app.js
```

## Honest Current Limits

The current share link system shares setup and board state. It is not full live multiplayer with synchronized waiting rooms yet.

The profile system is local to the browser through `localStorage`. It is not full server-side account registration yet.

The reward economy is not implemented as real money or real balances. That needs backend persistence, auth, and product/legal design before it can be shipped honestly.

## Documentation

Detailed notes are in:

- `docs/APP_DOCUMENTATION.md`
