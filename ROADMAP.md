# ChessQ Development Roadmap

This document outlines the planned features and implementation strategy for ChessQ.

## 🎯 Current Status (v0.1.0)

### ✅ Implemented Features

- **Chess Engine**
  - Bitboard representation
  - Magic bitboards for sliding pieces
  - Full chess rules (castling, en passant, promotion)
  - Check, checkmate, stalemate detection
  - FEN notation support

- **AI System**
  - Alpha-beta pruning search
  - Transposition table with Zobrist hashing
  - Quiescence search
  - Position evaluation
  - Three AI personalities

- **Web Interface**
  - Premium UI with black & green theme
  - Responsive design
  - Board flip for black player
  - Move history and notation
  - Time controls

- **Game Modes**
  - Local 2-player
  - Play vs AI
  - Learning mode (9 chapters)

---

## 🚀 Phase 1: Core Improvements (v0.2.0)

**Target: 2-3 months**

### 1.1 PGN Support

**Priority: High**  
**Effort: Medium**

#### Implementation Plan

```rust
// src/pgn/mod.rs
pub struct PGNGame {
    pub event: String,
    pub site: String,
    pub date: String,
    pub white: String,
    pub black: String,
    pub result: String,
    pub moves: Vec<String>,
}

impl PGNGame {
    pub fn from_game_state(states: &[GameState]) -> Self { }
    pub fn to_string(&self) -> String { }
    pub fn from_string(pgn: &str) -> Result<Self, String> { }
}
```

#### API Endpoints

```
GET  /api/pgn/export?game_id={id}
POST /api/pgn/import
```

#### Features
- Export games to PGN format
- Import PGN games
- Parse standard PGN tags
- Support variations and comments
- Validate PGN syntax

#### Testing
- Unit tests for PGN parsing
- Round-trip tests (export → import)
- Test with real PGN databases

---

### 1.2 Opening Book

**Priority: High**  
**Effort: Medium**

#### Implementation Plan

```rust
// src/opening/book.rs
pub struct OpeningBook {
    positions: HashMap<u64, Vec<BookMove>>,
}

pub struct BookMove {
    pub move_uci: String,
    pub weight: u32,
    pub win_rate: f32,
}

impl OpeningBook {
    pub fn load_from_file(path: &str) -> Result<Self, String> { }
    pub fn get_moves(&self, zobrist: u64) -> Vec<BookMove> { }
    pub fn select_move(&self, zobrist: u64) -> Option<String> { }
}
```

#### Data Sources
- Lichess opening database
- Master games database
- Custom curated openings

#### Features
- 10,000+ opening positions
- Weighted move selection
- Win rate statistics
- Opening name detection

---

### 1.3 Enhanced Evaluation

**Priority: Medium**  
**Effort: Medium**

#### Improvements
- King safety evaluation
- Piece coordination
- Outpost detection
- Rook on open files
- Bishop pair bonus
- Knight outposts
- Connected rooks

#### Implementation
```rust
// src/ai/evaluation.rs
pub fn evaluate_advanced(state: &GameState) -> i32 {
    let mut score = 0;
    
    score += material_score(state);
    score += piece_square_score(state);
    score += mobility_score(state);
    score += pawn_structure_score(state);
    score += king_safety_score(state);
    score += piece_coordination_score(state);
    score += control_center_score(state);
    
    score
}
```

---

## 🌐 Phase 2: Multiplayer (v0.3.0)

**Target: 4-6 months**

### 2.1 WebSocket Infrastructure

**Priority: Critical**  
**Effort: High**

#### Architecture

```
┌─────────────┐         WebSocket         ┌─────────────┐
│   Client 1  │◄──────────────────────────►│             │
└─────────────┘                            │   Server    │
                                           │  (Rust)     │
┌─────────────┐         WebSocket         │             │
│   Client 2  │◄──────────────────────────►│  - Rooms    │
└─────────────┘                            │  - Matchmak │
                                           │  - State    │
                                           └─────────────┘
```

#### Implementation

```rust
// src/multiplayer/websocket.rs
use tokio_tungstenite::WebSocketStream;

pub struct GameRoom {
    pub id: String,
    pub white_player: Option<PlayerId>,
    pub black_player: Option<PlayerId>,
    pub state: GameState,
    pub time_control: TimeControl,
}

pub struct MultiplayerServer {
    rooms: HashMap<String, GameRoom>,
    connections: HashMap<PlayerId, WebSocketStream>,
}

impl MultiplayerServer {
    pub async fn handle_connection(&mut self, ws: WebSocketStream) { }
    pub async fn create_room(&mut self, config: RoomConfig) -> String { }
    pub async fn join_room(&mut self, room_id: &str, player: PlayerId) { }
    pub async fn broadcast_move(&self, room_id: &str, mv: &Move) { }
}
```

#### Dependencies

```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
tokio-tungstenite = "0.21"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### Features
- Real-time move synchronization
- Room creation and joining
- Spectator mode
- Reconnection handling
- Latency compensation

---

### 2.2 Matchmaking System

**Priority: High**  
**Effort: Medium**

#### Implementation

```rust
// src/multiplayer/matchmaking.rs
pub struct MatchmakingQueue {
    players: Vec<QueuedPlayer>,
}

pub struct QueuedPlayer {
    pub id: PlayerId,
    pub rating: i32,
    pub time_control: TimeControl,
    pub joined_at: Instant,
}

impl MatchmakingQueue {
    pub fn add_player(&mut self, player: QueuedPlayer) { }
    pub fn find_match(&mut self) -> Option<(PlayerId, PlayerId)> { }
    pub fn remove_player(&mut self, id: &PlayerId) { }
}
```

#### Matching Algorithm
- ELO-based matching (±200 rating)
- Time control preference
- Wait time consideration
- Region-based matching (future)

---

### 2.3 Server-Side Time Control

**Priority: Critical**  
**Effort: Medium**

#### Implementation

```rust
// src/multiplayer/clock.rs
pub struct ServerClock {
    pub white_time: Duration,
    pub black_time: Duration,
    pub increment: Duration,
    pub last_move_time: Instant,
}

impl ServerClock {
    pub fn start(&mut self, color: Color) { }
    pub fn stop(&mut self, color: Color) { }
    pub fn add_increment(&mut self, color: Color) { }
    pub fn is_timeout(&self, color: Color) -> bool { }
}
```

#### Features
- Server-authoritative time
- Automatic timeout detection
- Time synchronization
- Lag compensation

---

## 🔐 Phase 3: Authentication & Profiles (v0.4.0)

**Target: 6-8 months**

### 3.1 User Authentication

**Priority: High**  
**Effort: High**

#### Database Schema

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    last_login TIMESTAMP,
    verified BOOLEAN DEFAULT FALSE
);

CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    token VARCHAR(255) UNIQUE NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
```

#### Implementation

```rust
// src/auth/mod.rs
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

pub struct AuthService {
    db: DatabasePool,
    jwt_secret: String,
}

impl AuthService {
    pub async fn register(&self, username: &str, email: &str, password: &str) -> Result<UserId, AuthError> { }
    pub async fn login(&self, username: &str, password: &str) -> Result<String, AuthError> { }
    pub async fn verify_token(&self, token: &str) -> Result<UserId, AuthError> { }
    pub async fn logout(&self, token: &str) -> Result<(), AuthError> { }
}
```

#### Dependencies

```toml
[dependencies]
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "uuid"] }
argon2 = "0.5"
jsonwebtoken = "9.2"
uuid = { version = "1.6", features = ["v4", "serde"] }
```

#### Features
- Email/password registration
- JWT-based authentication
- Password hashing (Argon2)
- Email verification
- Password reset
- OAuth2 (Google, GitHub) - future

---

### 3.2 Persistent Profiles

**Priority: High**  
**Effort: Medium**

#### Database Schema

```sql
CREATE TABLE profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id),
    display_name VARCHAR(100),
    avatar_url VARCHAR(500),
    bio TEXT,
    country VARCHAR(2),
    rating_bullet INT DEFAULT 1500,
    rating_blitz INT DEFAULT 1500,
    rating_rapid INT DEFAULT 1500,
    rating_classical INT DEFAULT 1500,
    games_played INT DEFAULT 0,
    games_won INT DEFAULT 0,
    games_lost INT DEFAULT 0,
    games_drawn INT DEFAULT 0,
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE game_history (
    id UUID PRIMARY KEY,
    white_player_id UUID REFERENCES users(id),
    black_player_id UUID REFERENCES users(id),
    result VARCHAR(10),
    time_control VARCHAR(20),
    pgn TEXT,
    played_at TIMESTAMP DEFAULT NOW()
);
```

#### API Endpoints

```
GET    /api/profile/{user_id}
PUT    /api/profile/{user_id}
GET    /api/profile/{user_id}/games
GET    /api/profile/{user_id}/stats
POST   /api/profile/{user_id}/avatar
```

---

### 3.3 ELO Rating System

**Priority: High**  
**Effort: Low**

#### Implementation

```rust
// src/rating/elo.rs
pub struct EloCalculator {
    k_factor: f64,
}

impl EloCalculator {
    pub fn calculate_new_ratings(
        &self,
        winner_rating: i32,
        loser_rating: i32,
        is_draw: bool,
    ) -> (i32, i32) {
        let expected_winner = self.expected_score(winner_rating, loser_rating);
        let expected_loser = 1.0 - expected_winner;
        
        let actual_winner = if is_draw { 0.5 } else { 1.0 };
        let actual_loser = if is_draw { 0.5 } else { 0.0 };
        
        let new_winner = winner_rating + (self.k_factor * (actual_winner - expected_winner)) as i32;
        let new_loser = loser_rating + (self.k_factor * (actual_loser - expected_loser)) as i32;
        
        (new_winner, new_loser)
    }
    
    fn expected_score(&self, rating_a: i32, rating_b: i32) -> f64 {
        1.0 / (1.0 + 10_f64.powf((rating_b - rating_a) as f64 / 400.0))
    }
}
```

#### Features
- Separate ratings per time control
- Provisional ratings for new players
- Rating deviation (Glicko-2 future)
- Rating history tracking

---

## 🎮 Phase 4: Advanced Features (v0.5.0)

**Target: 8-12 months**

### 4.1 Tournament System

**Priority: Medium**  
**Effort: High**

#### Tournament Types
- Swiss system
- Round-robin
- Knockout
- Arena (continuous)

#### Implementation

```rust
// src/tournament/mod.rs
pub enum TournamentType {
    Swiss { rounds: u8 },
    RoundRobin,
    Knockout,
    Arena { duration: Duration },
}

pub struct Tournament {
    pub id: String,
    pub name: String,
    pub tournament_type: TournamentType,
    pub participants: Vec<UserId>,
    pub pairings: Vec<Pairing>,
    pub standings: Vec<Standing>,
}
```

---

### 4.2 Puzzle Database

**Priority: Medium**  
**Effort: Medium**

#### Features
- 10,000+ tactical puzzles
- Difficulty ratings
- Themes (fork, pin, skewer, etc.)
- Daily puzzles
- Puzzle rush mode

#### Database Schema

```sql
CREATE TABLE puzzles (
    id UUID PRIMARY KEY,
    fen VARCHAR(100) NOT NULL,
    moves VARCHAR(200) NOT NULL,
    rating INT NOT NULL,
    themes VARCHAR(100)[],
    popularity INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE puzzle_attempts (
    user_id UUID REFERENCES users(id),
    puzzle_id UUID REFERENCES puzzles(id),
    solved BOOLEAN,
    time_taken INT,
    attempted_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (user_id, puzzle_id, attempted_at)
);
```

---

### 4.3 Game Analysis

**Priority: Medium**  
**Effort: High**

#### Features
- Move-by-move analysis
- Blunder detection
- Accuracy calculation
- Best move suggestions
- Opening identification
- Critical positions

#### Implementation

```rust
// src/analysis/mod.rs
pub struct GameAnalysis {
    pub moves: Vec<MoveAnalysis>,
    pub accuracy_white: f32,
    pub accuracy_black: f32,
    pub blunders: Vec<BlunderInfo>,
    pub opening_name: Option<String>,
}

pub struct MoveAnalysis {
    pub move_notation: String,
    pub evaluation: i32,
    pub best_move: Option<String>,
    pub classification: MoveClassification,
}

pub enum MoveClassification {
    Best,
    Excellent,
    Good,
    Inaccuracy,
    Mistake,
    Blunder,
}
```

---

## 🎨 Phase 5: Polish & Expansion (v1.0.0)

**Target: 12-18 months**

### 5.1 Mobile Apps

- React Native or Flutter
- iOS and Android
- Push notifications
- Offline mode

### 5.2 Advanced UI Features

- Multiple board themes
- Piece set customization
- Sound effects
- Move animations
- Board editor
- Analysis board

### 5.3 Social Features

- Friends system
- Chat system
- Clubs/teams
- Leaderboards
- Achievements
- Challenges

### 5.4 4-Player Chess

- Complete implementation
- Special rules
- Team mode
- FFA mode

---

## 📊 Success Metrics

### Technical Metrics
- [ ] 95%+ test coverage
- [ ] <100ms API response time
- [ ] <50ms WebSocket latency
- [ ] 99.9% uptime
- [ ] Support 10,000+ concurrent users

### User Metrics
- [ ] 1,000+ registered users
- [ ] 10,000+ games played
- [ ] 100+ daily active users
- [ ] 4.5+ star rating
- [ ] <5% churn rate

---

## 🛠️ Infrastructure Requirements

### Current (v0.1.0)
- Single Rust server
- Static file serving
- No database

### Phase 2-3 (v0.3.0-v0.4.0)
- PostgreSQL database
- Redis for sessions/cache
- WebSocket server
- Load balancer
- CDN for static assets

### Phase 4-5 (v0.5.0-v1.0.0)
- Kubernetes cluster
- Microservices architecture
- Message queue (RabbitMQ/Kafka)
- Elasticsearch for search
- Monitoring (Prometheus/Grafana)
- CI/CD pipeline

---

## 💰 Monetization Strategy (Future)

### Free Tier
- Unlimited games
- Basic features
- Ads (optional)

### Premium Tier ($5/month)
- Ad-free
- Advanced analysis
- Unlimited puzzles
- Priority matchmaking
- Custom themes
- Tournament hosting

### Pro Tier ($15/month)
- Everything in Premium
- Opening book access
- Game database
- API access
- Private lessons
- Team features

---

## 📝 Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for how to contribute to these features.

---

## 📅 Release Schedule

- **v0.2.0** - Q2 2024 - PGN & Opening Book
- **v0.3.0** - Q3 2024 - Multiplayer
- **v0.4.0** - Q4 2024 - Authentication
- **v0.5.0** - Q1 2025 - Advanced Features
- **v1.0.0** - Q2 2025 - Production Ready

---

**Last Updated**: 2024-01-XX  
**Maintainer**: Quantum Leaf Automation
