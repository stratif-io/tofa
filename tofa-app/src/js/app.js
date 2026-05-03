'use strict';
const { invoke } = window.__TAURI__.core;
const { secondsRemaining, updateCountdown, showToast } = window.OTP;

// === View routing ===

const views = ['locked', 'list', 'detail', 'add'];

function showView(name) {
  views.forEach(v => {
    document.getElementById(`view-${v}`).classList.toggle('active', v === name);
  });
  if (name === 'list') startGlobalCountdown();
  if (name !== 'list') stopGlobalCountdown();
}

// === Global countdown (list view progress bar) ===

let _countdownInterval = null;

function startGlobalCountdown() {
  stopGlobalCountdown();
  const bar  = document.getElementById('global-progress-bar');
  const tick = () => { updateCountdown(bar, null, secondsRemaining()); };
  tick();
  _countdownInterval = setInterval(tick, 1000);
}

function stopGlobalCountdown() {
  clearInterval(_countdownInterval);
  _countdownInterval = null;
}

// === Account list rendering ===

let _accounts = [];

function renderAccountList(accounts, filter = '') {
  _accounts = accounts;
  const container = document.getElementById('account-list');
  const filtered = filter
    ? accounts.filter(a => (a.name || '').toLowerCase().includes(filter.toLowerCase()))
    : accounts;

  container.innerHTML = filtered.map((a, i) => {
    const initial = (a.issuer || a.name || '?')[0].toUpperCase();
    const secs = secondsRemaining(a.period || 30);
    const codeColor = secs >= 10 ? 'var(--brand)' : secs >= 5 ? 'var(--warning)' : 'var(--danger)';
    return `
      <div class="account-item" data-index="${i}" onclick="openDetail(${i})">
        <div class="account-icon">${initial}</div>
        <div>
          <div class="account-name">${escHtml(a.issuer || a.name)}</div>
          <div class="account-login">${escHtml(a.account || '')}</div>
        </div>
        <div class="account-code" style="color:${codeColor}">${escHtml(formatCode(a.code))}</div>
      </div>`;
  }).join('');

  document.getElementById('list-badge').textContent = `${filtered.length} accounts`;
}

// === Detail view ===

let _detailIndex = -1;
let _detailInterval = null;

function openDetail(index) {
  const a = _accounts[index];
  if (!a) return;
  _detailIndex = index;

  document.getElementById('detail-title').textContent   = a.issuer || a.name;
  document.getElementById('detail-account').textContent = a.account || '';
  document.getElementById('detail-icon').textContent    = (a.issuer || a.name || '?')[0].toUpperCase();

  const codeEl  = document.getElementById('detail-code');
  const barEl   = document.getElementById('detail-progress-bar');
  const labelEl = document.getElementById('detail-seconds');

  const refresh = () => {
    const secs = secondsRemaining(a.period || 30);
    codeEl.textContent = formatCode(a.code);
    updateCountdown(barEl, labelEl, secs, a.period || 30);
    codeEl.style.color = secs >= 10 ? 'var(--brand)' : secs >= 5 ? 'var(--warning)' : 'var(--danger)';
  };

  clearInterval(_detailInterval);
  refresh();
  _detailInterval = setInterval(refresh, 1000);
  showView('detail');
}

// === Unlock ===

document.getElementById('form-unlock').addEventListener('submit', async (e) => {
  e.preventDefault();
  const pass  = document.getElementById('input-passphrase').value;
  const errEl = document.getElementById('unlock-error');
  errEl.style.display = 'none';
  try {
    const accounts = await invoke('unlock_vault', { passphrase: pass });
    renderAccountList(accounts);
    showView('list');
  } catch (err) {
    errEl.textContent = err;
    errEl.style.display = 'block';
  }
});

// === Lock ===

document.getElementById('btn-lock').addEventListener('click', async () => {
  await invoke('lock_vault');
  stopGlobalCountdown();
  showView('locked');
  document.getElementById('input-passphrase').value = '';
});

// === Search ===

document.getElementById('search-input').addEventListener('input', (e) => {
  renderAccountList(_accounts, e.target.value);
});

// === Navigation ===

document.getElementById('btn-add').addEventListener('click', () => showView('add'));
document.getElementById('btn-add-back').addEventListener('click', () => showView('list'));
document.getElementById('btn-detail-back').addEventListener('click', () => {
  clearInterval(_detailInterval);
  showView('list');
});

// === Detail actions ===

document.getElementById('btn-detail-copy').addEventListener('click', async () => {
  const a = _accounts[_detailIndex];
  if (!a) return;
  try {
    await invoke('copy_code', { name: a.name });
    showToast(`Copied · ${a.issuer || a.name}`);
  } catch (err) {
    showToast(err, 'error');
  }
});

document.getElementById('btn-detail-del').addEventListener('click', async () => {
  const a = _accounts[_detailIndex];
  if (!a) return;
  if (!confirm(`Remove ${a.issuer || a.name}?`)) return;
  try {
    await invoke('remove_account', { name: a.name });
    clearInterval(_detailInterval);
    const accounts = await invoke('list_accounts');
    renderAccountList(accounts);
    showView('list');
    showToast(`Removed ${a.issuer || a.name}.`);
  } catch (err) {
    showToast(err, 'error');
  }
});

// === Add view ===

document.getElementById('btn-add-submit').addEventListener('click', async () => {
  const uri   = document.getElementById('input-uri').value.trim();
  const errEl = document.getElementById('add-error');
  errEl.style.display = 'none';
  if (!uri) { errEl.textContent = 'Enter an otpauth:// URI.'; errEl.style.display = 'block'; return; }
  try {
    await invoke('add_account_uri', { uri });
    const accounts = await invoke('list_accounts');
    renderAccountList(accounts);
    showView('list');
    showToast("Sir Wink's got it. 😉");
  } catch (err) {
    errEl.textContent = err;
    errEl.style.display = 'block';
  }
});

// Drop zone
const dropZone = document.getElementById('drop-zone');
dropZone.addEventListener('dragover', e => { e.preventDefault(); dropZone.classList.add('drag-over'); });
dropZone.addEventListener('dragleave', () => dropZone.classList.remove('drag-over'));
dropZone.addEventListener('drop', async (e) => {
  e.preventDefault();
  dropZone.classList.remove('drag-over');
  const file = e.dataTransfer.files[0];
  if (!file) return;
  try {
    const uri = await invoke('decode_qr_file', { path: file.path });
    document.getElementById('input-uri').value = uri;
  } catch (err) {
    showToast(err, 'error');
  }
});

document.getElementById('btn-scan-screen').addEventListener('click', async () => {
  try {
    const uri = await invoke('scan_screen');
    document.getElementById('input-uri').value = uri;
  } catch (err) {
    showToast(err, 'error');
  }
});

// === Helpers ===

function formatCode(code) {
  if (!code || code.length < 6) return code || '------';
  return `${code.slice(0, 3)} ${code.slice(3)}`;
}

function escHtml(str) {
  return String(str).replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
}

// === Init ===

(async () => {
  try {
    const isLocked = await invoke('is_vault_locked');
    if (!isLocked) {
      const accounts = await invoke('list_accounts');
      renderAccountList(accounts);
      showView('list');
    }
  } catch (_) {
    // Stay on locked view
  }
})();
