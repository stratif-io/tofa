/* global OTP, IssuerIcons */
'use strict';

// Render the icon HTML for an entry: brand SVG when we have one, otherwise
// the legacy initial-letter circle. Returns a string (innerHTML-friendly).
function entryIconHTML(entry) {
  const issuer = entry.issuer || entry.account || '';
  const icon = (window.IssuerIcons && window.IssuerIcons.iconForIssuer)
    ? window.IssuerIcons.iconForIssuer(issuer)
    : null;
  if (icon) {
    return `<svg class="account-icon-svg" viewBox="0 0 24 24" style="color:${icon.color}" aria-hidden="true"><use href="#${icon.id}"/></svg>`;
  }
  const initial = (issuer || '?')[0].toUpperCase();
  return `<div class="account-icon">${initial}</div>`;
}

const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// ── State ──────────────────────────────────────────────────────────────────
let entries = [];
let filteredEntries = [];
let selectedId = null;
let tickInterval = null;
let fromView = 'view-list'; // view to return to when pressing Back

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

function currentView() {
  const el = document.querySelector('.view.active');
  return el ? el.id : 'view-list';
}

// ── Loader ─────────────────────────────────────────────────────────────────
function loaderStart() { $('loader-bar-inner').style.width = '60%'; }
function loaderDone() {
  $('loader-bar-inner').style.width = '100%';
  setTimeout(() => { $('loader-bar-inner').style.width = '0%'; }, 300);
}

// ── Popover pinning ────────────────────────────────────────────────────────
// The Tauri popover hides on focus loss by default. For operations that
// legitimately steal focus (file picker, screen scan, camera scan, native
// folder dialog), pin the popover open while the operation runs so the
// user doesn't lose their place.
async function withPopoverPinned(fn) {
  try { await invoke('set_popover_pinned', { pinned: true }); } catch (_) {}
  try {
    return await fn();
  } finally {
    try { await invoke('set_popover_pinned', { pinned: false }); } catch (_) {}
  }
}

// Blocking overlay — high-visibility feedback for operations long enough
// that the user needs to know something is happening (unlock, scan,
// import). The thin loader bar is a secondary indicator.
function showBlocking(message) {
  const msg = $('blocking-overlay-message');
  if (msg) msg.textContent = message;
  $('blocking-overlay').classList.add('visible');
}
function hideBlocking() {
  $('blocking-overlay').classList.remove('visible');
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

  if (filteredEntries.length === 0) {
    const empty = document.createElement('div');
    empty.className = 'empty-state';
    if (q) {
      empty.innerHTML = `<p class="empty-state-title">No results</p><p class="empty-state-sub">No accounts match "<em>${q}</em>"</p>`;
    } else {
      empty.innerHTML = `
        <svg width="40" height="40" style="color:var(--text-muted);margin-bottom:var(--s-3)" viewBox="0 0 128 128"><use href="#tofa-wink"/></svg>
        <p class="empty-state-title">No accounts yet</p>
        <p class="empty-state-sub">Add your first TOTP account to get started</p>
        <button class="btn btn-primary empty-state-btn" id="empty-state-add">Add account</button>`;
    }
    list.appendChild(empty);
    if (!q) {
      empty.querySelector('#empty-state-add').addEventListener('click', () => {
        fromView = 'view-list';
        showView('view-add');
      });
    }
    return;
  }

  filteredEntries.forEach(entry => {
    const item = document.createElement('div');
    item.className = 'account-item';
    item.dataset.id = entry.id;
    const secs = entry.seconds_left ?? OTP.secondsRemaining(entry.period);
    const timerColor = secs < 5 ? 'var(--danger)' : secs < 10 ? 'var(--warning)' : 'var(--brand)';
    item.innerHTML = `
      ${entryIconHTML(entry)}
      <div style="flex:1;min-width:0;overflow:hidden;">
        <div class="account-name" style="white-space:nowrap;overflow:hidden;text-overflow:ellipsis;">${entry.issuer || entry.name}</div>
        <div class="account-login" style="white-space:nowrap;overflow:hidden;text-overflow:ellipsis;">${entry.account}</div>
      </div>
      <div style="display:flex;align-items:center;gap:var(--s-2);flex-shrink:0;padding-left:var(--s-2);">
        <div class="item-code-col" style="text-align:right;cursor:pointer;" title="Click to copy">
          <div style="font-family:var(--font-mono);font-weight:700;font-size:14px;letter-spacing:0.08em;color:${timerColor}">${entry.code}</div>
          <div style="font-family:var(--font-mono);font-size:10px;color:${timerColor}" data-timer="${entry.id}">${secs}s</div>
        </div>
        <button class="btn btn-ghost btn-copy-item" data-id="${entry.id}" style="padding:4px 6px;font-size:14px;flex-shrink:0;" title="Copy code">⎘</button>
      </div>`;
    item.addEventListener('click', e => {
      if (e.target.closest('.btn-copy-item') || e.target.closest('.item-code-col')) return;
      openDetail(entry.id);
    });
    item.querySelector('.btn-copy-item').addEventListener('click', async e => {
      e.stopPropagation();
      try {
        await invoke('copy_code', { id: entry.id });
        toast('Copied!');
      } catch (err) { toast(String(err), true); }
    });
    item.querySelector('.item-code-col').addEventListener('click', async e => {
      e.stopPropagation();
      try {
        await invoke('copy_code', { id: entry.id });
        toast('Copied!');
      } catch (err) { toast(String(err), true); }
    });
    list.appendChild(item);
  });

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

  if (selectedId) {
    const entry = entries.find(e => e.id === selectedId);
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
    if (selectedId) {
      const entry = data.find(e => e.id === selectedId);
      if (entry) updateDetailCode(entry);
    }
  } catch (_) {}
}

// ── Detail view ────────────────────────────────────────────────────────────
function openDetail(id) {
  const entry = entries.find(e => e.id === id);
  if (!entry) return;
  selectedId = id;
  $('detail-title').textContent = entry.issuer || entry.name;
  // Replace the detail icon container with our themed icon (SVG or initial).
  $('detail-icon').outerHTML =
    `<div id="detail-icon" class="account-icon-detail">${entryIconHTML(entry)}</div>`;
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

async function openSettings(from) {
  fromView = from ?? currentView();
  let settings;
  try { settings = await invoke('get_settings'); } catch (_) { settings = { vault_path: '', theme: 'system' }; }
  const theme = settings.theme || 'system';

  const wrap = $('view-add-content');
  if (!wrap) return;
  wrap.innerHTML = `
    <div class="settings-section">
      <span class="settings-section-label">Vault path</span>
      <div class="settings-row">
        <input id="settings-vault-path" class="input settings-path-input" placeholder="~/path/to/vault">
        <button id="btn-browse-vault" class="btn btn-secondary settings-browse-btn">Browse…</button>
      </div>
    </div>
    <div class="settings-section">
      <span class="settings-section-label">Appearance</span>
      <div class="settings-segmented">
        <button class="settings-seg-btn${theme==='light'?' active':''}" data-theme-btn="light">
          <svg class="seg-icon" width="12" height="12" viewBox="0 0 24 24"><use href="#icon-sun"/></svg>Light
        </button>
        <button class="settings-seg-btn${theme==='dark'?' active':''}" data-theme-btn="dark">
          <svg class="seg-icon" width="12" height="12" viewBox="0 0 24 24"><use href="#icon-moon"/></svg>Dark
        </button>
        <button class="settings-seg-btn${theme==='system'?' active':''}" data-theme-btn="system">
          <svg class="seg-icon" width="12" height="12" viewBox="0 0 24 24"><use href="#icon-monitor"/></svg>Auto
        </button>
      </div>
    </div>
    <button id="btn-settings-save" class="btn btn-primary settings-save-btn">Save</button>
    <p id="settings-error" class="settings-error"></p>`;

  $('settings-vault-path').value = settings.vault_path;

  // Theme segmented control
  let selectedTheme = theme;
  wrap.querySelectorAll('[data-theme-btn]').forEach(btn => {
    btn.addEventListener('click', () => {
      selectedTheme = btn.dataset.themeBtn;
      wrap.querySelectorAll('[data-theme-btn]').forEach(b => b.classList.toggle('active', b === btn));
      applyTheme(selectedTheme);
    });
  });

  $('btn-browse-vault').addEventListener('click', async () => {
    try {
      const picked = await withPopoverPinned(() => invoke('pick_vault_folder'));
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
      errEl.style.display = 'block';
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
      showBlocking('Scanning screen for QR codes…');
      try {
        const added = await withPopoverPinned(() => invoke('scan_screen'));
        const data = await invoke('get_entries');
        renderList(data);
        showView('view-list');
        startTick();
        toast(`Added: ${added.join(', ')}`);
      } catch (err) { toast(String(err), true); }
      finally { loaderDone(); hideBlocking(); }
    });
  }

  const btnCam = $('btn-scan-camera');
  if (btnCam) {
    btnCam.addEventListener('click', async () => {
      toast('Opening camera in browser…');
      loaderStart();
      showBlocking('Waiting for camera scan…');
      try {
        const added = await withPopoverPinned(() => invoke('scan_camera'));
        const data = await invoke('get_entries');
        renderList(data);
        showView('view-list');
        startTick();
        toast(`Added: ${added.join(', ')}`);
      } catch (err) { toast(String(err), true); }
      finally { loaderDone(); hideBlocking(); }
    });
  }

  const btnFile = $('btn-open-file');
  if (btnFile) {
    btnFile.addEventListener('click', async () => {
      loaderStart();
      showBlocking('Importing file…');
      try {
        const added = await withPopoverPinned(() => invoke('pick_and_import_file'));
        if (added.length === 0) return;
        const data = await invoke('get_entries');
        renderList(data);
        showView('view-list');
        toast(`Added: ${added.join(', ')}`);
      } catch (err) { toast(String(err), true); }
      finally { loaderDone(); hideBlocking(); }
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
  const btn = $('btn-unlock-submit');
  btn.classList.add('active');
  setTimeout(() => btn.classList.remove('active'), 200);
  loaderStart();
  setLogoEye(true);
  await new Promise(r => setTimeout(r, 1000));
  const closeEyeAfterDelay = () => setLogoEye(false);

  let vaultExists;
  try { vaultExists = await invoke('vault_exists'); } catch (_) { vaultExists = true; }
  showBlocking(vaultExists ? 'Decrypting vault…' : 'Creating vault…');

  try {
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
        hideBlocking();
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
    hideBlocking();
    $('input-passphrase').value = '';
    $('input-passphrase-confirm').value = '';
  }
});

$('btn-lock').addEventListener('click', async () => {
  stopTick();
  entries = [];
  filteredEntries = [];
  selectedId = null;
  fromView = 'view-list';
  try { await invoke('lock'); } catch (_) {}
  init();
});

$('btn-add').addEventListener('click', () => {
  fromView = 'view-list';
  restoreAddView();
  showView('view-add');
});

$('btn-settings').addEventListener('click', () => openSettings());
$('btn-settings-locked').addEventListener('click', () => openSettings());

$('btn-detail-back').addEventListener('click', () => {
  selectedId = null;
  showView('view-list');
});

$('btn-add-back').addEventListener('click', () => {
  restoreAddView();
  showView(fromView);
});

$('btn-detail-copy').addEventListener('click', async () => {
  if (!selectedId) return;
  try {
    await invoke('copy_code', { id: selectedId });
    toast('Copied!');
  } catch (err) { toast(String(err), true); }
});

$('btn-detail-del').addEventListener('click', async () => {
  if (!selectedId) return;
  showBlocking('Deleting…');
  loaderStart();
  try {
    await invoke('delete_entry', { id: selectedId });
    selectedId = null;
    const data = await invoke('get_entries');
    renderList(data);
    showView('view-list');
    toast('Deleted');
  } catch (err) { toast(String(err), true); }
  finally {
    hideBlocking();
    loaderDone();
  }
});

$('btn-detail-qr').addEventListener('click', async () => {
  if (!selectedId) return;
  loaderStart();
  try {
    const dataUri = await invoke('generate_entry_qr', { id: selectedId });
    $('qr-overlay-title').textContent = $('detail-title').textContent;
    $('qr-overlay-img').src = dataUri;
    $('qr-overlay').style.display = 'flex';
  } catch (err) { toast(String(err), true); }
  finally { loaderDone(); }
});

$('btn-qr-close').addEventListener('click', () => {
  $('qr-overlay').style.display = 'none';
  $('qr-overlay-img').src = '';
});

$('btn-qr-save').addEventListener('click', async () => {
  const img = $('qr-overlay-img');
  if (!img.src) return;
  const filename = `${$('qr-overlay-title').textContent || 'tofa-qr'}.png`;
  try {
    await withPopoverPinned(() => invoke('save_qr_png', { dataUri: img.src, filename }));
  } catch (err) { toast(String(err), true); }
});

// ── Export QR (multi-select) ────────────────────────────────────────────────

$('btn-export-qr').addEventListener('click', () => {
  const list = $('export-qr-list');
  list.innerHTML = entries.map(e => `
    <label style="display:flex;align-items:center;gap:var(--s-3);padding:var(--s-2) var(--s-2);border-radius:var(--r-md);cursor:pointer;">
      <input type="checkbox" data-id="${e.id}" checked style="width:14px;height:14px;accent-color:var(--brand);">
      <span style="font-size:13px;">${e.issuer || e.name}</span>
      ${e.issuer ? `<span style="font-size:11px;color:var(--text-muted);margin-left:auto;">${e.account}</span>` : ''}
    </label>
  `).join('');
  $('export-qr-overlay').style.display = 'flex';
});

$('btn-export-qr-back').addEventListener('click', () => {
  $('export-qr-overlay').style.display = 'none';
});

$('btn-export-qr-generate').addEventListener('click', async () => {
  const ids = [...$('export-qr-list').querySelectorAll('input[type=checkbox]:checked')]
    .map(cb => cb.dataset.id);
  if (ids.length === 0) { toast('Select at least one account', true); return; }
  loaderStart();
  try {
    const dataUri = await invoke('generate_selection_qr', { ids });
    $('export-qr-overlay').style.display = 'none';
    $('qr-overlay-title').textContent = `${ids.length} account${ids.length > 1 ? 's' : ''}`;
    $('qr-overlay-img').src = dataUri;
    $('qr-overlay').style.display = 'flex';
  } catch (err) { toast(String(err), true); }
  finally { loaderDone(); }
});

$('btn-reveal-cancel').addEventListener('click', () => {
  $('reveal-overlay').style.display = 'none';
  $('reveal-passphrase').value = '';
});

$('btn-reveal-confirm').addEventListener('click', async () => {
  const passphrase = $('reveal-passphrase').value;
  const errEl = $('reveal-error');
  errEl.style.display = 'none';
  try {
    const secret = await invoke('get_secret', { id: selectedId, passphrase });
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
    filteredEntries = [];
    selectedId = null;
    fromView = 'view-list';
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
  if (e.key === 'Escape' && selectedId) { $('btn-detail-back').click(); }
});


// Scan step progress (emitted by scan_screen command)
listen('scan-step', ({ payload }) => { toast(String(payload)); });

// ── Boot ───────────────────────────────────────────────────────────────────
bindAddListeners();
init();
