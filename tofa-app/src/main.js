const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// --- Views ---
const views = {
  locked: document.getElementById('view-locked'),
  unlocked: document.getElementById('view-unlocked'),
  settings: document.getElementById('view-settings'),
  camera: document.getElementById('view-camera'),
  'add-confirm': document.getElementById('view-add-confirm'),
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

function renderEntries(entries) {
  otpList.innerHTML = '';
  liveRows = [];
  const now = Date.now();

  entries.forEach(entry => {
    const el = document.createElement('div');
    el.className = 'otp-entry';

    const row = document.createElement('div');
    row.className = 'otp-row';
    const nameEl = document.createElement('div');
    nameEl.className = 'otp-name';
    if (entry.account) {
      nameEl.innerHTML =
        `<span class="otp-issuer">${entry.issuer}</span>` +
        `<span class="otp-account">${entry.account}</span>`;
    } else {
      nameEl.innerHTML = `<span class="otp-issuer">${entry.issuer}</span>`;
    }
    const codeEl = document.createElement('span');
    codeEl.className = 'otp-code';
    codeEl.textContent = entry.code;
    row.appendChild(nameEl);
    row.appendChild(codeEl);

    const tooltip = document.createElement('div');
    tooltip.className = 'otp-tooltip';
    const rows = [
      entry.issuer  ? `<span class="tt-row"><span class="tt-label">Issuer</span><span>${entry.issuer}</span></span>` : '',
      entry.account ? `<span class="tt-row"><span class="tt-label">Account</span><span>${entry.account}</span></span>` : '',
      `<span class="tt-row"><span class="tt-label">Algorithm</span><span>${entry.algorithm}</span></span>`,
      `<span class="tt-row"><span class="tt-label">Digits</span><span>${entry.digits}</span></span>`,
      `<span class="tt-row"><span class="tt-label">Period</span><span>${entry.period}s</span></span>`,
      `<span class="tt-row"><span class="tt-label">Added</span><span>${entry.created_at}</span></span>`,
      `<button class="tt-delete" data-name="${entry.name}">Delete</button>`,
    ];
    tooltip.innerHTML = rows.join('');

    const barWrap = document.createElement('div');
    barWrap.className = 'otp-bar-wrap';
    const bar = document.createElement('div');
    bar.className = 'otp-bar';
    barWrap.appendChild(bar);

    el.appendChild(row);
    el.appendChild(tooltip);
    el.appendChild(barWrap);

    el.addEventListener('click', async (e) => {
      if (e.target.classList.contains('tt-delete')) {
        try {
          await invoke('delete_entry', { name: e.target.dataset.name });
          const entries = await invoke('get_entries');
          renderEntries(entries);
        } catch (err) {
          console.error('delete failed:', err);
        }
        return;
      }
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
  row.barEl.style.width = pct.toFixed(3) + '%';
  row.barEl.className = 'otp-bar' + (urgent ? ' urgent' : '');
}

let rafHandle = null;
let apiRefreshTimer = null;

async function refreshCodes() {
  try {
    const fresh = await invoke('get_entries');
    const refreshNow = Date.now();
    for (const row of liveRows) {
      const updated = fresh.find(e => e.name === row.entry.name);
      if (updated) {
        row.entry = updated;
        row.expiresAt = refreshNow + updated.seconds_left * 1000;
        row.codeEl.textContent = updated.code;
      }
    }
  } catch (_) {
    stopCountdown();
    showView('locked');
    setTimeout(() => inputPassphrase.focus(), 50);
  }
}

function stopCountdown() {
  if (rafHandle) { cancelAnimationFrame(rafHandle); rafHandle = null; }
  if (apiRefreshTimer) { clearInterval(apiRefreshTimer); apiRefreshTimer = null; }
}

function startCountdown(entries) {
  stopCountdown();
  renderEntries(entries);

  let refreshing = false;

  // Smooth bar via rAF — triggers immediate refresh on expiry
  function tick() {
    const now = Date.now();
    let anyExpired = false;
    for (const row of liveRows) {
      updateRow(row, now);
      if (now >= row.expiresAt) anyExpired = true;
    }
    if (anyExpired && !refreshing) {
      refreshing = true;
      refreshCodes().finally(() => { refreshing = false; });
    }
    rafHandle = requestAnimationFrame(tick);
  }
  rafHandle = requestAnimationFrame(tick);

  // Background refresh every 10s as safety net
  apiRefreshTimer = setInterval(refreshCodes, 10_000);
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
    showView('locked');
    setTimeout(() => inputPassphrase.focus(), 50);
  } catch (err) {
    settingsError.textContent = err;
    settingsError.classList.remove('hidden');
  }
});

// --- Scan Screen ---
document.getElementById('btn-scan-screen').addEventListener('click', async () => {
  const btn = document.getElementById('btn-scan-screen');
  btn.textContent = '⏳';
  btn.disabled = true;
  try {
    const added = await invoke('scan_screen');
    const entries = await invoke('get_entries');
    startCountdown(entries);
    showView('unlocked');
    if (added.length > 1) alert(`Added ${added.length} entries: ${added.join(', ')}`);
  } catch (err) {
    alert(err);
  } finally {
    btn.textContent = '🖥';
    btn.disabled = false;
  }
});

// --- Scan Camera ---
let cameraStream = null;
let cameraTimer = null;

document.getElementById('btn-scan-camera').addEventListener('click', () => startCamera());
document.getElementById('btn-camera-back').addEventListener('click', () => stopCamera());

async function startCamera() {
  showView('camera');
  const video = document.getElementById('camera-video');
  const status = document.getElementById('camera-status');
  try {
    cameraStream = await navigator.mediaDevices.getUserMedia({ video: { facingMode: 'environment' } });
    video.srcObject = cameraStream;
    status.textContent = 'Point camera at a QR code…';
    cameraTimer = setInterval(captureFrame, 400);
  } catch (err) {
    status.textContent = 'Camera unavailable: ' + err.message;
  }
}

function stopCamera() {
  clearInterval(cameraTimer);
  cameraTimer = null;
  if (cameraStream) {
    cameraStream.getTracks().forEach(t => t.stop());
    cameraStream = null;
  }
  showView('unlocked');
}

async function captureFrame() {
  const video = document.getElementById('camera-video');
  const canvas = document.getElementById('camera-canvas');
  if (video.readyState < 2) return;
  canvas.width = video.videoWidth;
  canvas.height = video.videoHeight;
  canvas.getContext('2d').drawImage(video, 0, 0);
  const dataUrl = canvas.toDataURL('image/png');
  const base64 = dataUrl.split(',')[1];
  try {
    const uri = await invoke('scan_image_data', { data: base64 });
    stopCamera();
    openAddConfirm(uri);
  } catch (_) { /* no QR yet */ }
}

// --- Add confirm ---
let pendingUri = null;

function openAddConfirm(uri) {
  pendingUri = uri;
  const info = document.getElementById('add-confirm-info');
  const nameInput = document.getElementById('input-entry-name');
  const addError = document.getElementById('add-error');
  addError.classList.add('hidden');
  try {
    const url = new URL(uri);
    const label = decodeURIComponent(url.pathname.replace(/^\/\/totp\//, ''));
    const issuer = url.searchParams.get('issuer') || '';
    info.textContent = issuer ? `${issuer} · ${label}` : label;
    nameInput.value = issuer && label ? `${issuer}:${label}` : label;
  } catch (_) {
    info.textContent = uri;
    nameInput.value = '';
  }
  showView('add-confirm');
  setTimeout(() => nameInput.focus(), 50);
}

document.getElementById('btn-add-back').addEventListener('click', () => showView('unlocked'));

document.getElementById('btn-add-confirm').addEventListener('click', async () => {
  const name = document.getElementById('input-entry-name').value.trim();
  const addError = document.getElementById('add-error');
  addError.classList.add('hidden');
  try {
    await invoke('add_from_uri', { uri: pendingUri, name });
    pendingUri = null;
    const entries = await invoke('get_entries');
    renderEntries(entries);
    showView('unlocked');
  } catch (err) {
    addError.textContent = err;
    addError.classList.remove('hidden');
  }
});

// --- Tray events ---
listen('tray-action', (event) => {
  const action = event.payload;
  if (action === 'scan-screen') {
    document.getElementById('btn-scan-screen').click();
  } else if (action === 'scan-camera') {
    document.getElementById('btn-scan-camera').click();
  } else if (action === 'settings') {
    openSettings(views.unlocked.classList.contains('hidden') ? 'locked' : 'unlocked');
  }
});

// --- Init ---
showView('locked');
setTimeout(() => inputPassphrase.focus(), 50);
