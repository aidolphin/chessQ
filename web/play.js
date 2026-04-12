// ChessQ Premium Play - Complete Game Logic
// High-Performance Chess Environment with Economy System

// === Configuration ===
const CONFIG = {
  API_BASE: '/api',
  GIFT_TO_CASH_RATE: 100, // 100 gifts = $1.00
  HOLDING_PERIOD_DAYS: 45,
  CHECKMATE_CASH: 5.00,
  DRAW_GIFTS: 50,
  WIN_GIFTS: 100,
  LOSS_GIFTS: 25
};

// === Game State ===
let gameState = {
  mode: null,
  timeControl: null,
  selectedTime: null,
  board: null,
  fen: null,
  turn: 'w',
  selectedSquare: null,
  legalMoves: [],
  moveHistory: [],
  whiteTime: 600000,
  blackTime: 600000,
  clockInterval: null,
  gameActive: false,
  playingAgainstAI: false,
  playerColor: 'w',
  aiPersonality: 'aggressive'
};

// === Economy State ===
let economyState = {
  availableBalance: 127.50,
  giftsEarned: 450,
  pendingBalance: 85.00,
  daysRemaining: 32,
  level: 12,
  giftsToNextLevel: 1000,
  currentGifts: 650,
  eloRating: 1720,
  lastEloChange: 15
};

// === Player Profile ===
let playerProfile = {
  username: 'Guest',
  email: '',
  avatar: 'YO',
  gamesPlayed: 0,
  gamesWon: 0,
  gamesLost: 0,
  gamesDraw: 0,
  createdAt: Date.now()
};

// === Load Profile from localStorage ===
function loadProfile() {
  const saved = localStorage.getItem('chessq_profile');
  if (saved) {
    try {
      playerProfile = JSON.parse(saved);
    } catch (e) {
      console.error('Failed to load profile:', e);
    }
  }
  
  const savedEconomy = localStorage.getItem('chessq_economy');
  if (savedEconomy) {
    try {
      economyState = JSON.parse(savedEconomy);
    } catch (e) {
      console.error('Failed to load economy:', e);
    }
  }
}

// === Save Profile to localStorage ===
function saveProfile() {
  localStorage.setItem('chessq_profile', JSON.stringify(playerProfile));
  localStorage.setItem('chessq_economy', JSON.stringify(economyState));
}

// === Show Profile Modal ===
function showProfileModal() {
  const modal = document.createElement('div');
  modal.className = 'profile-modal';
  modal.innerHTML = `
    <div class="modal-overlay" onclick="this.parentElement.remove()"></div>
    <div class="modal-content">
      <div class="modal-header">
        <h2>Your Profile</h2>
        <button class="modal-close" onclick="this.closest('.profile-modal').remove()">&times;</button>
      </div>
      <div class="profile-content">
        <div class="profile-form">
          <div class="form-group">
            <label>Username</label>
            <input type="text" id="profileUsername" value="${playerProfile.username}" maxlength="20">
          </div>
          <div class="form-group">
            <label>Email (optional)</label>
            <input type="email" id="profileEmail" value="${playerProfile.email}" placeholder="your@email.com">
          </div>
          <div class="form-group">
            <label>Avatar (2 letters)</label>
            <input type="text" id="profileAvatar" value="${playerProfile.avatar}" maxlength="2">
          </div>
        </div>
        
        <div class="profile-stats">
          <h3>Statistics</h3>
          <div class="stat-grid">
            <div class="stat-item">
              <span class="stat-label">Games Played</span>
              <span class="stat-value">${playerProfile.gamesPlayed}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Wins</span>
              <span class="stat-value">${playerProfile.gamesWon}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Losses</span>
              <span class="stat-value">${playerProfile.gamesLost}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Draws</span>
              <span class="stat-value">${playerProfile.gamesDraw}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Win Rate</span>
              <span class="stat-value">${playerProfile.gamesPlayed > 0 ? Math.round((playerProfile.gamesWon / playerProfile.gamesPlayed) * 100) : 0}%</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">ELO Rating</span>
              <span class="stat-value">${economyState.eloRating}</span>
            </div>
          </div>
        </div>
        
        <div class="profile-actions">
          <button class="btn-primary" onclick="saveProfileChanges()">Save Changes</button>
          <button class="btn-secondary" onclick="this.closest('.profile-modal').remove()">Cancel</button>
        </div>
      </div>
    </div>
  `;
  document.body.appendChild(modal);
}

// === Save Profile Changes ===
function saveProfileChanges() {
  const username = document.getElementById('profileUsername').value.trim();
  const email = document.getElementById('profileEmail').value.trim();
  const avatar = document.getElementById('profileAvatar').value.trim().toUpperCase();
  
  if (username) playerProfile.username = username;
  if (avatar.length === 2) playerProfile.avatar = avatar;
  playerProfile.email = email;
  
  saveProfile();
  updateProfileDisplay();
  showNotification('Profile Saved', 'Your profile has been updated!');
  document.querySelector('.profile-modal')?.remove();
}

// === Chess Piece Unicode ===
const PIECES = {
  'wp': '♙', 'wn': '♘', 'wb': '♗', 'wr': '♖', 'wq': '♕', 'wk': '♔',
  'bp': '♟', 'bn': '♞', 'bb': '♝', 'br': '♜', 'bq': '♛', 'bk': '♚'
};

// === DOM Elements ===
const elements = {
  modeSelection: document.getElementById('modeSelection'),
  timeControlModal: document.getElementById('timeControlModal'),
  opponentModal: document.getElementById('opponentModal'),
  colorModal: document.getElementById('colorModal'),
  boardContainer: document.getElementById('boardContainer'),
  chessBoard: document.getElementById('chessBoard'),
  learningContainer: document.getElementById('learningContainer'),
  chatPanel: document.getElementById('chatPanel'),
  movesList: document.getElementById('movesList'),
  clockTop: document.getElementById('clockTop'),
  clockBottom: document.getElementById('clockBottom'),
  notificationToast: document.getElementById('notificationToast'),
  shareModal: document.getElementById('shareModal'),
  resultModal: document.getElementById('resultModal')
};

// === Initialize ===
document.addEventListener('DOMContentLoaded', () => {
  loadProfile();
  loadNotifications();
  initializeGame();
  setupEventListeners();
  updateEconomyDisplay();
  updateRankingDisplay();
  updateProfileDisplay();
});

function initializeGame() {
  console.log('ChessQ Premium Play initialized');
  console.log('Welcome,', playerProfile.username);
  
  // Show mode selection by default
  showModeSelection();
}

// === Update Profile Display ===
function updateProfileDisplay() {
  // Update user avatar in navbar
  const userAvatar = document.querySelector('.user-avatar span');
  if (userAvatar) {
    userAvatar.textContent = playerProfile.avatar;
  }
  
  // Update player info in game
  const playerNameEl = document.querySelector('.player-bottom .player-name');
  if (playerNameEl) {
    playerNameEl.textContent = playerProfile.username;
  }
  
  const playerAvatarEl = document.querySelector('.player-bottom .player-avatar');
  if (playerAvatarEl) {
    playerAvatarEl.textContent = playerProfile.avatar;
  }
  
  // Update win rate
  const playerRatingEl = document.querySelector('.player-bottom .player-rating');
  if (playerRatingEl && playerProfile.gamesPlayed > 0) {
    const winRate = Math.round((playerProfile.gamesWon / playerProfile.gamesPlayed) * 100);
    playerRatingEl.textContent = `${economyState.eloRating} • ${winRate}% Win`;
  }
}

// === Mode Selection ===
function showModeSelection() {
  elements.modeSelection.classList.remove('hidden');
  elements.boardContainer.classList.add('hidden');
  elements.learningContainer.classList.add('hidden');
}

// === Event Listeners ===
function setupEventListeners() {
  // Mode cards
  document.querySelectorAll('.mode-card').forEach(card => {
    card.addEventListener('click', () => {
      const mode = card.dataset.mode;
      handleModeSelection(mode);
    });
  });
  
  // Time option selection
  document.querySelectorAll('.time-option').forEach(option => {
    option.addEventListener('click', () => {
      // Remove selected class from all options
      document.querySelectorAll('.time-option').forEach(opt => {
        opt.classList.remove('selected');
      });
      
      // Add selected class to clicked option
      option.classList.add('selected');
      
      // Store selected time
      gameState.selectedTime = {
        minutes: parseInt(option.dataset.minutes),
        increment: parseInt(option.dataset.increment)
      };
      
      // Enable Start Game button
      document.getElementById('startGameBtn').disabled = false;
    });
  });
  
  // Start Game button
  document.getElementById('startGameBtn')?.addEventListener('click', () => {
    if (gameState.selectedTime) {
      startGame(gameState.selectedTime.minutes, gameState.selectedTime.increment);
    }
  });
  
  // Opponent selection
  document.querySelectorAll('.opponent-card').forEach(card => {
    card.addEventListener('click', () => {
      const opponent = card.dataset.opponent;
      handleOpponentSelection(opponent);
    });
  });
  
  // Color selection
  document.querySelectorAll('.color-card').forEach(card => {
    card.addEventListener('click', () => {
      const color = card.dataset.color;
      handleColorSelection(color);
    });
  });
  
  // Close modals
  document.getElementById('closeTimeControl')?.addEventListener('click', closeTimeControlModal);
  document.getElementById('timeControlOverlay')?.addEventListener('click', closeTimeControlModal);
  
  document.getElementById('closeOpponent')?.addEventListener('click', closeOpponentModal);
  document.getElementById('opponentOverlay')?.addEventListener('click', closeOpponentModal);
  
  document.getElementById('closeColor')?.addEventListener('click', closeColorModal);
  document.getElementById('colorOverlay')?.addEventListener('click', closeColorModal);
  
  // Game controls
  document.getElementById('fullscreenBtn')?.addEventListener('click', toggleFullscreen);
  document.getElementById('shareBtn')?.addEventListener('click', showShareModal);
  document.getElementById('resignBtn')?.addEventListener('click', resignGame);
  document.getElementById('drawBtn')?.addEventListener('click', offerDraw);
  document.getElementById('undoBtn')?.addEventListener('click', requestUndo);
  
  // Chat
  document.getElementById('chatToggle')?.addEventListener('click', toggleChat);
  document.getElementById('chatClose')?.addEventListener('click', toggleChat);
  document.getElementById('sendMessage')?.addEventListener('click', sendChatMessage);
  document.getElementById('chatInput')?.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') sendChatMessage();
  });
  
  // Share modal
  document.getElementById('closeShare')?.addEventListener('click', closeShareModal);
  document.getElementById('shareOverlay')?.addEventListener('click', closeShareModal);
  document.getElementById('copyLink')?.addEventListener('click', copyShareLink);
  
  // Learning mode
  document.getElementById('backToModes')?.addEventListener('click', showModeSelection);
  
  // Withdraw
  document.getElementById('withdrawBtn')?.addEventListener('click', handleWithdraw);
  
  // Profile
  document.getElementById('userMenuBtn')?.addEventListener('click', showProfileModal);
  
  // Notification close
  document.getElementById('notificationBtn')?.addEventListener('click', toggleNotificationPanel);
  document.getElementById('notifClose')?.addEventListener('click', () => {
    document.getElementById('notificationPanel').classList.add('hidden');
  });
  document.getElementById('notifMarkAllRead')?.addEventListener('click', markAllNotificationsRead);
  
  // Toast close
  document.getElementById('toastClose')?.addEventListener('click', () => {
    elements.notificationToast.classList.add('hidden');
  });
}

// === Mode Selection Handler ===
function handleModeSelection(mode) {
  gameState.mode = mode;
  
  if (mode === 'learning') {
    showLearningMode();
  } else if (mode === 'custom') {
    showCustomChallenge();
  } else if (mode === 'standard') {
    // Show opponent selection modal
    showOpponentModal();
  } else {
    // Show time control selection
    elements.timeControlModal.classList.remove('hidden');
  }
}

function showOpponentModal() {
  elements.opponentModal.classList.remove('hidden');
}

function closeOpponentModal() {
  elements.opponentModal.classList.add('hidden');
}

function handleOpponentSelection(opponent) {
  closeOpponentModal();
  
  if (opponent === 'ai') {
    gameState.playingAgainstAI = true;
    // Show color selection modal
    showColorModal();
  } else {
    gameState.playingAgainstAI = false;
    gameState.playerColor = 'w'; // Default to white for human vs human
    showNotification('Human Opponent', 'Share the game link with your friend!');
    // Show time control selection
    elements.timeControlModal.classList.remove('hidden');
  }
}

function showColorModal() {
  elements.colorModal.classList.remove('hidden');
}

function closeColorModal() {
  elements.colorModal.classList.add('hidden');
}

function handleColorSelection(color) {
  closeColorModal();
  
  if (color === 'random') {
    gameState.playerColor = Math.random() < 0.5 ? 'w' : 'b';
  } else {
    gameState.playerColor = color;
  }
  
  const colorName = gameState.playerColor === 'w' ? 'White' : 'Black';
  showNotification('AI Opponent', `You are playing as ${colorName} against AI`);
  
  // Show time control selection
  elements.timeControlModal.classList.remove('hidden');
}

function closeTimeControlModal() {
  elements.timeControlModal.classList.add('hidden');
  // Reset selection
  document.querySelectorAll('.time-option').forEach(opt => {
    opt.classList.remove('selected');
  });
  gameState.selectedTime = null;
  document.getElementById('startGameBtn').disabled = true;
}

function showLearningMode() {
  elements.modeSelection.classList.add('hidden');
  elements.learningContainer.classList.remove('hidden');
  learnShowChapterSelect();
}

function showCustomChallenge() {
  showNotification('Custom Challenge', 'Custom challenge creator coming soon!');
}

// === Start Game ===
async function startGame(minutes, increment) {
  elements.timeControlModal.classList.add('hidden');
  elements.modeSelection.classList.add('hidden');
  elements.boardContainer.classList.remove('hidden');
  
  // Initialize times
  gameState.whiteTime = minutes * 60 * 1000;
  gameState.blackTime = minutes * 60 * 1000;
  gameState.timeControl = { minutes, increment };
  gameState.gameActive = true;
  
  // Initialize board
  await initializeBoard();
  
  // Start clock
  startClock();
  
  showNotification('Game Started', 'Good luck!');
  
  // If playing as black against AI, let AI make first move
  if (gameState.playingAgainstAI && gameState.playerColor === 'b') {
    setTimeout(() => makeAIMove(), 1000);
  }
}

// === Board Initialization ===
async function initializeBoard() {
  try {
    // Fetch initial game state from API
    const response = await fetch(`${CONFIG.API_BASE}/new`);
    const data = await response.json();
    
    gameState.board = data.board;
    gameState.fen = data.fen;
    gameState.turn = data.turn;
    
    renderBoard();
  } catch (error) {
    console.error('Failed to initialize board:', error);
    showNotification('Error', 'Failed to start game. Please try again.');
  }
}

// === Board Rendering ===
// API board array: index 0-7 = rank 8, 56-63 = rank 1
// When playing as black, flip the board so black pieces are at the bottom
function renderBoard() {
  const board = elements.chessBoard;
  board.innerHTML = '';

  const files = 'abcdefgh';
  const playingAsBlack = gameState.playerColor === 'b';

  // Ranks and files from the player's perspective
  const ranks = playingAsBlack ? [1,2,3,4,5,6,7,8] : [8,7,6,5,4,3,2,1];
  const fileIndices = playingAsBlack ? [7,6,5,4,3,2,1,0] : [0,1,2,3,4,5,6,7];

  // Update coordinate labels
  const leftCoords = document.querySelector('.board-coords-left');
  const rightCoords = document.querySelector('.board-coords-right');
  const bottomCoords = document.querySelector('.board-coords-bottom');
  if (leftCoords) leftCoords.innerHTML = ranks.map(r => `<span>${r}</span>`).join('');
  if (rightCoords) rightCoords.innerHTML = ranks.map(r => `<span>${r}</span>`).join('');
  if (bottomCoords) bottomCoords.innerHTML = fileIndices.map(f => `<span>${files[f]}</span>`).join('');

  for (const rank of ranks) {
    for (const fileIndex of fileIndices) {
      const file = files[fileIndex];
      const coord = `${file}${rank}`;

      // API sends board from rank 8 to rank 1
      const boardIndex = (8 - rank) * 8 + fileIndex;

      const square = document.createElement('div');
      square.className = 'square';
      square.dataset.square = coord;
      square.dataset.index = boardIndex;

      const isLight = (fileIndex + rank) % 2 === 0;
      square.classList.add(isLight ? 'light' : 'dark');

      const piece = gameState.board[boardIndex];
      if (piece) {
        const pieceElement = document.createElement('span');
        pieceElement.className = `piece ${piece[0] === 'w' ? 'white' : 'black'}`;
        pieceElement.textContent = PIECES[piece];
        square.appendChild(pieceElement);
      }

      square.addEventListener('click', () => handleSquareClick(coord, boardIndex));
      board.appendChild(square);
    }
  }
}

// === Square Click Handler ===
async function handleSquareClick(coord, index) {
  if (!gameState.gameActive) return;
  
  // If playing against AI, only allow moves when it's player's turn
  if (gameState.playingAgainstAI && gameState.turn !== gameState.playerColor) {
    showNotification('Wait', "It's AI's turn!");
    return;
  }
  
  const piece = gameState.board[index];
  
  // If no square selected
  if (!gameState.selectedSquare) {
    if (piece && piece[0] === gameState.turn) {
      gameState.selectedSquare = coord;
      await fetchLegalMoves(coord);
      highlightSquare(coord);
      highlightLegalMoves();
    }
    return;
  }
  
  // If clicking same square, deselect
  if (gameState.selectedSquare === coord) {
    clearHighlights();
    gameState.selectedSquare = null;
    gameState.legalMoves = [];
    return;
  }
  
  // If clicking another own piece, select it
  if (piece && piece[0] === gameState.turn) {
    clearHighlights();
    gameState.selectedSquare = coord;
    await fetchLegalMoves(coord);
    highlightSquare(coord);
    highlightLegalMoves();
    return;
  }
  
  // Try to make move
  const isLegal = gameState.legalMoves.some(m => m.to === coord);
  if (isLegal) {
    await makeMove(gameState.selectedSquare, coord);
  }
  
  clearHighlights();
  gameState.selectedSquare = null;
  gameState.legalMoves = [];
}

// === Fetch Legal Moves ===
async function fetchLegalMoves(from) {
  try {
    const response = await fetch(`${CONFIG.API_BASE}/legal-moves?fen=${encodeURIComponent(gameState.fen)}&from=${from}`);
    const data = await response.json();
    gameState.legalMoves = data.moves || [];
  } catch (error) {
    console.error('Failed to fetch legal moves:', error);
  }
}

// === Make Move ===
async function makeMove(from, to) {
  try {
    const response = await fetch(`${CONFIG.API_BASE}/move?fen=${encodeURIComponent(gameState.fen)}&from=${from}&to=${to}`);
    const data = await response.json();
    
    gameState.board = data.snapshot.board;
    gameState.fen = data.snapshot.fen;
    gameState.turn = data.snapshot.turn;
    gameState.moveHistory.push(data.moveNotation);
    
    renderBoard();
    updateMoveHistory();
    
    console.log('Move made:', data.moveNotation, 'Status:', data.snapshot.status);
    
    // Check for game over
    if (data.snapshot.gameOver) {
      console.log('Game over detected:', data.snapshot.status);
      endGame(data.snapshot.status);
      return;
    }
    
    // Switch clock
    switchClock();
    
    // If playing against AI and it's AI's turn, make AI move
    if (gameState.playingAgainstAI && gameState.turn !== gameState.playerColor) {
      setTimeout(() => makeAIMove(), 500); // Small delay for better UX
    }
    
  } catch (error) {
    console.error('Failed to make move:', error);
    showNotification('Error', 'Invalid move');
  }
}

// === AI Move ===
async function makeAIMove() {
  if (!gameState.gameActive) return;
  
  try {
    showNotification('AI Thinking...', 'Please wait');
    
    const response = await fetch(
      `${CONFIG.API_BASE}/ai-move?fen=${encodeURIComponent(gameState.fen)}&personality=${gameState.aiPersonality}&ms=1000`
    );
    const data = await response.json();
    
    gameState.board = data.snapshot.board;
    gameState.fen = data.snapshot.fen;
    gameState.turn = data.snapshot.turn;
    gameState.moveHistory.push(data.moveNotation);
    
    renderBoard();
    updateMoveHistory();
    
    showNotification('AI Moved', data.moveNotation);
    
    console.log('AI move made:', data.moveNotation, 'Status:', data.snapshot.status);
    
    // Check for game over
    if (data.snapshot.gameOver) {
      console.log('Game over detected after AI move:', data.snapshot.status);
      endGame(data.snapshot.status);
      return;
    }
    
    // Switch clock
    switchClock();
    
  } catch (error) {
    console.error('Failed to get AI move:', error);
    showNotification('Error', 'AI failed to move');
  }
}

// === Highlight Functions ===
function highlightSquare(coord) {
  const square = document.querySelector(`[data-square="${coord}"]`);
  if (square) square.classList.add('selected');
}

function highlightLegalMoves() {
  gameState.legalMoves.forEach(move => {
    const square = document.querySelector(`[data-square="${move.to}"]`);
    if (square) square.classList.add('highlight');
  });
}

function clearHighlights() {
  document.querySelectorAll('.square').forEach(sq => {
    sq.classList.remove('selected', 'highlight');
  });
}

// === Clock Management ===
function startClock() {
  if (gameState.clockInterval) clearInterval(gameState.clockInterval);
  
  gameState.clockInterval = setInterval(() => {
    if (!gameState.gameActive) return;
    
    if (gameState.turn === 'w') {
      gameState.whiteTime -= 100;
      if (gameState.whiteTime <= 0) {
        gameState.whiteTime = 0;
        endGame({ phase: 'timeout', winner: 'b', message: 'Black wins on time' });
      }
    } else {
      gameState.blackTime -= 100;
      if (gameState.blackTime <= 0) {
        gameState.blackTime = 0;
        endGame({ phase: 'timeout', winner: 'w', message: 'White wins on time' });
      }
    }
    
    updateClockDisplay();
  }, 100);
}

function switchClock() {
  // Add increment if applicable
  if (gameState.timeControl.increment > 0) {
    const increment = gameState.timeControl.increment * 1000;
    if (gameState.turn === 'b') {
      gameState.whiteTime += increment;
    } else {
      gameState.blackTime += increment;
    }
  }
}

function updateClockDisplay() {
  if (elements.clockTop) {
    elements.clockTop.querySelector('.clock-time').textContent = formatTime(gameState.blackTime);
  }
  if (elements.clockBottom) {
    elements.clockBottom.querySelector('.clock-time').textContent = formatTime(gameState.whiteTime);
  }
  
  // Update active clock
  elements.clockTop?.classList.toggle('active', gameState.turn === 'b');
  elements.clockBottom?.classList.toggle('active', gameState.turn === 'w');
}

function formatTime(ms) {
  const totalSeconds = Math.floor(ms / 1000);
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes}:${seconds.toString().padStart(2, '0')}`;
}

// === Move History ===
function updateMoveHistory() {
  const list = elements.movesList;
  list.innerHTML = '';
  
  for (let i = 0; i < gameState.moveHistory.length; i += 2) {
    const row = document.createElement('div');
    row.className = 'move-row';
    
    const moveNum = document.createElement('span');
    moveNum.className = 'move-number';
    moveNum.textContent = `${Math.floor(i / 2) + 1}.`;
    
    const whiteMove = document.createElement('span');
    whiteMove.textContent = gameState.moveHistory[i] || '';
    
    const blackMove = document.createElement('span');
    blackMove.textContent = gameState.moveHistory[i + 1] || '';
    
    row.appendChild(moveNum);
    row.appendChild(whiteMove);
    row.appendChild(blackMove);
    list.appendChild(row);
  }
  
  list.scrollTop = list.scrollHeight;
}

// === Game End ===
function endGame(status) {
  console.log('=== GAME ENDED ===');
  console.log('Status:', status);
  console.log('Winner:', status.winner);
  console.log('Message:', status.message);
  
  gameState.gameActive = false;
  if (gameState.clockInterval) {
    clearInterval(gameState.clockInterval);
  }
  
  // Update game statistics
  playerProfile.gamesPlayed++;
  let gameResult = '';
  if (status.phase === 'checkmate') {
    if (status.winner === gameState.playerColor) {
      playerProfile.gamesWon++;
      gameResult = 'won';
    } else {
      playerProfile.gamesLost++;
      gameResult = 'lost';
    }
  } else if (status.phase === 'stalemate') {
    playerProfile.gamesDraw++;
    gameResult = 'draw';
  } else if (status.phase === 'resignation') {
    if (status.winner === gameState.playerColor) {
      playerProfile.gamesWon++;
      gameResult = 'won';
    } else {
      playerProfile.gamesLost++;
      gameResult = 'lost';
    }
  } else if (status.phase === 'timeout') {
    if (status.winner === gameState.playerColor) {
      playerProfile.gamesWon++;
      gameResult = 'won';
    } else {
      playerProfile.gamesLost++;
      gameResult = 'lost';
    }
  }
  
  // Calculate rewards
  const rewards = calculateRewards(status);
  
  // Update economy
  updateEconomy(rewards);
  
  // Add notification
  if (gameResult === 'won') {
    addNotification('game', 'Victory!', `You won! Earned $${rewards.cash.toFixed(2)} and ${rewards.gifts} gifts`);
  } else if (gameResult === 'lost') {
    addNotification('game', 'Defeat', `You lost. Earned ${rewards.gifts} gifts for playing`);
  } else {
    addNotification('game', 'Draw', `Game ended in a draw. Earned ${rewards.gifts} gifts`);
  }
  
  // Save profile
  saveProfile();
  updateProfileDisplay();
  
  // Show result modal
  showResultModal(status, rewards);
  
  // Show notification
  showNotification('Game Over', status.message);
  
  // Show analysis
  document.getElementById('analysisSection')?.classList.remove('hidden');
}

function calculateRewards(status) {
  const rewards = {
    cash: 0,
    gifts: 0,
    eloChange: 0
  };
  
  console.log('Calculating rewards for status:', status);
  console.log('Player color:', gameState.playerColor);
  
  if (status.phase === 'checkmate') {
    // Check if player won
    const playerWon = status.winner === gameState.playerColor;
    if (playerWon) {
      rewards.cash = CONFIG.CHECKMATE_CASH;
      rewards.eloChange = 15;
    } else {
      rewards.gifts = CONFIG.LOSS_GIFTS;
      rewards.eloChange = -15;
    }
  } else if (status.phase === 'stalemate') {
    // Draw
    rewards.gifts = CONFIG.DRAW_GIFTS;
    rewards.eloChange = 0;
  } else if (status.phase === 'timeout') {
    const playerWon = status.winner === gameState.playerColor;
    if (playerWon) {
      rewards.cash = CONFIG.CHECKMATE_CASH;
      rewards.eloChange = 15;
    } else {
      rewards.gifts = CONFIG.LOSS_GIFTS;
      rewards.eloChange = -15;
    }
  } else if (status.phase === 'resignation') {
    const playerWon = status.winner === gameState.playerColor;
    if (playerWon) {
      rewards.cash = CONFIG.CHECKMATE_CASH;
      rewards.eloChange = 15;
    } else {
      rewards.gifts = CONFIG.LOSS_GIFTS;
      rewards.eloChange = -15;
    }
  }
  
  console.log('Calculated rewards:', rewards);
  return rewards;
}

function updateEconomy(rewards) {
  economyState.availableBalance += rewards.cash;
  economyState.giftsEarned += rewards.gifts;
  economyState.currentGifts += rewards.gifts;
  economyState.eloRating += rewards.eloChange;
  economyState.lastEloChange = rewards.eloChange;
  
  // Check level up
  while (economyState.currentGifts >= economyState.giftsToNextLevel) {
    economyState.currentGifts -= economyState.giftsToNextLevel;
    economyState.level++;
    economyState.giftsToNextLevel = Math.floor(economyState.giftsToNextLevel * 1.5);
    showNotification('Level Up!', `You reached Level ${economyState.level}!`);
  }
  
  updateEconomyDisplay();
  updateRankingDisplay();
}

// === Economy Display ===
function updateEconomyDisplay() {
  document.getElementById('availableBalance').textContent = `$${economyState.availableBalance.toFixed(2)}`;
  document.getElementById('giftsEarned').textContent = economyState.giftsEarned;
  document.getElementById('pendingBalance').textContent = `$${economyState.pendingBalance.toFixed(2)}`;
  document.getElementById('daysRemaining').textContent = economyState.daysRemaining;
}

function updateRankingDisplay() {
  document.getElementById('playerLevel').textContent = `Level ${economyState.level}`;
  document.getElementById('eloRating').textContent = economyState.eloRating;
  
  const eloChangeEl = document.getElementById('eloChange');
  const change = economyState.lastEloChange;
  eloChangeEl.textContent = `${change >= 0 ? '+' : ''}${change} (Last Game)`;
  eloChangeEl.className = `elo-change ${change >= 0 ? 'positive' : 'negative'}`;
  
  const progress = (economyState.currentGifts / economyState.giftsToNextLevel) * 100;
  document.getElementById('progressFill').style.width = `${progress}%`;
  document.getElementById('progressText').textContent = 
    `${economyState.currentGifts} / ${economyState.giftsToNextLevel} Gifts to Level ${economyState.level + 1}`;
}

// === Result Modal ===
function showResultModal(status, rewards) {
  const modal = elements.resultModal;

  const playerWon = status.winner === gameState.playerColor;
  const isDraw = status.phase === 'stalemate';

  document.getElementById('resultTitle').textContent =
    isDraw ? '🤝 Draw!' :
    status.phase === 'checkmate' ? (playerWon ? '🏆 You Win!' : '💀 You Lose!') :
    status.phase === 'timeout' ? (playerWon ? '⏱ You Win on Time!' : '⏱ Time Out!') :
    status.phase === 'resignation' ? 'Resigned' : 'Game Over';

  document.getElementById('resultMessage').textContent = status.message;
  document.getElementById('cashEarned').textContent = `$${rewards.cash.toFixed(2)}`;
  document.getElementById('giftsEarnedResult').textContent = rewards.gifts;
  document.getElementById('ratingChange').textContent = `${rewards.eloChange >= 0 ? '+' : ''}${rewards.eloChange}`;

  modal.style.display = 'flex';
  modal.classList.remove('hidden');

  document.getElementById('playAgainBtn').onclick = () => {
    modal.style.display = 'none';
    modal.classList.add('hidden');
    showModeSelection();
  };

  document.getElementById('viewAnalysisBtn').onclick = () => {
    modal.style.display = 'none';
    modal.classList.add('hidden');
    document.getElementById('analysisSection')?.scrollIntoView({ behavior: 'smooth' });
  };
}

// === Fullscreen ===
function toggleFullscreen() {
  if (!document.fullscreenElement) {
    document.documentElement.requestFullscreen();
    document.body.classList.add('fullscreen');
  } else {
    document.exitFullscreen();
    document.body.classList.remove('fullscreen');
  }
}

// === Share Modal ===
function showShareModal() {
  const challengeId = generateChallengeId();
  const link = `${window.location.origin}/challenge/${challengeId}`;
  
  document.getElementById('shareLink').value = link;
  elements.shareModal.classList.remove('hidden');
}

function closeShareModal() {
  elements.shareModal.classList.add('hidden');
}

function copyShareLink() {
  const input = document.getElementById('shareLink');
  input.select();
  document.execCommand('copy');
  showNotification('Copied!', 'Challenge link copied to clipboard');
}

function generateChallengeId() {
  return Math.random().toString(36).substring(2, 15);
}

// === Chat ===
function toggleChat() {
  elements.chatPanel.classList.toggle('hidden');
}

function sendChatMessage() {
  const input = document.getElementById('chatInput');
  const message = input.value.trim();
  
  if (!message) return;
  
  addChatMessage('you', message);
  input.value = '';
  
  // Simulate opponent response (replace with real WebSocket)
  setTimeout(() => {
    addChatMessage('opponent', 'Good move!');
  }, 1000);
}

function addChatMessage(sender, text) {
  const messagesContainer = document.getElementById('chatMessages');
  
  const messageDiv = document.createElement('div');
  messageDiv.className = `chat-message ${sender}`;
  
  if (sender === 'you') {
    messageDiv.innerHTML = `
      <div class="message-content">
        <div class="message-text">${text}</div>
        <div class="message-time">${new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</div>
      </div>
      <div class="message-avatar">YO</div>
    `;
  } else {
    messageDiv.innerHTML = `
      <div class="message-avatar">OP</div>
      <div class="message-content">
        <div class="message-text">${text}</div>
        <div class="message-time">${new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</div>
      </div>
    `;
  }
  
  messagesContainer.appendChild(messageDiv);
  messagesContainer.scrollTop = messagesContainer.scrollHeight;
}

// === Notifications System ===
let notifications = [];
let unreadCount = 0;

function loadNotifications() {
  const saved = localStorage.getItem('chessq_notifications');
  if (saved) {
    try {
      notifications = JSON.parse(saved);
      updateNotificationBadge();
    } catch (e) {
      console.error('Failed to load notifications:', e);
    }
  }
}

function saveNotifications() {
  localStorage.setItem('chessq_notifications', JSON.stringify(notifications));
}

function addNotification(type, title, message) {
  const notif = {
    id: Date.now(),
    type, // 'game', 'achievement', 'system', 'friend'
    title,
    message,
    timestamp: Date.now(),
    read: false
  };
  
  notifications.unshift(notif);
  if (notifications.length > 50) notifications = notifications.slice(0, 50);
  
  saveNotifications();
  updateNotificationBadge();
  renderNotifications();
  
  // Show toast
  showNotification(title, message);
}

function updateNotificationBadge() {
  unreadCount = notifications.filter(n => !n.read).length;
  const badge = document.getElementById('notificationBadge');
  if (badge) {
    if (unreadCount > 0) {
      badge.textContent = unreadCount > 9 ? '9+' : unreadCount;
      badge.classList.remove('hidden');
    } else {
      badge.classList.add('hidden');
    }
  }
}

function toggleNotificationPanel() {
  const panel = document.getElementById('notificationPanel');
  panel.classList.toggle('hidden');
  
  if (!panel.classList.contains('hidden')) {
    renderNotifications();
  }
}

function renderNotifications() {
  const list = document.getElementById('notifList');
  const empty = document.getElementById('notifEmpty');
  
  if (notifications.length === 0) {
    empty.classList.remove('hidden');
    return;
  }
  
  empty.classList.add('hidden');
  
  const notifHTML = notifications.map(n => {
    const icon = {
      'game': '♟',
      'achievement': '🏆',
      'system': '⚙️',
      'friend': '👤'
    }[n.type] || '🔔';
    
    const timeAgo = formatTimeAgo(n.timestamp);
    
    return `
      <div class="notif-item ${n.read ? 'notif-read' : 'notif-unread'}" data-id="${n.id}">
        <div class="notif-icon">${icon}</div>
        <div class="notif-content">
          <div class="notif-title">${n.title}</div>
          <div class="notif-message">${n.message}</div>
          <div class="notif-time">${timeAgo}</div>
        </div>
        ${!n.read ? '<div class="notif-dot"></div>' : ''}
      </div>
    `;
  }).join('');
  
  list.innerHTML = notifHTML;
  
  // Add click handlers
  list.querySelectorAll('.notif-item').forEach(item => {
    item.addEventListener('click', () => {
      const id = parseInt(item.dataset.id);
      markNotificationRead(id);
    });
  });
}

function markNotificationRead(id) {
  const notif = notifications.find(n => n.id === id);
  if (notif && !notif.read) {
    notif.read = true;
    saveNotifications();
    updateNotificationBadge();
    renderNotifications();
  }
}

function markAllNotificationsRead() {
  notifications.forEach(n => n.read = true);
  saveNotifications();
  updateNotificationBadge();
  renderNotifications();
}

function formatTimeAgo(timestamp) {
  const seconds = Math.floor((Date.now() - timestamp) / 1000);
  
  if (seconds < 60) return 'Just now';
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ago`;
  if (seconds < 604800) return `${Math.floor(seconds / 86400)}d ago`;
  return new Date(timestamp).toLocaleDateString();
}

// === Notifications ===
function showNotification(title, message) {
  document.getElementById('toastTitle').textContent = title;
  document.getElementById('toastMessage').textContent = message;
  elements.notificationToast.classList.remove('hidden');
  
  setTimeout(() => {
    elements.notificationToast.classList.add('hidden');
  }, 5000);
}

// === Game Actions ===
function resignGame() {
  if (confirm('Are you sure you want to resign?')) {
    endGame({ phase: 'resignation', winner: gameState.turn === 'w' ? 'b' : 'w', message: 'You resigned' });
  }
}

function offerDraw() {
  showNotification('Draw Offered', 'Waiting for opponent response...');
}

function requestUndo() {
  showNotification('Undo Requested', 'Waiting for opponent approval...');
}

function handleWithdraw() {
  if (economyState.daysRemaining > 0) {
    alert(`Withdrawal available in ${economyState.daysRemaining} days (45-day holding period)`);
  } else {
    alert(`Withdraw $${economyState.availableBalance.toFixed(2)} to your account?`);
  }
}

// === Export for debugging ===
window.ChessQ = {
  gameState,
  economyState,
  playerProfile,
  showNotification,
  startGame,
  showProfileModal,
  saveProfile,
  loadProfile
};

// Make functions global for modal buttons
window.saveProfileChanges = saveProfileChanges;

console.log('ChessQ Premium Play loaded ✓');
console.log('Profile:', playerProfile.username);
