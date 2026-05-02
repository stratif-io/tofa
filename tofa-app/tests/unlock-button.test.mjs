// TDD: unlock button state is reset when session is locked mid-flight
//
// Run: node --experimental-vm-modules tests/unlock-button.test.mjs

import assert from 'node:assert/strict';

// ── Minimal DOM stub ─────────────────────────────────────────────────────────

function makeEl(tag = 'div') {
  return {
    _tag: tag,
    _text: '',
    disabled: false,
    classList: (() => {
      const set = new Set();
      return {
        add: (...c) => c.forEach(x => set.add(x)),
        remove: (...c) => c.forEach(x => set.delete(x)),
        contains: c => set.has(c),
      };
    })(),
    get textContent() { return this._text; },
    set textContent(v) { this._text = v; },
    focus() {},
    click() {},
    addEventListener() {},
  };
}

// ── Unit under test (extracted logic, no Tauri dependency) ───────────────────

function resetUnlockButton(btn) {
  btn.disabled = false;
  if (btn.textContent === '…') {
    btn.textContent = btn._savedLabel ?? 'Unlock';
  }
}

function simulateLock(btn) {
  // doLock() must call resetUnlockButton before switching view
  resetUnlockButton(btn);
}

// ── Tests ─────────────────────────────────────────────────────────────────────

let passed = 0;
let failed = 0;

function test(name, fn) {
  try {
    fn();
    console.log(`  ✓ ${name}`);
    passed++;
  } catch (err) {
    console.log(`  ✗ ${name}`);
    console.log(`    ${err.message}`);
    failed++;
  }
}

console.log('\nUnlock button state on lock\n');

test('button is enabled and shows label after lock when idle', () => {
  const btn = makeEl('button');
  btn.textContent = 'Unlock';
  btn.disabled = false;

  simulateLock(btn);

  assert.equal(btn.disabled, false);
  assert.equal(btn.textContent, 'Unlock');
});

test('button is reset when locked mid-flight (disabled + spinner)', () => {
  const btn = makeEl('button');
  btn._savedLabel = 'Unlock';
  btn.textContent = '…';
  btn.disabled = true;

  simulateLock(btn);

  assert.equal(btn.disabled, false, 'button must be re-enabled');
  assert.notEqual(btn.textContent, '…', 'button must not show spinner');
  assert.equal(btn.textContent, 'Unlock', 'button must restore original label');
});

test('button in create-vault mode is also reset', () => {
  const btn = makeEl('button');
  btn._savedLabel = 'Create vault';
  btn.textContent = '…';
  btn.disabled = true;

  simulateLock(btn);

  assert.equal(btn.disabled, false);
  assert.equal(btn.textContent, 'Create vault');
});

console.log(`\n${passed} passed, ${failed} failed\n`);
if (failed > 0) process.exit(1);
