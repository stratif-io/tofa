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
    renderEntries(entries);
    startRefresh();
    showView('unlocked');
  } catch (err) {
    unlockError.textContent = err;
    unlockError.classList.remove('hidden');
  }
});

// --- Unlocked view ---
const otpList = document.getElementById('otp-list');
const btnSettings = document.getElementById('btn-settings');

let refreshTimer = null;

function renderEntries(entries) {
  otpList.innerHTML = '';
  entries.forEach(entry => {
    const pct = Math.round((entry.seconds_left / entry.period) * 100);
    const urgent = entry.seconds_left <= 5;

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
    bar.className = 'otp-bar' + (urgent ? ' urgent' : '');
    bar.style.width = pct + '%';
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
  });
}

async function refreshEntries() {
  try {
    const entries = await invoke('get_entries');
    renderEntries(entries);
  } catch (err) {
    // locked — go back to lock screen
    clearInterval(refreshTimer);
    refreshTimer = null;
    showView('locked');
    setTimeout(() => inputPassphrase.focus(), 50);
  }
}

function startRefresh() {
  if (refreshTimer) clearInterval(refreshTimer);
  refreshTimer = setInterval(refreshEntries, 10000);
}

btnSettings.addEventListener('click', async () => {
  try {
    const s = await invoke('get_settings');
    document.getElementById('input-vault-path').value = s.vault_path;
  } catch (_) {}
  showView('settings');
});

// --- Settings view ---
const formSettings = document.getElementById('form-settings');
const btnBack = document.getElementById('btn-back');
const settingsError = document.getElementById('settings-error');

btnBack.addEventListener('click', () => showView('unlocked'));

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
