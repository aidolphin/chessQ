const locales = {
  en: {
    "play": "Play",
    "home": "Home",
    "kids_mode": "Kids Mode",
    "adult_mode": "Adult Mode",
    "standard_chess": "Standard Chess",
    "learning_mode": "Learning Mode",
    "custom_challenge": "Custom Challenge",
    "play_now": "Play Now",
    "start_learning": "Start Learning",
    "choose_game": "Choose Your Game",
    "select_game": "Select a game mode to start playing",
    "friends_online": "Friends Online",
    "active_players": "Active Players",
    "economy": "Your Economy",
    "ranking": "Your Ranking",
    "move_history": "Move History",
    "resign": "Resign",
    "offer_draw": "Offer Draw",
    "undo": "Undo",
    "share": "Share",
    "upgrade_premium": "Upgrade to Premium ($4.99/mo)",
    "unlock_levels": "Unlock All Levels ($4.99)",
    "select_time_control": "Select Time Control",
    "start_game": "Start Game",
    "choose_opponent": "Choose Your Opponent",
    "play_ai": "Play Against AI",
    "challenge_ai": "Challenge our intelligent chess bot",
    "play_human": "Play Against Human",
    "challenge_human": "Challenge a friend or online player",
    "choose_color": "Choose Your Color",
    "play_white": "Play as White",
    "play_black": "Play as Black",
    "random_color": "Random",
    "waiting_opponent": "Waiting for Opponent",
    "share_invite": "Share this link to invite a friend:",
    "waiting_join": "Waiting for someone to join..."
  },
  ne: {
    "play": "खेल्नुहोस्",
    "home": "गृहपृष्ठ",
    "kids_mode": "बच्चा मोड",
    "adult_mode": "वयस्क मोड",
    "standard_chess": "मानक चेस",
    "learning_mode": "सिक्ने मोड",
    "custom_challenge": "कस्टम चुनौती",
    "play_now": "अहिले खेल्नुहोस्",
    "start_learning": "सिक्न सुरु गर्नुहोस्",
    "choose_game": "आफ्नो खेल छान्नुहोस्",
    "select_game": "खेल्न सुरु गर्न खेल मोड चयन गर्नुहोस्",
    "friends_online": "अनलाइन साथीहरू",
    "active_players": "सक्रिय खेलाडीहरू",
    "economy": "तपाईंको अर्थव्यवस्था",
    "ranking": "तपाईंको श्रेणी",
    "move_history": "चाल इतिहास",
    "resign": "हार मान्नुहोस्",
    "offer_draw": "बराबर प्रस्ताव गर्नुहोस्",
    "undo": "फिर्ता लिनुहोस्",
    "share": "सेयर गर्नुहोस्",
    "upgrade_premium": "प्रिमियममा अपग्रेड गर्नुहोस् ($4.99/mo)",
    "unlock_levels": "सबै स्तर अनलक गर्नुहोस् ($4.99)",
    "select_time_control": "समय नियन्त्रण छान्नुहोस्",
    "start_game": "खेल सुरु गर्नुहोस्",
    "choose_opponent": "आफ्नो विपक्षी छान्नुहोस्",
    "play_ai": "कम्प्युटर (AI) सँग खेल्नुहोस्",
    "challenge_ai": "हाम्रो बुद्धिमान चेस बोटलाई चुनौती दिनुहोस्",
    "play_human": "मानवसँग खेल्नुहोस्",
    "challenge_human": "साथी वा अनलाइन खेलाडीलाई चुनौती दिनुहोस्",
    "choose_color": "आफ्नो रङ छान्नुहोस्",
    "play_white": "सेतो भएर खेल्नुहोस्",
    "play_black": "कालो भएर खेल्नुहोस्",
    "random_color": "अनियमित",
    "waiting_opponent": "विपक्षीको पर्खाइमा",
    "share_invite": "साथीलाई आमन्त्रित गर्न यो लिङ्क सेयर गर्नुहोस्:",
    "waiting_join": "कोही जोडिने प्रतीक्षामा..."
  }
};

function translateUI(lang) {
  document.querySelectorAll('[data-i18n]').forEach(el => {
    const key = el.getAttribute('data-i18n');
    if (locales[lang] && locales[lang][key]) {
      el.textContent = locales[lang][key];
    }
  });
  
  // Custom updates for specific elements if needed
  const modeBtn = document.getElementById('modeToggleBtn');
  if (modeBtn) {
    const currentMode = gameState.modeStyle || 'adult';
    modeBtn.textContent = currentMode === 'kids' 
      ? (locales[lang]['adult_mode'] || 'Adult Mode') 
      : (locales[lang]['kids_mode'] || 'Kids Mode');
  }
}
