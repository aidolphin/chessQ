// ChessQ Learning Engine
// Professional interactive chess lessons with live board

const LEARN_CHAPTERS = [
  {
    id: 'basics',
    title: 'Chess Basics',
    icon: '♟',
    desc: 'Board, pieces, and how the game works',
    steps: [
      {
        title: 'The Chessboard',
        text: 'Chess is played on an 8×8 board with 64 squares. The board has files (a-h) and ranks (1-8). The bottom-right square is always light.',
        fen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
        highlight: ['a1', 'h1', 'a8', 'h8']
      },
      {
        title: 'Starting Position',
        text: 'Each player starts with 16 pieces: 8 pawns, 2 rooks, 2 knights, 2 bishops, 1 queen, and 1 king. White always moves first.',
        fen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'
      },
      {
        title: 'The Goal',
        text: 'The goal is to checkmate your opponent\'s king. This means the king is under attack and has no way to escape.',
        fen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'
      }
    ]
  },
  {
    id: 'pawn',
    title: 'The Pawn',
    icon: '♙',
    desc: 'How pawns move, capture, and promote',
    steps: [
      {
        title: 'Pawn Movement',
        text: 'Pawns move forward one square. On their first move, they can move two squares forward.',
        fen: '8/8/8/8/8/8/4P3/8 w - - 0 1',
        highlight: ['e3', 'e4']
      },
      {
        title: 'Pawn Captures',
        text: 'Pawns capture diagonally forward, one square. They cannot capture straight ahead.',
        fen: '8/8/8/3p1p2/4P3/8/8/8 w - - 0 1',
        highlight: ['d5', 'f5']
      },
      {
        title: 'Pawn Promotion',
        text: 'When a pawn reaches the opposite end of the board, it promotes to a queen, rook, bishop, or knight.',
        fen: '8/4P3/8/8/8/8/8/8 w - - 0 1',
        highlight: ['e8']
      },
      {
        title: 'Quiz: Pawn Moves',
        quiz: {
          question: 'A pawn on e2 can move to which squares on its first move?',
          options: ['e3 only', 'e3 or e4', 'e3, e4, or e5', 'd3 or f3'],
          correct: 1,
          explanation: 'Pawns can move 1 or 2 squares forward on their first move.'
        },
        fen: '8/8/8/8/8/8/4P3/8 w - - 0 1'
      }
    ]
  },
  {
    id: 'knight',
    title: 'The Knight',
    icon: '♞',
    desc: 'The knight\'s unique L-shaped move',
    steps: [
      {
        title: 'Knight Movement',
        text: 'The knight moves in an L-shape: 2 squares in one direction, then 1 square perpendicular. It\'s the only piece that can jump over others.',
        fen: '8/8/8/8/3N4/8/8/8 w - - 0 1',
        highlight: ['c6', 'e6', 'f5', 'f3', 'e2', 'c2', 'b3', 'b5']
      },
      {
        title: 'Knight Jumps',
        text: 'Knights can jump over pieces. This makes them powerful in crowded positions.',
        fen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
        highlight: ['f3', 'h3']
      },
      {
        title: 'Quiz: Knight Moves',
        quiz: {
          question: 'From d4, how many squares can a knight move to?',
          options: ['4 squares', '6 squares', '8 squares', '12 squares'],
          correct: 2,
          explanation: 'A knight in the center can reach 8 squares.'
        },
        fen: '8/8/8/8/3N4/8/8/8 w - - 0 1'
      }
    ]
  },
  {
    id: 'bishop',
    title: 'The Bishop',
    icon: '♗',
    desc: 'Diagonal movement and control',
    steps: [
      {
        title: 'Bishop Movement',
        text: 'Bishops move diagonally any number of squares. Each bishop stays on its starting color (light or dark) for the entire game.',
        fen: '8/8/8/8/3B4/8/8/8 w - - 0 1',
        highlight: ['a1', 'b2', 'c3', 'e5', 'f6', 'g7', 'h8', 'a7', 'b6', 'c5', 'e3', 'f2', 'g1']
      },
      {
        title: 'Light and Dark Bishops',
        text: 'You start with one light-squared bishop and one dark-squared bishop. They can never meet!',
        fen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
        highlight: ['c1', 'f1']
      }
    ]
  },
  {
    id: 'rook',
    title: 'The Rook',
    icon: '♜',
    desc: 'Straight-line power',
    steps: [
      {
        title: 'Rook Movement',
        text: 'Rooks move horizontally or vertically any number of squares. They\'re most powerful on open files.',
        fen: '8/8/8/8/3R4/8/8/8 w - - 0 1',
        highlight: ['d1', 'd2', 'd3', 'd5', 'd6', 'd7', 'd8', 'a4', 'b4', 'c4', 'e4', 'f4', 'g4', 'h4']
      },
      {
        title: 'Rook Power',
        text: 'Rooks are worth about 5 pawns. They work best on open files and the 7th rank.',
        fen: '6k1/6pp/8/8/8/8/R5PP/6K1 w - - 0 1',
        highlight: ['a2', 'a7']
      }
    ]
  },
  {
    id: 'queen',
    title: 'The Queen',
    icon: '♕',
    desc: 'The most powerful piece',
    steps: [
      {
        title: 'Queen Movement',
        text: 'The queen combines the power of the rook and bishop. She can move any number of squares horizontally, vertically, or diagonally.',
        fen: '8/8/8/8/3Q4/8/8/8 w - - 0 1',
        highlight: ['d1','d2','d3','d5','d6','d7','d8','a4','b4','c4','e4','f4','g4','h4','a1','b2','c3','e5','f6','g7','h8','a7','b6','c5','e3','f2','g1']
      },
      {
        title: 'Queen Value',
        text: 'The queen is worth about 9 pawns. Don\'t bring her out too early or she\'ll be attacked!',
        fen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'
      }
    ]
  },
  {
    id: 'king',
    title: 'The King',
    icon: '♔',
    desc: 'Protect your king at all costs',
    steps: [
      {
        title: 'King Movement',
        text: 'The king moves one square in any direction. He can never move into check (under attack).',
        fen: '8/8/8/8/3K4/8/8/8 w - - 0 1',
        highlight: ['c3', 'd3', 'e3', 'c4', 'e4', 'c5', 'd5', 'e5']
      },
      {
        title: 'King Safety',
        text: 'The king is priceless. If your king is checkmated, you lose the game. Always keep your king safe!',
        fen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
        highlight: ['e1']
      },
      {
        title: 'Castling',
        text: 'Castling is a special move that protects your king. The king moves 2 squares toward a rook, and the rook jumps over. You can only castle if: neither piece has moved, there are no pieces between them, and the king isn\'t in check.',
        fen: 'r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1',
        highlight: ['c1', 'g1']
      }
    ]
  },
  {
    id: 'checkmate',
    title: 'Check & Checkmate',
    icon: '♚',
    desc: 'How to win the game',
    steps: [
      {
        title: 'Check',
        text: 'When the king is under attack, it\'s called "check". You must get out of check immediately by moving the king, blocking, or capturing the attacker.',
        fen: '4k3/8/8/8/8/8/8/4K2R w - - 0 1',
        highlight: ['e8', 'h1']
      },
      {
        title: 'Checkmate',
        text: 'Checkmate means the king is in check and has no legal moves. The game is over!',
        fen: '5rk1/6pp/8/8/8/8/8/4K2R w - - 0 1',
        highlight: ['g8', 'f8', 'h1']
      },
      {
        title: 'Back Rank Mate',
        text: 'This is the most common checkmate pattern. The king is trapped on the back rank by its own pawns.',
        fen: '6k1/5ppp/8/8/8/8/8/R6K w - - 0 1',
        highlight: ['g8', 'a1']
      },
      {
        title: 'Quiz: Checkmate',
        quiz: {
          question: 'What is checkmate?',
          options: [
            'The king is under attack',
            'The king is under attack and cannot escape',
            'The king has no legal moves',
            'The game ends in a draw'
          ],
          correct: 1,
          explanation: 'Checkmate = king in check + no way to escape = game over!'
        },
        fen: '5rk1/6pp/8/8/8/8/8/4K2R w - - 0 1'
      }
    ]
  },
  {
    id: 'tactics',
    title: 'Basic Tactics',
    icon: '⚔️',
    desc: 'Winning combinations',
    steps: [
      {
        title: 'The Fork',
        text: 'A fork attacks two pieces at once. Knights are especially good at forking because they can\'t be blocked.',
        fen: '4k3/8/8/3N4/8/8/4R3/4K3 w - - 0 1',
        highlight: ['d5', 'e8', 'e2']
      },
      {
        title: 'The Pin',
        text: 'A pin attacks a piece that can\'t move because a more valuable piece is behind it.',
        fen: '4k3/4r3/8/8/8/8/4R3/4K3 w - - 0 1',
        highlight: ['e2', 'e7', 'e8']
      },
      {
        title: 'The Skewer',
        text: 'A skewer is like a reverse pin. You attack a valuable piece, and when it moves, you capture the piece behind it.',
        fen: '4k3/8/8/8/8/8/4K3/4R3 w - - 0 1',
        highlight: ['e1', 'e2', 'e8']
      },
      {
        title: 'Quiz: Tactics',
        quiz: {
          question: 'What tactic attacks two pieces at once?',
          options: ['Pin', 'Fork', 'Skewer', 'Discovery'],
          correct: 1,
          explanation: 'A fork attacks two or more pieces simultaneously!'
        },
        fen: '4k3/8/8/3N4/8/8/4R3/4K3 w - - 0 1'
      }
    ]
  }
];

let learnState = {
  currentChapter: null,
  currentStep: 0,
  completedChapters: []
};

function learnShowChapterSelect() {
  document.getElementById('learnChapterSelect').classList.remove('hidden');
  document.getElementById('learnPlayer').classList.add('hidden');
  
  const container = document.getElementById('lcChapters');
  container.innerHTML = LEARN_CHAPTERS.map((ch, i) => `
    <div class="lc-card" data-chapter="${i}">
      <div class="lc-card-icon">${ch.icon}</div>
      <div class="lc-card-content">
        <h3>${ch.title}</h3>
        <p>${ch.desc}</p>
        <div class="lc-card-meta">${ch.steps.length} steps</div>
      </div>
      <button class="lc-card-btn">Start →</button>
    </div>
  `).join('');
  
  container.querySelectorAll('.lc-card').forEach(card => {
    card.addEventListener('click', () => {
      const idx = parseInt(card.dataset.chapter);
      learnStartChapter(idx);
    });
  });
}

function learnStartChapter(chapterIdx) {
  learnState.currentChapter = chapterIdx;
  learnState.currentStep = 0;
  
  document.getElementById('learnChapterSelect').classList.add('hidden');
  document.getElementById('learnPlayer').classList.remove('hidden');
  
  const chapter = LEARN_CHAPTERS[chapterIdx];
  document.getElementById('lpChapterTitle').textContent = chapter.title;
  
  learnRenderStep();
}

function learnRenderStep() {
  const chapter = LEARN_CHAPTERS[learnState.currentChapter];
  const step = chapter.steps[learnState.currentStep];
  const stepNum = learnState.currentStep + 1;
  const totalSteps = chapter.steps.length;
  
  document.getElementById('lpSteps').textContent = `${stepNum} / ${totalSteps}`;
  document.getElementById('lpStepNum').textContent = `Step ${stepNum}`;
  document.getElementById('lpStepTitle').textContent = step.title;
  document.getElementById('lpStepBody').textContent = step.text;
  
  // Render board
  learnRenderBoard(step.fen, step.highlight || []);
  
  // Board hint
  const hint = document.getElementById('lpBoardHint');
  if (step.highlight && step.highlight.length > 0) {
    hint.textContent = 'Highlighted squares show key positions';
    hint.classList.remove('hidden');
  } else {
    hint.classList.add('hidden');
  }
  
  // Quiz
  const quizEl = document.getElementById('lpQuiz');
  if (step.quiz) {
    quizEl.classList.remove('hidden');
    document.getElementById('lpQuizQ').textContent = step.quiz.question;
    
    const optsEl = document.getElementById('lpQuizOpts');
    optsEl.innerHTML = step.quiz.options.map((opt, i) => 
      `<button class="lp-quiz-opt" data-idx="${i}">${opt}</button>`
    ).join('');
    
    optsEl.querySelectorAll('.lp-quiz-opt').forEach(btn => {
      btn.addEventListener('click', () => learnCheckQuiz(step.quiz, parseInt(btn.dataset.idx)));
    });
    
    document.getElementById('lpQuizResult').classList.add('hidden');
  } else {
    quizEl.classList.add('hidden');
  }
  
  // Navigation
  document.getElementById('lpPrev').disabled = stepNum === 1;
  document.getElementById('lpNext').textContent = stepNum === totalSteps ? 'Complete ✓' : 'Next →';
}

function learnRenderBoard(fen, highlights) {
  const board = document.getElementById('lpBoard');
  board.innerHTML = '';
  
  // Parse FEN
  const parts = fen.split(' ');
  const rows = parts[0].split('/');
  
  for (let rank = 0; rank < 8; rank++) {
    const row = rows[rank];
    let file = 0;
    
    for (const char of row) {
      if (char >= '1' && char <= '8') {
        const empty = parseInt(char);
        for (let i = 0; i < empty; i++) {
          const sq = document.createElement('div');
          sq.className = 'lp-sq';
          const coord = String.fromCharCode(97 + file) + (8 - rank);
          sq.dataset.coord = coord;
          if ((file + rank) % 2 === 0) sq.classList.add('lp-sq-light');
          else sq.classList.add('lp-sq-dark');
          if (highlights.includes(coord)) sq.classList.add('lp-sq-hl');
          board.appendChild(sq);
          file++;
        }
      } else {
        const sq = document.createElement('div');
        sq.className = 'lp-sq';
        const coord = String.fromCharCode(97 + file) + (8 - rank);
        sq.dataset.coord = coord;
        if ((file + rank) % 2 === 0) sq.classList.add('lp-sq-light');
        else sq.classList.add('lp-sq-dark');
        if (highlights.includes(coord)) sq.classList.add('lp-sq-hl');
        
        const piece = document.createElement('span');
        piece.className = 'lp-piece';
        const pieceMap = {
          'P': '♙', 'N': '♘', 'B': '♗', 'R': '♖', 'Q': '♕', 'K': '♔',
          'p': '♟', 'n': '♞', 'b': '♝', 'r': '♜', 'q': '♛', 'k': '♚'
        };
        piece.textContent = pieceMap[char];
        piece.classList.add(char === char.toUpperCase() ? 'lp-piece-w' : 'lp-piece-b');
        sq.appendChild(piece);
        
        board.appendChild(sq);
        file++;
      }
    }
  }
}

function learnCheckQuiz(quiz, selectedIdx) {
  const resultEl = document.getElementById('lpQuizResult');
  resultEl.classList.remove('hidden');
  
  if (selectedIdx === quiz.correct) {
    resultEl.innerHTML = `<div class="lp-quiz-correct">✓ Correct! ${quiz.explanation}</div>`;
  } else {
    resultEl.innerHTML = `<div class="lp-quiz-wrong">✗ Wrong. ${quiz.explanation}</div>`;
  }
}

function learnNextStep() {
  const chapter = LEARN_CHAPTERS[learnState.currentChapter];
  if (learnState.currentStep < chapter.steps.length - 1) {
    learnState.currentStep++;
    learnRenderStep();
  } else {
    // Chapter complete
    if (!learnState.completedChapters.includes(learnState.currentChapter)) {
      learnState.completedChapters.push(learnState.currentChapter);
    }
    showNotification('Chapter Complete!', `You finished ${chapter.title}`);
    learnShowChapterSelect();
  }
}

function learnPrevStep() {
  if (learnState.currentStep > 0) {
    learnState.currentStep--;
    learnRenderStep();
  }
}

// Event listeners
document.addEventListener('DOMContentLoaded', () => {
  document.getElementById('lpBack')?.addEventListener('click', learnShowChapterSelect);
  document.getElementById('lpNext')?.addEventListener('click', learnNextStep);
  document.getElementById('lpPrev')?.addEventListener('click', learnPrevStep);
});
