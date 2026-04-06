// ChessQ Professional Landing - Interactive Module
// Black & Green Theme - Minimalist Interactions

document.addEventListener('DOMContentLoaded', function() {
  
  // === Mobile Navigation Toggle ===
  const mobileToggle = document.querySelector('.mobile-toggle');
  const navLinks = document.querySelector('.nav-links');
  
  if (mobileToggle && navLinks) {
    mobileToggle.addEventListener('click', () => {
      navLinks.classList.toggle('mobile-open');
      mobileToggle.classList.toggle('open');
      
      // Animate hamburger
      const spans = mobileToggle.querySelectorAll('span');
      if (mobileToggle.classList.contains('open')) {
        spans[0].style.transform = 'rotate(45deg) translateY(8px)';
        spans[1].style.opacity = '0';
        spans[2].style.transform = 'rotate(-45deg) translateY(-8px)';
      } else {
        spans[0].style.transform = 'none';
        spans[1].style.opacity = '1';
        spans[2].style.transform = 'none';
      }
    });
  }
  
  // === Smooth Scroll for Anchor Links ===
  document.querySelectorAll('a[href^="#"]').forEach(link => {
    link.addEventListener('click', (e) => {
      const href = link.getAttribute('href');
      if (href === '#') return;
      
      e.preventDefault();
      const target = document.querySelector(href);
      if (target) {
        target.scrollIntoView({ 
          behavior: 'smooth', 
          block: 'start' 
        });
        
        // Close mobile menu if open
        if (navLinks && navLinks.classList.contains('mobile-open')) {
          navLinks.classList.remove('mobile-open');
          mobileToggle.classList.remove('open');
        }
      }
    });
  });
  
  // === Stats Counter Animation ===
  function animateCounter(element, target, duration = 2000) {
    const start = 0;
    const increment = target / (duration / 16);
    let current = start;
    
    const timer = setInterval(() => {
      current += increment;
      if (current >= target) {
        element.textContent = formatStatValue(target);
        clearInterval(timer);
      } else {
        element.textContent = formatStatValue(Math.floor(current));
      }
    }, 16);
  }
  
  function formatStatValue(value) {
    if (value >= 1000) {
      return (value / 1000).toFixed(1) + 'K+';
    }
    return value + '%';
  }
  
  // Intersection Observer for Stats
  const statsObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        const statValues = entry.target.querySelectorAll('.stat-value');
        statValues.forEach(stat => {
          const text = stat.textContent;
          let target = 0;
          
          if (text.includes('K+')) {
            target = parseFloat(text) * 1000;
          } else if (text.includes('%')) {
            target = parseFloat(text);
          }
          
          if (target > 0) {
            animateCounter(stat, target);
          }
        });
        statsObserver.unobserve(entry.target);
      }
    });
  }, { threshold: 0.5 });
  
  const heroStats = document.querySelector('.hero-stats');
  if (heroStats) {
    statsObserver.observe(heroStats);
  }
  
  // === Card Hover Effects ===
  const cards = document.querySelectorAll('.feature-card, .profile-card');
  cards.forEach(card => {
    card.addEventListener('mouseenter', () => {
      card.style.transform = 'translateY(-4px)';
    });
    
    card.addEventListener('mouseleave', () => {
      card.style.transform = 'translateY(0)';
    });
  });
  
  // === Scroll Reveal Animation ===
  const revealElements = document.querySelectorAll('.feature-card, .profile-card');
  const revealObserver = new IntersectionObserver((entries) => {
    entries.forEach((entry, index) => {
      if (entry.isIntersecting) {
        setTimeout(() => {
          entry.target.style.opacity = '1';
          entry.target.style.transform = 'translateY(0)';
        }, index * 100);
        revealObserver.unobserve(entry.target);
      }
    });
  }, { 
    threshold: 0.1,
    rootMargin: '0px 0px -50px 0px'
  });
  
  revealElements.forEach(el => {
    el.style.opacity = '0';
    el.style.transform = 'translateY(30px)';
    el.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
    revealObserver.observe(el);
  });
  
  // === Navbar Scroll Effect ===
  let lastScroll = 0;
  const navbar = document.querySelector('.navbar');
  
  window.addEventListener('scroll', () => {
    const currentScroll = window.pageYOffset;
    
    if (currentScroll > 100) {
      navbar.style.background = 'rgba(10, 10, 10, 0.98)';
      navbar.style.boxShadow = '0 4px 6px -1px rgba(0, 255, 136, 0.1)';
    } else {
      navbar.style.background = 'rgba(10, 10, 10, 0.95)';
      navbar.style.boxShadow = 'none';
    }
    
    lastScroll = currentScroll;
  });
  
  // === Button Ripple Effect ===
  const buttons = document.querySelectorAll('.btn-primary, .btn-secondary, .nav-link-cta');
  buttons.forEach(button => {
    button.addEventListener('click', function(e) {
      const ripple = document.createElement('span');
      const rect = this.getBoundingClientRect();
      const size = Math.max(rect.width, rect.height);
      const x = e.clientX - rect.left - size / 2;
      const y = e.clientY - rect.top - size / 2;
      
      ripple.style.width = ripple.style.height = size + 'px';
      ripple.style.left = x + 'px';
      ripple.style.top = y + 'px';
      ripple.classList.add('ripple');
      
      this.appendChild(ripple);
      
      setTimeout(() => ripple.remove(), 600);
    });
  });
  
  // === SSL Badge Pulse ===
  const sslBadge = document.querySelector('.ssl-badge');
  if (sslBadge) {
    setInterval(() => {
      sslBadge.style.transform = 'scale(1.05)';
      setTimeout(() => {
        sslBadge.style.transform = 'scale(1)';
      }, 200);
    }, 5000);
  }
  
  // === Preload Play Page ===
  const playLinks = document.querySelectorAll('a[href="/play.html"]');
  playLinks.forEach(link => {
    link.addEventListener('mouseenter', () => {
      const preloadLink = document.createElement('link');
      preloadLink.rel = 'prefetch';
      preloadLink.href = '/play.html';
      document.head.appendChild(preloadLink);
    }, { once: true });
  });
  
  console.log('ChessQ Professional Landing - Black & Green Theme Loaded ✓');
});

// Add ripple CSS dynamically
const style = document.createElement('style');
style.textContent = `
  .ripple {
    position: absolute;
    border-radius: 50%;
    background: rgba(0, 255, 136, 0.3);
    transform: scale(0);
    animation: ripple-animation 0.6s ease-out;
    pointer-events: none;
  }
  
  @keyframes ripple-animation {
    to {
      transform: scale(4);
      opacity: 0;
    }
  }
  
  @media (max-width: 768px) {
    .nav-links.mobile-open {
      display: flex;
      flex-direction: column;
      position: absolute;
      top: 100%;
      left: 0;
      right: 0;
      background: rgba(10, 10, 10, 0.98);
      padding: 1rem;
      border-top: 1px solid rgba(0, 255, 136, 0.1);
      animation: slideDown 0.3s ease-out;
    }
    
    @keyframes slideDown {
      from {
        opacity: 0;
        transform: translateY(-10px);
      }
      to {
        opacity: 1;
        transform: translateY(0);
      }
    }
  }
`;
document.head.appendChild(style);
