'use strict';

// === Countdown & color logic ===

/** Seconds remaining in current 30s TOTP window */
function secondsRemaining(period = 30) {
  const now = Math.floor(Date.now() / 1000);
  return period - (now % period);
}

/**
 * Update a progress bar element and optional seconds label.
 * Applies brand→warning→danger color transition.
 */
function updateCountdown(barEl, labelEl, seconds, period = 30) {
  const ratio = seconds / period;
  const color = seconds >= 10 ? 'var(--brand)'
              : seconds >= 5  ? 'var(--warning)'
              :                  'var(--danger)';
  barEl.style.setProperty('--progress', `${ratio * 100}%`);
  barEl.style.background = color;
  if (labelEl) {
    labelEl.textContent = `${seconds}s remaining`;
    labelEl.style.color = color;
  }
}

// === Toast ===

let _toastTimer = null;

function showToast(message, variant = 'success') {
  const el = document.getElementById('toast');
  el.textContent = message;
  el.className = `toast visible${variant === 'error' ? ' error' : ''}`;
  clearTimeout(_toastTimer);
  _toastTimer = setTimeout(() => { el.className = 'toast'; }, 2000);
}

// === Exports ===
window.OTP = { secondsRemaining, updateCountdown, showToast };
