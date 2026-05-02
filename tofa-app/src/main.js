const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// ── Shared SVG icon strings ──────────────────────────
const SVG_COPY = `<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>`;
const SVG_CHECK = `<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>`;

// --- Views ---
const views = {
  locked: document.getElementById('view-locked'),
  unlocked: document.getElementById('view-unlocked'),
  settings: document.getElementById('view-settings'),
  camera: document.getElementById('view-camera'),
  'add-confirm': document.getElementById('view-add-confirm'),
  'manual-add': document.getElementById('view-manual-add'),
};

function showView(name) {
  Object.values(views).forEach(v => v.classList.add('hidden'));
  views[name].classList.remove('hidden');
}

// --- Locked view ---
const formUnlock = document.getElementById('form-unlock');
const inputPassphrase = document.getElementById('input-passphrase');
const unlockError = document.getElementById('unlock-error');

const SVG_EYE = `<svg id="eye-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>`;
const SVG_EYE_OFF = `<svg id="eye-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>`;

document.getElementById('btn-toggle-pass').addEventListener('click', () => {
  const isHidden = inputPassphrase.type === 'password';
  inputPassphrase.type = isHidden ? 'text' : 'password';
  document.getElementById('btn-toggle-pass').innerHTML = isHidden ? SVG_EYE_OFF : SVG_EYE;
  document.getElementById('btn-toggle-pass').setAttribute('aria-label', isHidden ? 'Hide passphrase' : 'Show passphrase');
});

let isCreateMode = false;
let unlockAborted = false;

// ── Loader bar ───────────────────────────────────────
let _loaderCount = 0;
const _loaderEl = document.getElementById('loader-bar');

function loaderShow() {
  _loaderCount++;
  _loaderEl.classList.add('active');
}

function loaderHide() {
  _loaderCount = Math.max(0, _loaderCount - 1);
  if (_loaderCount === 0) _loaderEl.classList.remove('active');
}

async function call(cmd, args) {
  loaderShow();
  try { return await invoke(cmd, args); }
  finally { loaderHide(); }
}

async function initVaultMode() {
  const exists = await invoke('vault_exists');
  isCreateMode = !exists;
  document.getElementById('label-passphrase').textContent = exists ? 'Passphrase' : 'Choose a passphrase';
  document.getElementById('btn-unlock-submit').textContent = exists ? 'Unlock' : 'Create vault';
  document.getElementById('confirm-wrap').classList.toggle('hidden', exists);
  inputPassphrase.focus();
}

formUnlock.addEventListener('submit', async (e) => {
  e.preventDefault();
  unlockError.classList.add('hidden');
  const passphrase = inputPassphrase.value;

  const submitBtn = document.getElementById('btn-unlock-submit');
  submitBtn._savedLabel = submitBtn.textContent;
  unlockAborted = false;
  loaderShow();
  submitBtn.classList.add('pressed');
  await new Promise(r => setTimeout(r, 180));
  submitBtn.classList.remove('pressed');
  if (unlockAborted) { loaderHide(); return; }
  submitBtn.disabled = true;
  submitBtn.textContent = '…';

  if (isCreateMode) {
    const confirm = document.getElementById('input-passphrase-confirm').value;
    if (passphrase !== confirm) {
      unlockError.textContent = 'Passphrases do not match.';
      unlockError.classList.remove('hidden');
      submitBtn.disabled = false;
      submitBtn.textContent = submitBtn._savedLabel;
      return;
    }
    if (passphrase.length < 4) {
      unlockError.textContent = 'Passphrase must be at least 4 characters.';
      unlockError.classList.remove('hidden');
      submitBtn.disabled = false;
      submitBtn.textContent = submitBtn._savedLabel;
      return;
    }
    try {
      const entries = await call('create_vault', { passphrase });
      inputPassphrase.value = '';
      document.getElementById('input-passphrase-confirm').value = '';
      startCountdown(entries);
      showView('unlocked');
      showToast('Vault created — add your first OTP!', 'success', 4000);
    } catch (err) {
      unlockError.textContent = err;
      unlockError.classList.remove('hidden');
      submitBtn.disabled = false;
      submitBtn.textContent = submitBtn._savedLabel;
    } finally {
      loaderHide();
    }
  } else {
    try {
      const entries = await call('unlock', { passphrase });
      inputPassphrase.value = '';
      startCountdown(entries);
      showView('unlocked');
    } catch (err) {
      unlockError.textContent = err;
      unlockError.classList.remove('hidden');
      inputPassphrase.value = '';
      inputPassphrase.focus();
      submitBtn.disabled = false;
      submitBtn.textContent = submitBtn._savedLabel;
    } finally {
      loaderHide();
    }
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

  if (entries.length === 0) {
    const empty = document.createElement('div');
    empty.className = 'empty-state';
    empty.innerHTML = `
      <svg class="empty-logo" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
        <circle cx="32" cy="32" r="22" stroke="#4493f8" stroke-width="5"/>
        <path d="M 32 10 A 22 22 0 1 1 10 32" stroke="#e6edf3" stroke-width="5" stroke-linecap="round" fill="none"/>
        <circle cx="10" cy="32" r="3.5" fill="#e6edf3"/>
        <rect x="20" y="22" width="24" height="4.5" rx="2.25" fill="#e6edf3"/>
        <rect x="29.75" y="26.5" width="4.5" height="16" rx="2.25" fill="#e6edf3"/>
      </svg>
      <p class="empty-title">No entries yet</p>
      <p class="empty-sub">Scan a QR code or drop an image below<br>to add your first account.</p>
      <div class="drop-zone" id="empty-drop-zone">
        <input type="file" accept="image/*" id="empty-file-input" tabindex="-1">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="16 16 12 12 8 16"/>
          <line x1="12" y1="12" x2="12" y2="21"/>
          <path d="M20.39 18.39A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.3"/>
        </svg>
        <span>Drop a QR image here, or click to browse</span>
      </div>
      <div class="empty-actions">
        <button class="empty-action-btn" id="empty-btn-screen">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/></svg>
          Scan screen
        </button>
        <button class="empty-action-btn" id="empty-btn-camera">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"/><circle cx="12" cy="13" r="4"/></svg>
          Camera
        </button>
        <button class="empty-action-btn" id="empty-btn-manual">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
          Manual
        </button>
      </div>`;
    otpList.appendChild(empty);

    // Wire up drop zone
    setupDropZone(
      document.getElementById('empty-drop-zone'),
      document.getElementById('empty-file-input')
    );
    document.getElementById('empty-btn-screen').addEventListener('click', () =>
      document.getElementById('btn-scan-screen').click());
    document.getElementById('empty-btn-camera').addEventListener('click', () =>
      document.getElementById('btn-scan-camera').click());
    document.getElementById('empty-btn-manual').addEventListener('click', openManualAdd);
    return;
  }

  entries.forEach(entry => {
    const el = document.createElement('div');
    el.className = 'otp-entry';

    const row = document.createElement('div');
    row.className = 'otp-row';
    const nameEl = document.createElement('div');
    nameEl.className = 'otp-name';
    const issuerEl = document.createElement('span');
    issuerEl.className = 'otp-issuer';
    issuerEl.textContent = entry.issuer;
    nameEl.appendChild(issuerEl);
    if (entry.account) {
      const accountEl = document.createElement('span');
      accountEl.className = 'otp-account';
      accountEl.textContent = entry.account;
      nameEl.appendChild(accountEl);
    }
    const codeEl = document.createElement('span');
    codeEl.className = 'otp-code';
    codeEl.textContent = entry.code;

    const secsEl = document.createElement('span');
    secsEl.className = 'otp-secs';
    secsEl.textContent = entry.seconds_left + 's';

    const copyBtn = document.createElement('button');
    copyBtn.className = 'otp-copy-btn icon-btn';
    copyBtn.title = 'Copy';
    copyBtn.innerHTML = SVG_COPY;
    copyBtn.addEventListener('click', async (e) => {
      e.stopPropagation();
      await call('copy_code', { name: entry.name });
      copyBtn.innerHTML = SVG_CHECK;
      setTimeout(() => { copyBtn.innerHTML = SVG_COPY; }, 1200);
    });

    const rightEl = document.createElement('div');
    rightEl.className = 'otp-right';
    rightEl.appendChild(codeEl);
    rightEl.appendChild(secsEl);
    rightEl.appendChild(copyBtn);

    row.appendChild(nameEl);
    row.appendChild(rightEl);


    const barWrap = document.createElement('div');
    barWrap.className = 'otp-bar-wrap';
    const bar = document.createElement('div');
    bar.className = 'otp-bar';
    barWrap.appendChild(bar);

    el.appendChild(row);
    el.appendChild(barWrap);

    el.addEventListener('click', (e) => {
      if (!e.target.classList.contains('otp-copy-btn')) {
        openDetailModal(entry);
      }
    });
    otpList.appendChild(el);

    liveRows.push({
      entry,
      expiresAt: now + entry.seconds_left * 1000,
      barEl: bar,
      codeEl,
      secsEl,
    });
  });
}

function updateRow(row, now) {
  const msLeft = Math.max(0, row.expiresAt - now);
  const secsLeft = Math.ceil(msLeft / 1000);
  const pct = (msLeft / (row.entry.period * 1000)) * 100;
  const urgent = msLeft <= 5000;
  row.barEl.style.width = pct.toFixed(3) + '%';
  row.barEl.className = 'otp-bar' + (urgent ? ' urgent' : '');
  row.secsEl.textContent = secsLeft + 's';
  row.secsEl.className = 'otp-secs' + (urgent ? ' urgent' : '');
}

let rafHandle = null;
let apiRefreshTimer = null;

async function refreshCodes() {
  try {
    const fresh = await call('get_entries');
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

async function handleImageFile(file) {
  if (!file || !file.type.startsWith('image/')) return;
  const arrayBuffer = await file.arrayBuffer();
  const bytes = Array.from(new Uint8Array(arrayBuffer));
  try {
    showToast('Analyzing image…', 'info', 2000);
    const added = await call('scan_image_bytes', { bytes });
    showToast(`Added: ${added.join(', ')}`, 'success', 4000);
    const entries = await call('get_entries');
    startCountdown(entries);
  } catch (err) {
    showToast(err, 'error', 4000);
  }
}

function setupDropZone(zone, fileInput) {
  zone.addEventListener('dragover', e => {
    e.preventDefault();
    zone.classList.add('drag-over');
  });
  zone.addEventListener('dragleave', () => zone.classList.remove('drag-over'));
  zone.addEventListener('drop', e => {
    e.preventDefault();
    zone.classList.remove('drag-over');
    const file = e.dataTransfer.files[0];
    handleImageFile(file);
  });
  fileInput.addEventListener('change', () => {
    handleImageFile(fileInput.files[0]);
    fileInput.value = '';
  });
}

// Global drag & drop on unlocked view (when entries exist)
const viewUnlocked = document.getElementById('view-unlocked');
viewUnlocked.addEventListener('dragover', e => {
  e.preventDefault();
  viewUnlocked.classList.add('drag-over-global');
});
viewUnlocked.addEventListener('dragleave', e => {
  if (!viewUnlocked.contains(e.relatedTarget))
    viewUnlocked.classList.remove('drag-over-global');
});
viewUnlocked.addEventListener('drop', e => {
  viewUnlocked.classList.remove('drag-over-global');
  e.preventDefault();
  handleImageFile(e.dataTransfer.files[0]);
});

// Header image upload button
const headerFileInput = document.getElementById('header-file-input');
document.getElementById('btn-image-upload').addEventListener('click', () => headerFileInput.click());
headerFileInput.addEventListener('change', () => {
  handleImageFile(headerFileInput.files[0]);
  headerFileInput.value = '';
});

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
    const s = await call('get_settings');
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

document.getElementById('btn-reload-vault').addEventListener('click', async () => {
  const btn = document.getElementById('btn-reload-vault');
  btn.disabled = true;
  try {
    const entries = await call('get_entries');
    startCountdown(entries);
    showView('unlocked');
    showToast('Vault reloaded', 'success');
  } catch (err) {
    showToast(err, 'error');
    btn.disabled = false;
  }
});

formSettings.addEventListener('submit', async (e) => {
  e.preventDefault();
  settingsError.classList.add('hidden');
  try {
    await call('save_settings', {
      settings: { vault_path: document.getElementById('input-vault-path').value }
    });
    showView('locked');
    initVaultMode();
  } catch (err) {
    settingsError.textContent = err;
    settingsError.classList.remove('hidden');
  }
});

// --- Toast helper ---
let toastTimer = null;
function showToast(msg, type = '', duration = 3000) {
  const el = document.getElementById('toast');
  el.textContent = msg;
  el.className = 'toast' + (type ? ' ' + type : '');
  el.classList.remove('hidden');
  if (toastTimer) clearTimeout(toastTimer);
  toastTimer = setTimeout(() => el.classList.add('hidden'), duration);
}

// --- Scan Screen ---
const SVG_MONITOR = `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2" ry="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/></svg>`;

// Force CSS animation to replay by swapping the element
function setCountdown(text) {
  const el = document.getElementById('scan-countdown');
  const fresh = el.cloneNode(false);
  fresh.textContent = text;
  el.replaceWith(fresh);
  fresh.id = 'scan-countdown';
}

// Rust emits this once the window is back and QR analysis begins
listen('scan-step', (event) => {
  const overlay = document.getElementById('scan-overlay');
  overlay.classList.add('analyzing');
  setCountdown('');
  const label = document.getElementById('scan-label');
  label.classList.remove('scan-step-in');
  void label.offsetWidth; // reflow to restart animation
  label.textContent = event.payload;
  label.classList.add('scan-step-in');
});

document.getElementById('btn-scan-screen').addEventListener('click', async () => {
  const btn = document.getElementById('btn-scan-screen');
  const overlay = document.getElementById('scan-overlay');
  btn.disabled = true;
  overlay.classList.remove('analyzing', 'hidden');

  // Animated countdown while window is still visible
  for (const [digit, label] of [['3', 'Capturing screen…'], ['2', 'Capturing screen…'], ['1', 'Capturing screen…']]) {
    setCountdown(digit);
    document.getElementById('scan-label').textContent = label;
    await new Promise(r => setTimeout(r, 500));
  }
  // Window disappears here (scan_screen hides it), reappears with spinner + "Analyzing…"

  try {
    const added = await call('scan_screen');
    const entries = await call('get_entries');
    startCountdown(entries);
    showView('unlocked');
    if (added.length === 1) {
      showToast(`Added: ${added[0]}`, 'success');
    } else if (added.length > 1) {
      showToast(`Added ${added.length} entries`, 'success');
    }
  } catch (err) {
    showView('unlocked');
    showToast(err, 'error');
  } finally {
    overlay.classList.add('hidden');
    overlay.classList.remove('analyzing');
    btn.disabled = false;
  }
});

// --- Scan Camera ---
let cameraStream = null;
let cameraTimer = null;

document.getElementById('btn-scan-camera').addEventListener('click', () => startCamera());
document.getElementById('btn-camera-back').addEventListener('click', () => stopCamera());

let cameraSent = false;
let steadyCount = 0;

async function startCamera() {
  showView('camera');
  cameraSent = false;
  steadyCount = 0;
  const video = document.getElementById('camera-video');
  const status = document.getElementById('camera-status');
  try {
    cameraStream = await navigator.mediaDevices.getUserMedia({
      video: { facingMode: 'environment', width: { ideal: 1280 } }
    });
    video.srcObject = cameraStream;
    await video.play();
    status.textContent = 'Scanning for QR code…';
    cameraTick();
  } catch (err) {
    status.textContent = 'Camera unavailable: ' + err.message;
  }
}

function stopCamera() {
  cameraSent = true;
  if (cameraTimer) { cancelAnimationFrame(cameraTimer); cameraTimer = null; }
  if (cameraStream) {
    cameraStream.getTracks().forEach(t => t.stop());
    cameraStream = null;
  }
  showView('unlocked');
}

function cameraTick() {
  if (cameraSent) return;
  const video = document.getElementById('camera-video');
  if ('BarcodeDetector' in window) {
    const bd = new BarcodeDetector({ formats: ['qr_code'] });
    bd.detect(video).then(codes => {
      if (codes.length > 0) onCameraFound(codes[0].rawValue);
      else { steadyCount = 0; cameraTimer = requestAnimationFrame(cameraTick); }
    }).catch(() => { cameraTimer = requestAnimationFrame(cameraTick); });
  } else {
    if (!window._jsqrLoaded) {
      window._jsqrLoaded = true;
      const s = document.createElement('script');
      s.src = 'https://cdn.jsdelivr.net/npm/jsqr@1.4.0/dist/jsQR.min.js';
      s.onload = cameraTick;
      document.head.appendChild(s);
      return;
    }
    if (typeof jsQR === 'undefined') { cameraTimer = requestAnimationFrame(cameraTick); return; }
    const canvas = document.createElement('canvas');
    canvas.width = video.videoWidth; canvas.height = video.videoHeight;
    canvas.getContext('2d').drawImage(video, 0, 0);
    const img = canvas.getContext('2d').getImageData(0, 0, canvas.width, canvas.height);
    const code = jsQR(img.data, img.width, img.height);
    if (code) onCameraFound(code.data);
    else { steadyCount = 0; cameraTimer = requestAnimationFrame(cameraTick); }
  }
}

function onCameraFound(uri) {
  if (cameraSent) return;
  steadyCount++;
  const status = document.getElementById('camera-status');
  if (steadyCount < 3) {
    status.textContent = 'QR detected — hold steady…';
    cameraTimer = requestAnimationFrame(cameraTick);
    return;
  }

  // Validate it's an OTP URI before proceeding
  if (!uri.startsWith('otpauth://') && !uri.startsWith('otpauth-migration://')) {
    status.textContent = 'Not an OTP QR code. Try again.';
    steadyCount = 0;
    cameraTimer = requestAnimationFrame(cameraTick);
    return;
  }

  cameraSent = true;
  status.textContent = 'Got it!';
  if (cameraStream) { cameraStream.getTracks().forEach(t => t.stop()); cameraStream = null; }

  if (uri.startsWith('otpauth-migration://')) {
    // Migration QR — import all accounts directly, no confirm screen needed
    importMigrationUri(uri);
  } else {
    openAddConfirm(uri);
  }
}

async function importMigrationUri(uri) {
  showView('unlocked');
  showToast('Importing…', '', 60000);
  try {
    const added = await call('add_from_uri', { uri, name: '' });
    const entries = await call('get_entries');
    renderEntries(entries);
    showToast(
      Array.isArray(added) && added.length > 1
        ? `Added ${added.length} entries`
        : `Added: ${Array.isArray(added) ? added[0] : added}`,
      'success'
    );
  } catch (err) {
    showToast(err, 'error');
  }
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

// --- Manual add view ---
function openManualAdd() {
  document.getElementById('input-manual-uri').value = '';
  document.getElementById('input-manual-name').value = '';
  document.getElementById('manual-error').classList.add('hidden');
  showView('manual-add');
  setTimeout(() => document.getElementById('input-manual-uri').focus(), 50);
}

document.getElementById('btn-manual-add').addEventListener('click', openManualAdd);
document.getElementById('btn-manual-back').addEventListener('click', () => showView('unlocked'));

document.getElementById('btn-manual-confirm').addEventListener('click', async () => {
  const rawInput = document.getElementById('input-manual-uri').value.trim();
  const name = document.getElementById('input-manual-name').value.trim();
  const manualError = document.getElementById('manual-error');
  const btn = document.getElementById('btn-manual-confirm');
  manualError.classList.add('hidden');

  if (!rawInput) {
    manualError.textContent = 'Enter a URI or Base32 secret.';
    manualError.classList.remove('hidden');
    return;
  }

  // Wrap a bare Base32 secret into a minimal otpauth URI
  const uri = rawInput.startsWith('otpauth') || rawInput.startsWith('otpauth-migration')
    ? rawInput
    : `otpauth://totp/${encodeURIComponent(name || 'Account')}?secret=${rawInput.replace(/\s/g, '').toUpperCase()}`;

  btn.textContent = 'Saving…';
  btn.disabled = true;
  try {
    const added = await call('add_from_uri', { uri, name });
    const entries = await call('get_entries');
    startCountdown(entries);
    showView('unlocked');
    showToast(`Added: ${added.join(', ')}`, 'success');
  } catch (err) {
    manualError.textContent = err;
    manualError.classList.remove('hidden');
    btn.textContent = 'Add';
    btn.disabled = false;
  }
});

document.getElementById('btn-add-confirm').addEventListener('click', async () => {
  const name = document.getElementById('input-entry-name').value.trim();
  const addError = document.getElementById('add-error');
  const btn = document.getElementById('btn-add-confirm');
  addError.classList.add('hidden');
  btn.textContent = 'Saving…';
  btn.disabled = true;
  try {
    const added = await call('add_from_uri', { uri: pendingUri, name });
    pendingUri = null;
    const entries = await call('get_entries');
    renderEntries(entries);
    showView('unlocked');
    showToast(`Added: ${added[0] || name || 'entry'}`, 'success');
  } catch (err) {
    addError.textContent = err;
    addError.classList.remove('hidden');
    btn.textContent = 'Add';
    btn.disabled = false;
  }
});

// --- OTP Detail Modal ---
let modalEntry = null;
let modalRaf = null;

function openDetailModal(entry) {
  modalEntry = entry;
  document.getElementById('modal-issuer').textContent = entry.issuer;
  document.getElementById('modal-account').textContent = entry.account;
  document.getElementById('md-issuer-row').style.display = entry.issuer ? '' : 'none';
  document.getElementById('md-account-row').style.display = entry.account ? '' : 'none';
  document.getElementById('md-issuer').textContent = entry.issuer;
  document.getElementById('md-account').textContent = entry.account;
  document.getElementById('modal-code').textContent = entry.code;
  document.getElementById('md-algorithm').textContent = entry.algorithm;
  document.getElementById('md-digits').textContent = entry.digits;
  document.getElementById('md-period').textContent = entry.period + 's';
  document.getElementById('md-created').textContent = entry.created_at;
  document.getElementById('modal-delete').dataset.name = entry.name;
  document.getElementById('modal-overlay').classList.remove('hidden');

  function tickModal() {
    const row = liveRows.find(r => r.entry.name === modalEntry.name);
    if (!row) return;
    const now = Date.now();
    const msLeft = Math.max(0, row.expiresAt - now);
    const pct = (msLeft / (row.entry.period * 1000)) * 100;
    document.getElementById('modal-bar').style.width = pct.toFixed(3) + '%';
    document.getElementById('modal-code').textContent = row.codeEl.textContent;
    modalRaf = requestAnimationFrame(tickModal);
  }
  modalRaf = requestAnimationFrame(tickModal);
}

function closeDetailModal() {
  document.getElementById('modal-overlay').classList.add('hidden');
  if (modalRaf) { cancelAnimationFrame(modalRaf); modalRaf = null; }
  modalEntry = null;
}

document.getElementById('modal-close').addEventListener('click', closeDetailModal);
document.getElementById('modal-overlay').addEventListener('click', (e) => {
  if (e.target === document.getElementById('modal-overlay')) closeDetailModal();
});
document.getElementById('modal-delete').addEventListener('click', async (e) => {
  const btn = document.getElementById('modal-delete');
  const name = btn.dataset.name;
  btn.textContent = 'Deleting…';
  btn.disabled = true;
  try {
    await call('delete_entry', { name });
    closeDetailModal();
    btn.textContent = 'Delete';
    btn.disabled = false;
    const entries = await call('get_entries');
    renderEntries(entries);
    showToast(`Deleted: ${name}`, 'success');
  } catch (err) {
    btn.textContent = 'Delete';
    btn.disabled = false;
    showToast(err, 'error');
  }
});

document.getElementById('modal-copy').addEventListener('click', async () => {
  if (!modalEntry) return;
  await call('copy_code', { name: modalEntry.name });
  const btn = document.getElementById('modal-copy');
  btn.innerHTML = SVG_CHECK;
  setTimeout(() => { btn.innerHTML = SVG_COPY; }, 1200);
});

async function doLock() {
  unlockAborted = true;
  _loaderCount = 0;
  _loaderEl.classList.remove('active');
  try { await invoke('lock'); } catch (_) {}
  stopCountdown();
  const submitBtn = document.getElementById('btn-unlock-submit');
  submitBtn.disabled = false;
  submitBtn.classList.remove('pressed');
  submitBtn.textContent = submitBtn._savedLabel ?? 'Unlock';
  showView('locked');
  setTimeout(() => inputPassphrase.focus(), 50);
}

document.getElementById('btn-lock').addEventListener('click', doLock);

// --- Tray events ---
listen('tray-action', (event) => {
  const action = event.payload;
  if (action === 'scan-screen') {
    document.getElementById('btn-scan-screen').click();
  } else if (action === 'scan-camera') {
    document.getElementById('btn-scan-camera').click();
  } else if (action === 'settings') {
    openSettings(views.unlocked.classList.contains('hidden') ? 'locked' : 'unlocked');
  } else if (action === 'lock') {
    doLock();
  }
});

// --- Init ---
showView('locked');
initVaultMode();
