/* global OTP */
'use strict';

const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// ── State ──────────────────────────────────────────────────────────────────
let entries = [];
let filteredEntries = [];
let selectedName = null;
let tickInterval = null;

// ── DOM refs ───────────────────────────────────────────────────────────────
const $ = id => document.getElementById(id);

function bufToBase64(buf) {
  let binary = '';
  const bytes = new Uint8Array(buf);
  for (let i = 0; i < bytes.length; i++) binary += String.fromCharCode(bytes[i]);
  return btoa(binary);
}

// ── Theme ──────────────────────────────────────────────────────────────────
const mq = window.matchMedia('(prefers-color-scheme: light)');

function applyTheme(theme) {
  const isLight = theme === 'light' || (theme === 'system' && mq.matches);
  document.documentElement.setAttribute('data-theme', isLight ? 'light' : 'dark');
}

mq.addEventListener('change', () => {
  const current = document.documentElement.getAttribute('data-theme-pref') || 'system';
  if (current === 'system') applyTheme('system');
});

// ── View router ────────────────────────────────────────────────────────────
function setLogoEye(open) {
  const symbol = open ? '#tofa-open' : '#tofa-wink';
  const animClass = open ? 'eye-opening' : 'eye-closing';
  const removeClass = open ? 'eye-closing' : 'eye-opening';
  const logos = ['logo-hero', 'logo-header-locked', 'logo-header-list'];
  logos.forEach(id => {
    const el = $(id);
    if (!el) return;
    const use = el.querySelector('use');
    if (!use) return;
    if (use.getAttribute('href') === symbol) return;
    use.setAttribute('href', symbol);
    el.classList.remove(removeClass, 'eye-opening', 'eye-closing');
    el.classList.add(animClass);
    el.addEventListener('animationend', () => el.classList.remove(animClass), { once: true });
  });
}

function showView(id) {
  document.querySelectorAll('.view').forEach(v => v.classList.remove('active'));
  $(id).classList.add('active');
  if (id === 'view-locked') setLogoEye(false);
}

// ── Loader ─────────────────────────────────────────────────────────────────
function loaderStart() { $('loader-bar-inner').style.width = '60%'; }
function loaderDone() {
  $('loader-bar-inner').style.width = '100%';
  setTimeout(() => { $('loader-bar-inner').style.width = '0%'; }, 300);
}

// ── Toast ──────────────────────────────────────────────────────────────────
let toastTimer;
function toast(msg, error = false) {
  const el = $('toast');
  el.textContent = msg;
  el.className = 'toast visible' + (error ? ' error' : '');
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => { el.className = 'toast'; }, 2500);
}

// ── Init ───────────────────────────────────────────────────────────────────
async function init() {
  try {
    const s = await invoke('get_settings');
    applyTheme(s.theme || 'system');
    document.documentElement.setAttribute('data-theme-pref', s.theme || 'system');
  } catch (_) {
    applyTheme('system');
  }
  const exists = await invoke('vault_exists');
  $('confirm-wrap').style.display = exists ? 'none' : 'block';
  $('btn-unlock-submit').textContent = exists ? 'Unlock' : 'Create vault';
  showView('view-locked');
  $('input-passphrase').focus();
}

// ── Render list ────────────────────────────────────────────────────────────
function renderList(data) {
  entries = data;
  applyFilter($('search-input').value);
}

function applyFilter(query) {
  const q = query.toLowerCase();
  filteredEntries = q
    ? entries.filter(e =>
        e.name.toLowerCase().includes(q) ||
        e.issuer.toLowerCase().includes(q) ||
        e.account.toLowerCase().includes(q))
    : entries.slice();

  const list = $('account-list');
  list.innerHTML = '';
  filteredEntries.forEach(entry => {
    const item = document.createElement('div');
    item.className = 'account-item';
    item.dataset.name = entry.name;
    const initial = (entry.issuer || entry.account || '?')[0].toUpperCase();
    const secs = entry.seconds_left ?? OTP.secondsRemaining(entry.period);
    const timerColor = secs < 5 ? 'var(--danger)' : secs < 10 ? 'var(--warning)' : 'var(--brand)';
    item.innerHTML = `
      <div class="account-icon">${initial}</div>
      <div style="flex:1;min-width:0;overflow:hidden;">
        <div class="account-name" style="white-space:nowrap;overflow:hidden;text-overflow:ellipsis;">${entry.issuer || entry.name}</div>
        <div class="account-login" style="white-space:nowrap;overflow:hidden;text-overflow:ellipsis;">${entry.account}</div>
      </div>
      <div style="display:flex;align-items:center;gap:var(--s-2);flex-shrink:0;padding-left:var(--s-2);">
        <div class="item-code-col" style="text-align:right;cursor:pointer;" title="Click to copy">
          <div style="font-family:var(--font-mono);font-weight:700;font-size:14px;letter-spacing:0.08em;color:${timerColor}">${entry.code}</div>
          <div style="font-family:var(--font-mono);font-size:10px;color:${timerColor}" data-timer="${entry.name}">${secs}s</div>
        </div>
        <button class="btn btn-ghost btn-copy-item" data-name="${entry.name}" style="padding:4px 6px;font-size:14px;flex-shrink:0;" title="Copy code">⎘</button>
      </div>`;
    item.addEventListener('click', e => {
      if (e.target.closest('.btn-copy-item') || e.target.closest('.item-code-col')) return;
      openDetail(entry.name);
    });
    item.querySelector('.btn-copy-item').addEventListener('click', async e => {
      e.stopPropagation();
      try {
        await invoke('copy_code', { name: entry.name });
        toast('Copied!');
      } catch (err) { toast(String(err), true); }
    });
    item.querySelector('.item-code-col').addEventListener('click', async e => {
      e.stopPropagation();
      try {
        await invoke('copy_code', { name: entry.name });
        toast('Copied!');
      } catch (err) { toast(String(err), true); }
    });
    list.appendChild(item);
  });

  const badge = $('list-badge');
  badge.textContent = filteredEntries.length ? `${filteredEntries.length}` : '';
}

// ── Tick (countdown) ───────────────────────────────────────────────────────
function startTick() {
  stopTick();
  tick();
  tickInterval = setInterval(tick, 1000);
}

function stopTick() {
  if (tickInterval) { clearInterval(tickInterval); tickInterval = null; }
}

function tick() {
  const secs = OTP.secondsRemaining();
  const globalBar = $('global-progress-bar');
  const pct = (secs / 30) * 100;
  globalBar.style.setProperty('--progress', `${pct}%`);
  globalBar.style.background = secs < 5 ? 'var(--danger)' : secs < 10 ? 'var(--warning)' : 'var(--brand)';

  // Update per-item timers in the list
  document.querySelectorAll('[data-timer]').forEach(el => {
    const s = OTP.secondsRemaining();
    const color = s < 5 ? 'var(--danger)' : s < 10 ? 'var(--warning)' : 'var(--brand)';
    el.textContent = `${s}s`;
    el.style.color = color;
    const codeEl = el.previousElementSibling;
    if (codeEl) codeEl.style.color = color;
  });

  if (secs <= 1) { refreshEntries(); return; }

  if (selectedName) {
    const entry = entries.find(e => e.name === selectedName);
    if (entry) {
      const period = entry.period || 30;
      const s = OTP.secondsRemaining(period);
      $('detail-seconds').textContent = `${s}s remaining`;
      const bar = $('detail-progress-bar');
      bar.style.setProperty('--progress', `${(s / period) * 100}%`);
      bar.style.background = s < 5 ? 'var(--danger)' : s < 10 ? 'var(--warning)' : 'var(--brand)';
      $('detail-code').style.color = s < 5 ? 'var(--danger)' : s < 10 ? 'var(--warning)' : 'var(--brand)';
    }
  }
}

async function refreshEntries() {
  try {
    const data = await invoke('get_entries');
    renderList(data);
    if (selectedName) {
      const entry = data.find(e => e.name === selectedName);
      if (entry) updateDetailCode(entry);
    }
  } catch (_) {}
}

// ── Detail view ────────────────────────────────────────────────────────────
function openDetail(name) {
  const entry = entries.find(e => e.name === name);
  if (!entry) return;
  selectedName = name;
  const initial = (entry.issuer || entry.account || '?')[0].toUpperCase();
  $('detail-title').textContent = entry.issuer || entry.name;
  $('detail-icon').textContent = initial;
  updateDetailCode(entry);
  renderDetailMeta(entry);
  $('reveal-overlay').style.display = 'none';
  showView('view-detail');
}

function renderDetailMeta(entry) {
  const rows = [
    ['Account',   entry.account || '—'],
    ['Issuer',    entry.issuer  || '—'],
    ['Algorithm', entry.algorithm || 'SHA1'],
    ['Digits',    String(entry.digits || 6)],
    ['Period',    `${entry.period || 30}s`],
    ['Added',     entry.created_at || '—'],
    ['ID',        entry.id || '—'],
    ['Secret',    null],
  ];
  const tbody = $('detail-meta-body');
  tbody.innerHTML = '';
  rows.forEach(([label, value]) => {
    const tr = document.createElement('tr');
    if (label === 'Secret') {
      tr.innerHTML = `<td>${label}</td><td><span class="secret-masked" id="secret-cell" title="Click to reveal">●●●●●●●●●●●●●●●●</span></td>`;
      tbody.appendChild(tr);
      $('secret-cell').addEventListener('click', () => {
        $('reveal-passphrase').value = '';
        $('reveal-error').style.display = 'none';
        $('reveal-overlay').style.display = 'flex';
        $('reveal-passphrase').focus();
      });
    } else {
      tr.innerHTML = `<td>${label}</td><td>${value}</td>`;
      tbody.appendChild(tr);
    }
  });
}

function updateDetailCode(entry) {
  $('detail-code').textContent = entry.code;
  const period = entry.period || 30;
  const secs = OTP.secondsRemaining(period);
  $('detail-seconds').textContent = `${secs}s remaining`;
  const bar = $('detail-progress-bar');
  bar.style.setProperty('--progress', `${(secs / period) * 100}%`);
}

// ── Settings view ──────────────────────────────────────────────────────────
const ADD_INNER_HTML = (() => {
  const wrap = $('view-add-content');
  return wrap ? wrap.innerHTML : '';
})();

function restoreAddView() {
  const wrap = $('view-add-content');
  if (wrap) wrap.innerHTML = ADD_INNER_HTML;
  bindAddListeners();
}

async function openSettings() {
  let settings;
  try { settings = await invoke('get_settings'); } catch (_) { settings = { vault_path: '', theme: 'system' }; }
  const theme = settings.theme || 'system';

  const wrap = $('view-add-content');
  if (!wrap) return;
  wrap.innerHTML = `
    <h3 style="font-family:var(--font-mono);font-size:12px;text-transform:uppercase;letter-spacing:0.1em;color:var(--text-muted);margin-bottom:var(--s-4);">Settings</h3>
    <label class="input-label" for="settings-vault-path">Vault path</label>
    <div style="display:flex;gap:var(--s-2);margin-bottom:var(--s-3);">
      <input id="settings-vault-path" class="input" style="flex:1;font-size:12px;">
      <button id="btn-browse-vault" class="btn btn-secondary" style="white-space:nowrap;font-size:12px;">Browse…</button>
    </div>
    <label class="input-label" style="margin-bottom:var(--s-3);">Appearance</label>
    <div style="display:flex;gap:var(--s-2);margin-bottom:var(--s-4);">
      <button data-theme-btn="light" style="flex:1;border:2px solid ${theme==='light'?'var(--brand)':'var(--border)'};border-radius:var(--r-md);padding:var(--s-2);cursor:pointer;background:#f5f5f7;transition:border-color 0.2s;">
        <div style="height:28px;background:#ffffff;border-radius:4px;margin-bottom:4px;border:1px solid #e0e0e0;"></div>
        <span style="font-family:var(--font-mono);font-size:10px;color:#333;display:block;text-align:center;">Light</span>
      </button>
      <button data-theme-btn="dark" style="flex:1;border:2px solid ${theme==='dark'?'var(--brand)':'var(--border)'};border-radius:var(--r-md);padding:var(--s-2);cursor:pointer;background:#1a1a2e;transition:border-color 0.2s;">
        <div style="height:28px;background:#0d0d1a;border-radius:4px;margin-bottom:4px;border:1px solid #333;"></div>
        <span style="font-family:var(--font-mono);font-size:10px;color:#aaa;display:block;text-align:center;">Dark</span>
      </button>
      <button data-theme-btn="system" style="flex:1;border:2px solid ${theme==='system'?'var(--brand)':'var(--border)'};border-radius:var(--r-md);padding:var(--s-2);cursor:pointer;background:linear-gradient(135deg,#f5f5f7 50%,#1a1a2e 50%);transition:border-color 0.2s;">
        <div style="height:28px;border-radius:4px;margin-bottom:4px;background:linear-gradient(135deg,#ffffff 50%,#0d0d1a 50%);border:1px solid #888;"></div>
        <span style="font-family:var(--font-mono);font-size:10px;color:var(--text-muted);display:block;text-align:center;">Auto</span>
      </button>
    </div>
    <button id="btn-settings-save" class="btn btn-primary" style="width:100%;margin-bottom:var(--s-2);">Save</button>
    <p id="settings-error" style="font-family:var(--font-mono);font-size:11px;color:var(--danger);display:none;"></p>`;

  $('settings-vault-path').value = settings.vault_path;

  // Theme card toggle
  let selectedTheme = theme;
  wrap.querySelectorAll('[data-theme-btn]').forEach(btn => {
    btn.addEventListener('click', () => {
      selectedTheme = btn.dataset.themeBtn;
      wrap.querySelectorAll('[data-theme-btn]').forEach(b => {
        b.style.borderColor = b === btn ? 'var(--brand)' : 'var(--border)';
      });
      applyTheme(selectedTheme);
    });
  });

  $('btn-browse-vault').addEventListener('click', async () => {
    try {
      const picked = await invoke('pick_vault_folder');
      if (picked) $('settings-vault-path').value = picked;
    } catch (_) {}
  });

  $('btn-settings-save').addEventListener('click', async () => {
    const vault_path = $('settings-vault-path').value.trim();
    try {
      await invoke('save_settings', { settings: { vault_path, theme: selectedTheme } });
      document.documentElement.setAttribute('data-theme-pref', selectedTheme);
      toast('Settings saved');
      showView('view-locked');
      init();
    } catch (err) {
      const errEl = $('settings-error');
      errEl.textContent = String(err);
      errEl.style.display = '';
    }
  });

  showView('view-add');
}

// ── Add view listeners ─────────────────────────────────────────────────────
function bindAddListeners() {
  const btnSubmit = $('btn-add-submit');
  if (btnSubmit) {
    btnSubmit.addEventListener('click', async () => {
      const uri = $('input-uri').value.trim();
      const errEl = $('add-error');
      errEl.style.display = 'none';
      if (!uri) { errEl.textContent = 'Enter an otpauth:// URI.'; errEl.style.display = ''; return; }
      loaderStart();
      try {
        const added = await invoke('add_from_uri', { uri, name: '' });
        $('input-uri').value = '';
        const data = await invoke('get_entries');
        renderList(data);
        showView('view-list');
        toast(`Added: ${added.join(', ')}`);
      } catch (err) {
        errEl.textContent = String(err);
        errEl.style.display = '';
      } finally { loaderDone(); }
    });
  }

  const btnScan = $('btn-scan-screen');
  if (btnScan) {
    btnScan.addEventListener('click', async () => {
      for (const n of [3, 2, 1]) {
        toast(`Scanning in ${n}…`);
        await new Promise(r => setTimeout(r, 1000));
      }
      loaderStart();
      try {
        const added = await invoke('scan_screen');
        const data = await invoke('get_entries');
        renderList(data);
        showView('view-list');
        startTick();
        toast(`Added: ${added.join(', ')}`);
      } catch (err) { toast(String(err), true); }
      finally { loaderDone(); }
    });
  }

  const btnCam = $('btn-scan-camera');
  if (btnCam) {
    btnCam.addEventListener('click', async () => {
      toast('Opening camera in browser…');
      loaderStart();
      try {
        const added = await invoke('scan_camera');
        const data = await invoke('get_entries');
        renderList(data);
        showView('view-list');
        startTick();
        toast(`Added: ${added.join(', ')}`);
      } catch (err) { toast(String(err), true); }
      finally { loaderDone(); }
    });
  }

  const btnFile = $('btn-open-file');
  if (btnFile) {
    btnFile.addEventListener('click', () => {
      const input = document.createElement('input');
      input.type = 'file';
      input.accept = 'image/*,.json,.txt,.zip';
      input.onchange = async () => {
        const file = input.files[0];
        if (!file) return;
        const buf = await file.arrayBuffer();
        const b64 = bufToBase64(buf);
        loaderStart();
        try {
          const added = await invoke('import_file', { filename: file.name, b64 });
          const data = await invoke('get_entries');
          renderList(data);
          showView('view-list');
          toast(`Added: ${added.join(', ')}`);
        } catch (err) { toast(String(err), true); }
        finally { loaderDone(); }
      };
      input.click();
    });
  }

}

// ── Static event listeners ─────────────────────────────────────────────────

$('form-unlock').addEventListener('submit', async e => {
  e.preventDefault();
  const passphrase = $('input-passphrase').value;
  const errEl = $('unlock-error');
  errEl.style.visibility = 'hidden';
  errEl.textContent = '';
  setLogoEye(true);
  await new Promise(r => setTimeout(r, 1000));
  loaderStart();
  const closeEyeAfterDelay = () => setLogoEye(false);

  try {
    const vaultExists = await invoke('vault_exists');
    let data;
    if (vaultExists) {
      data = await invoke('unlock', { passphrase });
    } else {
      const confirm = $('input-passphrase-confirm').value;
      if (passphrase !== confirm) {
        errEl.textContent = 'Passphrases do not match.';
        errEl.style.visibility = 'visible';
        loaderDone();
        closeEyeAfterDelay();
        return;
      }
      data = await invoke('create_vault', { passphrase });
    }
    renderList(data);
    startTick();
    showView('view-list');
  } catch (err) {
    errEl.textContent = String(err);
    errEl.style.visibility = 'visible';
    closeEyeAfterDelay(); // wrong passphrase — close eye after delay
  } finally {
    loaderDone();
    $('input-passphrase').value = '';
    $('input-passphrase-confirm').value = '';
  }
});

$('btn-lock').addEventListener('click', async () => {
  stopTick();
  entries = [];
  selectedName = null;
  try { await invoke('lock'); } catch (_) {}
  init();
});

$('btn-add').addEventListener('click', () => {
  restoreAddView();
  showView('view-add');
});

$('btn-settings').addEventListener('click', () => openSettings());
$('btn-settings-locked').addEventListener('click', () => openSettings());

$('btn-detail-back').addEventListener('click', () => {
  selectedName = null;
  showView('view-list');
});

$('btn-add-back').addEventListener('click', () => {
  restoreAddView();
  showView('view-list');
});

$('btn-detail-copy').addEventListener('click', async () => {
  if (!selectedName) return;
  try {
    await invoke('copy_code', { name: selectedName });
    toast('Copied!');
  } catch (err) { toast(String(err), true); }
});

$('btn-detail-del').addEventListener('click', async () => {
  if (!selectedName) return;
  const overlay = $('blocking-overlay');
  overlay.style.display = 'flex';
  loaderStart();
  try {
    await invoke('delete_entry', { name: selectedName });
    selectedName = null;
    const data = await invoke('get_entries');
    renderList(data);
    showView('view-list');
    toast('Deleted');
  } catch (err) { toast(String(err), true); }
  finally {
    overlay.style.display = 'none';
    loaderDone();
  }
});

$('btn-detail-qr').addEventListener('click', () => { toast('QR export coming soon'); });

$('btn-reveal-cancel').addEventListener('click', () => {
  $('reveal-overlay').style.display = 'none';
  $('reveal-passphrase').value = '';
});

$('btn-reveal-confirm').addEventListener('click', async () => {
  const passphrase = $('reveal-passphrase').value;
  const errEl = $('reveal-error');
  errEl.style.display = 'none';
  try {
    const secret = await invoke('get_secret', { name: selectedName, passphrase });
    $('reveal-overlay').style.display = 'none';
    $('reveal-passphrase').value = '';
    // Show secret in cell, truncate after 30s
    const cell = $('secret-cell');
    cell.textContent = secret;
    cell.className = '';
    cell.style.color = 'var(--brand)';
    setTimeout(() => {
      cell.textContent = '●●●●●●●●●●●●●●●●';
      cell.className = 'secret-masked';
      cell.style.color = '';
    }, 30000);
  } catch (err) {
    errEl.textContent = String(err);
    errEl.style.display = '';
  }
});

$('reveal-passphrase').addEventListener('keydown', e => {
  if (e.key === 'Enter') $('btn-reveal-confirm').click();
  if (e.key === 'Escape') $('btn-reveal-cancel').click();
});

$('search-input').addEventListener('input', e => applyFilter(e.target.value));

// Tray menu events
listen('tray-action', ({ payload }) => {
  if (payload === 'lock') {
    stopTick();
    entries = [];
    selectedName = null;
    invoke('lock').catch(() => {});
    init();
  } else if (payload === 'settings') {
    openSettings();
  } else if (payload === 'scan-screen') {
    restoreAddView();
    showView('view-add');
    setTimeout(() => { const btn = $('btn-scan-screen'); if (btn) btn.click(); }, 100);
  } else if (payload === 'scan-camera') {
    const btnCam = $('btn-scan-camera');
    if (btnCam) btnCam.click();
  }
});

// Keyboard shortcuts
document.addEventListener('keydown', e => {
  if ((e.metaKey || e.ctrlKey) && e.key === 'n') { e.preventDefault(); $('btn-add').click(); }
  if ((e.metaKey || e.ctrlKey) && e.key === 'k') { e.preventDefault(); $('search-input').focus(); }
  if (e.key === 'Escape' && selectedName) { $('btn-detail-back').click(); }
});


// Scan step progress (emitted by scan_screen command)
listen('scan-step', ({ payload }) => { toast(String(payload)); });

// ── Boot ───────────────────────────────────────────────────────────────────
bindAddListeners();
init();
