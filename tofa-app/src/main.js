const { invoke } = window.__TAURI__.core;

// --- Views ---
const views = {
  locked: document.getElementById('view-locked'),
  unlocked: document.getElementById('view-unlocked'),
  settings: document.getElementById('view-settings'),
};

function showView(name) {
  Object.values(views).forEach(v => v.classList.add('hidden'));
  views[name].classList.remove('hidden');
}

// --- Locked view ---
const formUnlock = document.getElementById('form-unlock');
const inputPassphrase = document.getElementById('input-passphrase');
const unlockError = document.getElementById('unlock-error');

formUnlock.addEventListener('submit', async (e) => {
  e.preventDefault();
  unlockError.classList.add('hidden');
  try {
    const entries = await invoke('unlock', { passphrase: inputPassphrase.value });
    inputPassphrase.value = '';
    startCountdown(entries);
    showView('unlocked');
  } catch (err) {
    unlockError.textContent = err;
    unlockError.classList.remove('hidden');
  }
});

// --- Unlocked view ---
const otpList = document.getElementById('otp-list');
const btnSettings = document.getElementById('btn-settings');

// Live state: array of { entry, expiresAt, barEl, codeEl }
// expiresAt = Date.now() + seconds_left * 1000
let liveRows = [];
let countdownTimer = null;

function renderEntries(entries) {
  otpList.innerHTML = '';
  liveRows = [];
  const now = Date.now();

  entries.forEach(entry => {
    const el = document.createElement('div');
    el.className = 'otp-entry';

    const row = document.createElement('div');
    row.className = 'otp-row';
    const nameEl = document.createElement('span');
    nameEl.className = 'otp-name';
    nameEl.textContent = entry.name;
    const codeEl = document.createElement('span');
    codeEl.className = 'otp-code';
    codeEl.textContent = entry.code;
    row.appendChild(nameEl);
    row.appendChild(codeEl);

    const barWrap = document.createElement('div');
    barWrap.className = 'otp-bar-wrap';
    const bar = document.createElement('div');
    bar.className = 'otp-bar';
    barWrap.appendChild(bar);

    el.appendChild(row);
    el.appendChild(barWrap);

    el.addEventListener('click', async () => {
      try {
        await invoke('copy_code', { name: entry.name });
      } catch (err) {
        console.error('copy failed:', err);
      }
    });
    otpList.appendChild(el);

    liveRows.push({
      entry,
      expiresAt: now + entry.seconds_left * 1000,
      barEl: bar,
      codeEl,
    });
  });
}

function updateRow(row, now) {
  const msLeft = Math.max(0, row.expiresAt - now);
  const pct = (msLeft / (row.entry.period * 1000)) * 100;
  const urgent = msLeft <= 5000;
  row.barEl.style.width = pct.toFixed(2) + '%';
  row.barEl.className = 'otp-bar' + (urgent ? ' urgent' : '');
}

function startCountdown(entries) {
  if (countdownTimer) clearInterval(countdownTimer);
  renderEntries(entries);

  countdownTimer = setInterval(async () => {
    const now = Date.now();
    let needsRefresh = false;

    for (const row of liveRows) {
      updateRow(row, now);
      if (now >= row.expiresAt) needsRefresh = true;
    }

    if (needsRefresh) {
      try {
        const fresh = await invoke('get_entries');
        const refreshNow = Date.now();
        for (const row of liveRows) {
          const updated = fresh.find(e => e.name === row.entry.name);
          if (updated && updated.code !== row.codeEl.textContent) {
            row.entry = updated;
            row.expiresAt = refreshNow + updated.seconds_left * 1000;
            row.codeEl.textContent = updated.code;
          }
        }
      } catch (_) {
        clearInterval(countdownTimer);
        countdownTimer = null;
        showView('locked');
        setTimeout(() => inputPassphrase.focus(), 50);
      }
    }
  }, 100);
}

let settingsReturnView = 'locked';

async function openSettings(returnTo) {
  settingsReturnView = returnTo;
  try {
    const s = await invoke('get_settings');
    document.getElementById('input-vault-path').value = s.vault_path;
  } catch (_) {}
  showView('settings');
}

btnSettings.addEventListener('click', () => openSettings('unlocked'));
document.getElementById('btn-settings-locked').addEventListener('click', () => openSettings('locked'));

// --- Settings view ---
const formSettings = document.getElementById('form-settings');
const btnBack = document.getElementById('btn-back');
const settingsError = document.getElementById('settings-error');

btnBack.addEventListener('click', () => showView(settingsReturnView));

formSettings.addEventListener('submit', async (e) => {
  e.preventDefault();
  settingsError.classList.add('hidden');
  try {
    await invoke('save_settings', {
      settings: { vault_path: document.getElementById('input-vault-path').value }
    });
    showView('unlocked');
  } catch (err) {
    settingsError.textContent = err;
    settingsError.classList.remove('hidden');
  }
});

// --- Init ---
showView('locked');
setTimeout(() => inputPassphrase.focus(), 50);
