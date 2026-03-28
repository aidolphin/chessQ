const pieceGlyphs = {
  wp: "♙",
  wn: "♘",
  wb: "♗",
  wr: "♖",
  wq: "♕",
  wk: "♔",
  bp: "♟",
  bn: "♞",
  bb: "♝",
  br: "♜",
  bq: "♛",
  bk: "♚",
};

const FILES = "abcdefgh";
const RANKS = "87654321";
const INITIAL_COUNTS = { p: 8, n: 2, b: 2, r: 2, q: 1, k: 1 };
const PIECE_VALUES = { p: 1, n: 3, b: 3, r: 5, q: 9, k: 0 };
const PROFILE_KEY = "chessq.profile";
const HISTORY_KEY = "chessq.matchHistory";
const DEFAULT_TIME = 5;

const boardElement = document.querySelector("[data-board]");
const statusElement = document.querySelector("[data-status]");
const modeLabelElement = document.querySelector("[data-mode-label]");
const fenInput = document.querySelector("[data-fen-input]");
const moveListElement = document.querySelector("[data-moves]");
const whiteCapturedElement = document.querySelector("[data-captured-white]");
const blackCapturedElement = document.querySelector("[data-captured-black]");
const bannerElement = document.querySelector("[data-banner]");
const savedGamesElement = document.querySelector("[data-saved-games]");

const playerNameElements = {
  w: document.querySelector("[data-player-name='w']"),
  b: document.querySelector("[data-player-name='b']"),
};
const playerRoleElements = {
  w: document.querySelector("[data-player-role='w']"),
  b: document.querySelector("[data-player-role='b']"),
};
const playerAvatarElements = {
  w: document.querySelector("[data-player-avatar='w']"),
  b: document.querySelector("[data-player-avatar='b']"),
};
const playerClockElements = {
  w: document.querySelector("[data-player-clock='w']"),
  b: document.querySelector("[data-player-clock='b']"),
};
const boardClockElements = {
  w: document.querySelector("[data-board-clock='w']"),
  b: document.querySelector("[data-board-clock='b']"),
};
const playerCardElements = {
  w: document.querySelector("[data-player-card='w']"),
  b: document.querySelector("[data-player-card='b']"),
};

const summaryWhiteElement = document.querySelector("[data-summary-white]");
const summaryBlackElement = document.querySelector("[data-summary-black]");
const summaryTimeElement = document.querySelector("[data-summary-time]");
const summaryModeElement = document.querySelector("[data-summary-mode]");

const promotionDialog = document.querySelector("[data-promotion]");
const promotionChoices = document.querySelector("[data-promotion-choices]");
const promotionCloseButton = document.querySelector("[data-promotion-close]");

const setupDialog = document.querySelector("[data-setup]");
const setupForm = document.querySelector("[data-setup-form]");
const setupCloseButton = document.querySelector("[data-setup-close]");
const setupWhiteInput = document.querySelector("[data-setup-white]");
const setupBlackInput = document.querySelector("[data-setup-black]");
const setupPersonalitySelect = document.querySelector("[data-setup-personality]");
const botSettingsElement = document.querySelector("[data-bot-settings]");
const modeOptionButtons = Array.from(document.querySelectorAll("[data-mode-option]"));
const timeOptionButtons = Array.from(document.querySelectorAll("[data-time-option]"));

const resultDialog = document.querySelector("[data-result]");
const resultTitleElement = document.querySelector("[data-result-title]");
const resultCopyElement = document.querySelector("[data-result-copy]");
const resultCloseButton = document.querySelector("[data-result-close]");
const resultNewButton = document.querySelector("[data-result-new]");

const newGameButton = document.querySelector("[data-action='new']");
const shareButton = document.querySelector("[data-action='share']");
const undoButton = document.querySelector("[data-action='undo']");
const flipButton = document.querySelector("[data-action='flip']");
const copyFenButton = document.querySelector("[data-action='copy-fen']");
const loadFenButton = document.querySelector("[data-action='load-fen']");

const BOARD_COORDS = [];
for (const rank of RANKS) {
  for (const file of FILES) {
    BOARD_COORDS.push(`${file}${rank}`);
  }
}
const BOARD_INDEX_BY_COORD = new Map(BOARD_COORDS.map((coord, index) => [coord, index]));

let gameState = null;
let moveHistory = [];
let selectedSquare = null;
let selectedMoves = [];
let flipped = false;
let pendingPromotion = null;
let busy = false;
let gameConfig = loadInitialConfig();
let clocks = initialClocks(gameConfig.timeMinutes);
let clockIntervalId = null;
let lastClockTick = Date.now();
let resultState = null;
let resultSaved = false;
let pendingAiMove = null;

function loadStoredProfile() {
  try {
    return JSON.parse(localStorage.getItem(PROFILE_KEY) || "null");
  } catch {
    return null;
  }
}

function loadInitialConfig() {
  const stored = loadStoredProfile();
  const params = new URLSearchParams(window.location.search);

  const config = {
    mode: params.get("mode") === "bot" ? "bot" : stored?.mode || "human",
    whiteName: params.get("white") || stored?.whiteName || "White",
    blackName: params.get("black") || stored?.blackName || "Black",
    timeMinutes: Number(params.get("time") || stored?.timeMinutes || DEFAULT_TIME),
    aiPersonality: params.get("ai") || stored?.aiPersonality || "aggressive",
    botColor: "b",
  };

  if (config.mode === "bot" && (!config.blackName || config.blackName === "Black")) {
    config.blackName = "ChessQ AI";
  }

  return config;
}

function persistProfile() {
  localStorage.setItem(
    PROFILE_KEY,
    JSON.stringify({
      mode: gameConfig.mode,
      whiteName: gameConfig.whiteName,
      blackName: gameConfig.blackName,
      timeMinutes: gameConfig.timeMinutes,
      aiPersonality: gameConfig.aiPersonality,
    }),
  );
}

function loadSavedGames() {
  try {
    return JSON.parse(localStorage.getItem(HISTORY_KEY) || "[]");
  } catch {
    return [];
  }
}

function persistSavedGames(entries) {
  localStorage.setItem(HISTORY_KEY, JSON.stringify(entries.slice(0, 10)));
}

function initialClocks(timeMinutes) {
  const totalMs = Math.max(1, Number(timeMinutes) || DEFAULT_TIME) * 60 * 1000;
  return { w: totalMs, b: totalMs };
}

function turnLabel(color) {
  return color === "w" ? "White" : "Black";
}

function modeLabel() {
  return gameConfig.mode === "bot" ? "Play vs Bot" : "Friend Match";
}

function clockLabel() {
  return `${gameConfig.timeMinutes} min`;
}

function avatarLabel(name) {
  return name
    .trim()
    .split(/\s+/)
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase() || "")
    .join("") || "?";
}

function getDisplayOrder() {
  return flipped ? [...BOARD_COORDS].reverse() : BOARD_COORDS;
}

function clearSelection() {
  selectedSquare = null;
  selectedMoves = [];
}

function openOverlay(element) {
  element.hidden = false;
  element.classList.add("is-open");
}

function closeOverlay(element) {
  element.hidden = true;
  element.classList.remove("is-open");
}

function pieceAt(coord) {
  if (!gameState) {
    return null;
  }
  return gameState.board[BOARD_INDEX_BY_COORD.get(coord)] || null;
}

function isLastMoveSquare(coord) {
  return gameState?.lastMove && (gameState.lastMove.from === coord || gameState.lastMove.to === coord);
}

function isCheckedKing(coord) {
  const piece = pieceAt(coord);
  if (!piece || piece[1] !== "k") {
    return false;
  }
  return gameState.status.phase === "check" && piece[0] === gameState.turn;
}

function isLightSquare(coord) {
  const fileIndex = FILES.indexOf(coord[0]);
  const rank = Number(coord[1]);
  return (fileIndex + rank) % 2 === 0;
}

function formatClock(ms) {
  const totalSeconds = Math.max(0, Math.ceil(ms / 1000));
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
}

function renderProfiles() {
  const whiteRole = gameConfig.mode === "bot" ? "Human" : "Player";
  const blackRole = gameConfig.mode === "bot" ? `Bot • ${gameConfig.aiPersonality}` : "Player";

  playerNameElements.w.textContent = gameConfig.whiteName;
  playerNameElements.b.textContent = gameConfig.blackName;
  playerRoleElements.w.textContent = `White • ${whiteRole}`;
  playerRoleElements.b.textContent = `Black • ${blackRole}`;
  playerAvatarElements.w.textContent = avatarLabel(gameConfig.whiteName);
  playerAvatarElements.b.textContent = avatarLabel(gameConfig.blackName);

  summaryWhiteElement.textContent = gameConfig.whiteName;
  summaryBlackElement.textContent = gameConfig.blackName;
  summaryTimeElement.textContent = clockLabel();
  summaryModeElement.textContent = modeLabel();
  modeLabelElement.textContent = modeLabel();

  for (const color of ["w", "b"]) {
    playerClockElements[color].textContent = formatClock(clocks[color]);
    playerClockElements[color].classList.toggle("low", clocks[color] <= 30_000);
    playerCardElements[color].classList.toggle("active", gameState && gameState.turn === color && !resultState);
    boardClockElements[color].textContent = formatClock(clocks[color]);
    boardClockElements[color].classList.toggle("low", clocks[color] <= 30_000);
    boardClockElements[color].classList.toggle("active", gameState && gameState.turn === color && !resultState);
  }
}

function renderSavedGames() {
  const savedGames = loadSavedGames();
  savedGamesElement.innerHTML = "";

  if (!savedGames.length) {
    savedGamesElement.innerHTML = "<li class='empty-line'>No saved match results yet.</li>";
    return;
  }

  for (const entry of savedGames) {
    const row = document.createElement("li");
    row.className = "saved-line";
    row.innerHTML = `
      <strong>${entry.whiteName} vs ${entry.blackName}</strong>
      <span>${entry.result} • ${entry.timeControl} • ${entry.playedAt}</span>
    `;
    savedGamesElement.appendChild(row);
  }
}

function renderBoard() {
  boardElement.innerHTML = "";

  for (const coord of getDisplayOrder()) {
    const square = document.createElement("button");
    square.type = "button";
    square.className = "square";
    square.dataset.square = coord;
    square.classList.add(isLightSquare(coord) ? "light" : "dark");

    if (selectedSquare === coord) {
      square.classList.add("selected");
    }
    if (selectedMoves.some((move) => move.to === coord)) {
      square.classList.add("legal");
    }
    if (isLastMoveSquare(coord)) {
      square.classList.add("last-move");
    }
    if (isCheckedKing(coord)) {
      square.classList.add("check");
    }

    const piece = pieceAt(coord);
    const pieceClass = piece ? `${piece[0] === "w" ? "white-piece" : "black-piece"}` : "";

    square.innerHTML = `
      <span class="square-coord">${coord}</span>
      <span class="piece ${pieceClass}">${piece ? pieceGlyphs[piece] : ""}</span>
      <span class="legal-dot"></span>
    `;

    boardElement.appendChild(square);
  }
}

function renderMoves() {
  moveListElement.innerHTML = "";

  if (!moveHistory.length) {
    moveListElement.innerHTML = "<li class='empty-line'>No moves yet. Set the board and start the fight.</li>";
    return;
  }

  for (let index = 0; index < moveHistory.length; index += 2) {
    const row = document.createElement("li");
    row.className = "move-row";
    row.innerHTML = `
      <span class="move-number">${index / 2 + 1}.</span>
      <span>${moveHistory[index]?.notation || ""}</span>
      <span>${moveHistory[index + 1]?.notation || ""}</span>
    `;
    moveListElement.appendChild(row);
  }
}

function computeCapturedPieces() {
  const counts = {
    w: { p: 0, n: 0, b: 0, r: 0, q: 0, k: 0 },
    b: { p: 0, n: 0, b: 0, r: 0, q: 0, k: 0 },
  };

  for (const piece of gameState.board) {
    if (!piece) {
      continue;
    }
    counts[piece[0]][piece[1]] += 1;
  }

  const captured = { w: [], b: [] };
  for (const color of ["w", "b"]) {
    for (const type of ["q", "r", "b", "n", "p"]) {
      const missing = INITIAL_COUNTS[type] - counts[color][type];
      for (let index = 0; index < missing; index += 1) {
        captured[color].push(`${color}${type}`);
      }
    }
    captured[color].sort((a, b) => PIECE_VALUES[b[1]] - PIECE_VALUES[a[1]]);
  }

  return captured;
}

function renderCaptured() {
  const captured = computeCapturedPieces();
  whiteCapturedElement.textContent = captured.w.map((piece) => pieceGlyphs[piece]).join(" ") || "None";
  blackCapturedElement.textContent = captured.b.map((piece) => pieceGlyphs[piece]).join(" ") || "None";
}

function renderStatus() {
  const message = resultState?.message || gameState.status.message;
  statusElement.textContent = message;

  if (resultState) {
    bannerElement.hidden = false;
    bannerElement.textContent = message;
  } else if (gameState.gameOver) {
    bannerElement.hidden = false;
    bannerElement.textContent = gameState.status.message;
  } else {
    bannerElement.hidden = true;
  }
}

function renderSetupOptions() {
  setupWhiteInput.value = gameConfig.whiteName;
  setupBlackInput.value = gameConfig.blackName;
  setupPersonalitySelect.value = gameConfig.aiPersonality;

  for (const button of modeOptionButtons) {
    button.classList.toggle("active", button.dataset.modeOption === gameConfig.mode);
  }

  for (const button of timeOptionButtons) {
    button.classList.toggle("active", Number(button.dataset.timeOption) === gameConfig.timeMinutes);
  }

  botSettingsElement.hidden = gameConfig.mode !== "bot";
}

function render() {
  if (!gameState) {
    return;
  }

  renderProfiles();
  renderBoard();
  renderMoves();
  renderCaptured();
  renderStatus();
  renderSavedGames();
  fenInput.value = gameState.fen;
  undoButton.disabled = moveHistory.length === 0 || gameConfig.mode === "bot";
}

function setStatusMessage(message) {
  statusElement.textContent = message;
}

function closePromotionDialog() {
  pendingPromotion = null;
  closeOverlay(promotionDialog);
  promotionChoices.innerHTML = "";
}

function promotionName(promotion) {
  return {
    q: "queen",
    r: "rook",
    b: "bishop",
    n: "knight",
  }[promotion];
}

function openPromotionDialog(moves) {
  pendingPromotion = moves;
  promotionChoices.innerHTML = "";

  const color = gameState.turn;
  for (const promotion of ["q", "r", "b", "n"]) {
    const piece = `${color}${promotion}`;
    const button = document.createElement("button");
    button.type = "button";
    button.className = "promotion-option";
    button.dataset.promotion = promotion;
    button.innerHTML = `
      <span>${pieceGlyphs[piece]}</span>
      <span class="promotion-label">${promotionName(promotion)}</span>
    `;
    promotionChoices.appendChild(button);
  }

  openOverlay(promotionDialog);
}

function openResultDialog(title, message) {
  resultTitleElement.textContent = title;
  resultCopyElement.textContent = message;
  openOverlay(resultDialog);
}

function closeResultDialog() {
  closeOverlay(resultDialog);
}

function updateUrlFromState() {
  const params = new URLSearchParams();
  params.set("mode", gameConfig.mode);
  params.set("white", gameConfig.whiteName);
  params.set("black", gameConfig.blackName);
  params.set("time", String(gameConfig.timeMinutes));
  params.set("ai", gameConfig.aiPersonality);
  params.set("fen", gameState.fen);
  window.history.replaceState({}, "", `${window.location.pathname}?${params}`);
}

function buildShareUrl() {
  const params = new URLSearchParams();
  params.set("mode", gameConfig.mode);
  params.set("white", gameConfig.whiteName);
  params.set("black", gameConfig.blackName);
  params.set("time", String(gameConfig.timeMinutes));
  params.set("ai", gameConfig.aiPersonality);
  params.set("fen", gameState.fen);
  return `${window.location.origin}${window.location.pathname}?${params}`;
}

async function apiRequest(path, params = {}) {
  const search = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    if (value !== undefined && value !== null && value !== "") {
      search.set(key, value);
    }
  }

  const target = search.size ? `${path}?${search}` : path;
  const response = await fetch(target);
  if (!response.ok) {
    const message = await response.text();
    throw new Error(message || `Request failed with ${response.status}`);
  }
  return response.json();
}

async function runSafely(task) {
  if (busy) {
    return null;
  }

  busy = true;
  try {
    return await task();
  } catch (error) {
    setStatusMessage(error.message);
    return null;
  } finally {
    busy = false;
  }
}

function resetGameSession() {
  if (pendingAiMove) {
    clearTimeout(pendingAiMove);
    pendingAiMove = null;
  }
  clearSelection();
  closePromotionDialog();
  closeResultDialog();
  resultState = null;
  resultSaved = false;
  moveHistory = [];
  clocks = initialClocks(gameConfig.timeMinutes);
  lastClockTick = Date.now();
  stopClockLoop();
}

function startClockLoop() {
  stopClockLoop();
  lastClockTick = Date.now();
  clockIntervalId = window.setInterval(tickClocks, 250);
}

function stopClockLoop() {
  if (clockIntervalId) {
    clearInterval(clockIntervalId);
    clockIntervalId = null;
  }
}

function tickClocks() {
  if (!gameState || resultState || gameState.gameOver || setupDialog.classList.contains("is-open")) {
    return;
  }

  const now = Date.now();
  const delta = now - lastClockTick;
  lastClockTick = now;

  const activeColor = gameState.turn;
  clocks[activeColor] = Math.max(0, clocks[activeColor] - delta);
  renderProfiles();

  if (clocks[activeColor] === 0) {
    handleTimeout(activeColor);
  }
}

function saveCompletedGame(resultText) {
  if (resultSaved) {
    return;
  }

  const savedGames = loadSavedGames();
  savedGames.unshift({
    whiteName: gameConfig.whiteName,
    blackName: gameConfig.blackName,
    result: resultText,
    timeControl: clockLabel(),
    playedAt: new Date().toLocaleString(),
  });
  persistSavedGames(savedGames);
  resultSaved = true;
}

function handleTimeout(loserColor) {
  const winnerColor = loserColor === "w" ? "b" : "w";
  const winnerName = winnerColor === "w" ? gameConfig.whiteName : gameConfig.blackName;
  resultState = {
    title: "Time Up",
    message: `${winnerName} wins on time.`,
  };
  stopClockLoop();
  openResultDialog(resultState.title, resultState.message);
  saveCompletedGame(resultState.message);
  render();
}

function handleFinishedGame() {
  if (!gameState.gameOver || resultState) {
    return;
  }

  resultState = {
    title: gameState.status.phase === "stalemate" ? "Draw" : "Winner",
    message: gameState.status.message,
  };
  stopClockLoop();
  openResultDialog(resultState.title, resultState.message);
  saveCompletedGame(gameState.status.message);
  render();
}

function maybeQueueAiMove() {
  if (
    gameConfig.mode !== "bot"
    || !gameState
    || gameState.turn !== gameConfig.botColor
    || gameState.gameOver
    || resultState
  ) {
    return;
  }

  if (pendingAiMove) {
    clearTimeout(pendingAiMove);
  }

  pendingAiMove = window.setTimeout(() => {
    pendingAiMove = null;
    void runAiMove();
  }, 350);
}

async function runAiMove() {
  setStatusMessage(`${gameConfig.blackName} is thinking...`);
  const payload = await runSafely(async () => apiRequest("/api/ai-move", {
    fen: gameState.fen,
    personality: gameConfig.aiPersonality,
    ms: 700,
  }));

  if (!payload || resultState) {
    return;
  }

  moveHistory.push({
    notation: payload.moveNotation,
    snapshot: gameState,
    clocks: { ...clocks },
  });

  gameState = payload.snapshot;
  updateUrlFromState();
  render();
  handleFinishedGame();
}

async function startConfiguredGame(fen = null) {
  persistProfile();
  resetGameSession();

  const payload = await runSafely(async () => {
    if (fen) {
      return apiRequest("/api/state", { fen });
    }
    return apiRequest("/api/new");
  });

  if (!payload) {
    return;
  }

  gameState = payload;
  updateUrlFromState();
  renderSetupOptions();
  render();
  startClockLoop();
  maybeQueueAiMove();
}

async function fetchLegalMoves(from) {
  const payload = await apiRequest("/api/legal-moves", { fen: gameState.fen, from });
  return payload.moves;
}

async function applyMoveFromPayload(payload) {
  moveHistory.push({
    notation: payload.moveNotation,
    snapshot: gameState,
    clocks: { ...clocks },
  });

  gameState = payload.snapshot;
  closePromotionDialog();
  clearSelection();
  updateUrlFromState();
  render();
  handleFinishedGame();
  maybeQueueAiMove();
}

async function applyMove(move) {
  const payload = await runSafely(async () => apiRequest("/api/move", {
    fen: gameState.fen,
    from: move.from,
    to: move.to,
    promotion: move.promotion,
  }));

  if (payload) {
    await applyMoveFromPayload(payload);
  }
}

async function loadFenPosition() {
  const fen = fenInput.value.trim();
  if (!fen) {
    return;
  }

  resetGameSession();
  const payload = await runSafely(async () => apiRequest("/api/state", { fen }));
  if (!payload) {
    return;
  }

  gameState = payload;
  updateUrlFromState();
  render();
  startClockLoop();
  maybeQueueAiMove();
}

async function tryMove(to) {
  const candidates = selectedMoves.filter((move) => move.to === to);
  if (!candidates.length) {
    return false;
  }

  if (candidates.length > 1) {
    openPromotionDialog(candidates);
    return true;
  }

  await applyMove(candidates[0]);
  return true;
}

function isHumanTurn() {
  return gameConfig.mode !== "bot" || gameState.turn !== gameConfig.botColor;
}

async function handleSquareClick(coord) {
  if (!gameState || resultState || pendingPromotion || gameState.gameOver || busy || !isHumanTurn()) {
    return;
  }

  const piece = pieceAt(coord);
  const turn = gameState.turn;

  if (selectedSquare === null) {
    if (piece && piece[0] === turn) {
      const moves = await runSafely(async () => fetchLegalMoves(coord));
      if (!moves) {
        return;
      }
      selectedSquare = coord;
      selectedMoves = moves;
      renderBoard();
    }
    return;
  }

  if (selectedSquare === coord) {
    clearSelection();
    renderBoard();
    return;
  }

  if (piece && piece[0] === turn) {
    const moves = await runSafely(async () => fetchLegalMoves(coord));
    if (!moves) {
      return;
    }
    selectedSquare = coord;
    selectedMoves = moves;
    renderBoard();
    return;
  }

  if (!await tryMove(coord)) {
    clearSelection();
    renderBoard();
  }
}

function openSetupDialog() {
  renderSetupOptions();
  openOverlay(setupDialog);
}

function closeSetupDialog() {
  closeOverlay(setupDialog);
}

function modeButtonSelection(mode) {
  gameConfig.mode = mode;
  if (mode === "bot" && (!setupBlackInput.value.trim() || setupBlackInput.value.trim() === "Black")) {
    setupBlackInput.value = "ChessQ AI";
  }
  renderSetupOptions();
}

function timeButtonSelection(minutes) {
  gameConfig.timeMinutes = Number(minutes);
  renderSetupOptions();
}

function readSetupForm() {
  const whiteName = setupWhiteInput.value.trim() || "White";
  const blackFallback = gameConfig.mode === "bot" ? "ChessQ AI" : "Black";
  const blackName = setupBlackInput.value.trim() || blackFallback;

  gameConfig = {
    ...gameConfig,
    whiteName,
    blackName,
    aiPersonality: setupPersonalitySelect.value,
  };
}

async function shareCurrentLink() {
  try {
    await navigator.clipboard.writeText(buildShareUrl());
    setStatusMessage("Share link copied to clipboard.");
  } catch (error) {
    setStatusMessage(error.message);
  }
}

boardElement.addEventListener("click", (event) => {
  const square = event.target.closest("[data-square]");
  if (!square) {
    return;
  }
  void handleSquareClick(square.dataset.square);
});

promotionDialog.addEventListener("click", (event) => {
  if (event.target === promotionDialog) {
    closePromotionDialog();
    render();
  }
});

promotionChoices.addEventListener("click", (event) => {
  const option = event.target.closest("[data-promotion]");
  if (!option) {
    return;
  }

  const move = pendingPromotion?.find((candidate) => candidate.promotion === option.dataset.promotion);
  if (move) {
    void applyMove(move);
  }
});

promotionCloseButton.addEventListener("click", () => {
  closePromotionDialog();
  render();
});

setupDialog.addEventListener("click", (event) => {
  if (event.target === setupDialog) {
    closeSetupDialog();
  }
});

setupCloseButton.addEventListener("click", () => {
  closeSetupDialog();
});

for (const button of modeOptionButtons) {
  button.addEventListener("click", () => modeButtonSelection(button.dataset.modeOption));
}

for (const button of timeOptionButtons) {
  button.addEventListener("click", () => timeButtonSelection(button.dataset.timeOption));
}

setupForm.addEventListener("submit", (event) => {
  event.preventDefault();
  readSetupForm();
  closeSetupDialog();
  void startConfiguredGame();
});

resultDialog.addEventListener("click", (event) => {
  if (event.target === resultDialog) {
    closeResultDialog();
  }
});

resultCloseButton.addEventListener("click", () => {
  closeResultDialog();
});

resultNewButton.addEventListener("click", () => {
  closeResultDialog();
  openSetupDialog();
});

newGameButton.addEventListener("click", () => {
  openSetupDialog();
});

shareButton.addEventListener("click", () => {
  void shareCurrentLink();
});

undoButton.addEventListener("click", () => {
  if (!moveHistory.length || busy || gameConfig.mode === "bot") {
    return;
  }

  const previous = moveHistory.pop();
  gameState = previous.snapshot;
  clocks = previous.clocks;
  resultState = null;
  resultSaved = false;
  closePromotionDialog();
  closeResultDialog();
  clearSelection();
  updateUrlFromState();
  render();
  startClockLoop();
});

flipButton.addEventListener("click", () => {
  flipped = !flipped;
  renderBoard();
});

copyFenButton.addEventListener("click", async () => {
  try {
    await navigator.clipboard.writeText(gameState.fen);
    setStatusMessage("FEN copied to clipboard.");
  } catch (error) {
    setStatusMessage(error.message);
  }
});

loadFenButton.addEventListener("click", () => {
  void loadFenPosition();
});

document.addEventListener("keydown", (event) => {
  if (event.key === "Escape") {
    closePromotionDialog();
    closeSetupDialog();
    closeResultDialog();
    render();
  }
});

function boot() {
  renderSetupOptions();
  renderSavedGames();
  closePromotionDialog();
  closeSetupDialog();
  closeResultDialog();

  const sharedFen = new URLSearchParams(window.location.search).get("fen");
  void startConfiguredGame(sharedFen);
}

boot();
