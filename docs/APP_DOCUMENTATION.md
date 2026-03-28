# ChessQ App Documentation

## Overview

ChessQ is now organized like a real product instead of a single page demo:

- the landing page lives at `/`
- the chess app lives at `/play.html`
- the Rust backend owns chess rules and bot decisions
- the frontend owns presentation, clocks, setup flow, and local persistence

## Current User Experience

Players can:

- open the landing page and go into the game flow
- start a new match with a chosen clock
- play locally against another person
- play against the backend AI bot
- see clocks while the game is running
- share the current setup and board state with a generated link
- use small local profiles and keep recent game history in the browser

## Frontend Responsibilities

The browser app in `web/app.js` handles:

- rendering the board and pieces
- setup modal and profile fields
- player clocks
- promotion picker
- move history
- saved local match history
- share link generation
- bot turn orchestration through Rust API calls

## Backend Responsibilities

The Rust backend in `src/server.rs` handles:

- serving the static web files
- creating new games
- loading FEN positions
- generating legal moves
- applying legal moves
- computing game status
- producing AI moves through the Rust search code

## Routes

### Public Pages

- `/`
  - SEO landing page
- `/play.html`
  - main chess app

### API

- `/api/new`
- `/api/state`
- `/api/legal-moves`
- `/api/move`
- `/api/ai-move`

## SEO Work Added

The landing page now includes:

- title and description metadata
- keywords
- canonical URL
- Open Graph tags
- Twitter summary tags
- JSON-LD structured data
- `robots.txt`
- `sitemap.xml`

## Bot Notes

The AI bot uses the Rust search code in `src/ai/search.rs`.

During this update, the AI search had a real overflow bug caused by using `i32::MIN` and negating it during alpha-beta search. That was fixed by switching to a safe bounded search constant.

## Time Control Notes

Clocks are currently managed on the client side for the single-browser experience.

That is acceptable for:

- local play
- bot play
- shared setup links

It is not enough for:

- authoritative online multiplayer
- anti-cheat timing
- reconnectable sessions

## Share Links

The current share system copies a URL containing:

- mode
- player names
- time control
- AI style
- current FEN

This lets someone else open the same setup and board state.

It is not a full live invite room yet.

## Profiles and Registration

The app currently supports lightweight local profiles through browser storage:

- player names
- preferred mode
- preferred time control
- preferred bot personality
- recent match results

This is not server-backed account registration yet.

## Product Gaps Still Open

The following things are not fully implemented and should not be advertised as complete production features yet:

- real-time multiplayer waiting rooms
- synchronized invite links for two remote players
- server-side accounts and registration
- real reward or money economy
- founder earnings model
- legal/compliance handling for paid competitive play

## Recommended Next Backend Steps

If ChessQ is going to become a serious global product, the next backend milestones should be:

1. Add room/session persistence.
2. Add WebSocket-based real-time multiplayer.
3. Add account registration and authentication.
4. Add server-side clock authority.
5. Add database-backed profile, rewards, and progression.

## Verification

Verified in this iteration:

- `cargo test`
- `node --check web/app.js`
- live Rust API smoke tests for `/api/new` and `/api/ai-move`
