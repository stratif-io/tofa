import { useState } from 'react';
import { Tabs, TabsContent, TabsList, TabsTrigger } from './ui/tabs';
import CopyButton from './CopyButton';

interface TabDef { id: string; label: string; commands: string[] }

const TABS: TabDef[] = [
  {
    id: 'shell',
    label: 'Shell',
    commands: ['curl -fsSL https://tofa.stratif.io/install.sh | sh'],
  },
  {
    id: 'macos',
    label: 'macOS',
    commands: ['brew tap stratif-io/tofa', 'brew install tofa', '# menu bar app:', 'brew install --cask tofa'],
  },
  {
    id: 'linux',
    label: 'Linux',
    commands: ['# shell installer (no Rust required):', 'curl -fsSL https://tofa.stratif.io/install.sh | sh', '# or with Homebrew:', 'brew tap stratif-io/tofa', 'brew install tofa'],
  },
  {
    id: 'cargo',
    label: 'Cargo',
    commands: ['cargo install tofa'],
  },
  {
    id: 'source',
    label: 'From source',
    commands: ['git clone https://github.com/stratif-io/tofa', 'cd tofa', 'cargo build --release', './target/release/tofa --help'],
  },
];

function NotarizedModal({ onClose }: { onClose: () => void }) {
  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
      onClick={onClose}
    >
      <div
        className="relative w-full max-w-md rounded-tofa-lg bg-surface border border-border shadow-xl p-6"
        onClick={(e) => e.stopPropagation()}
      >
        <button
          onClick={onClose}
          aria-label="Close"
          className="absolute top-4 right-4 text-text-subtle hover:text-text transition-colors"
        >
          <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
            <path d="M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.749.749 0 0 1 1.06 1.06L9.06 8l3.22 3.22a.749.749 0 0 1-1.06 1.06L8 9.06l-3.22 3.22a.749.749 0 0 1-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z"/>
          </svg>
        </button>

        <div className="flex items-center gap-3 mb-4">
          <span className="text-2xl" aria-hidden="true">🔒</span>
          <h2 className="font-display font-semibold text-lg text-text">First launch on macOS</h2>
        </div>

        <p className="text-sm text-text-muted mb-5">
          TOFA isn't notarized yet, so macOS Gatekeeper will block it on first launch. Here's how to allow it — takes 10 seconds.
        </p>

        <ol className="space-y-4 text-sm text-text">
          <li className="flex gap-3">
            <span className="flex-shrink-0 w-6 h-6 rounded-full bg-brand/15 text-brand font-semibold flex items-center justify-center text-xs">1</span>
            <span>Open <strong>System Settings</strong> → <strong>Privacy &amp; Security</strong></span>
          </li>
          <li className="flex gap-3">
            <span className="flex-shrink-0 w-6 h-6 rounded-full bg-brand/15 text-brand font-semibold flex items-center justify-center text-xs">2</span>
            <span>Scroll down to the <strong>Security</strong> section — you'll see a message about TOFA being blocked</span>
          </li>
          <li className="flex gap-3">
            <span className="flex-shrink-0 w-6 h-6 rounded-full bg-brand/15 text-brand font-semibold flex items-center justify-center text-xs">3</span>
            <span>Click <strong>Open Anyway</strong>, then confirm in the dialog</span>
          </li>
        </ol>

        <div className="mt-5 pt-4 border-t border-border">
          <p className="text-xs text-text-subtle mb-2">Or use the terminal one-liner:</p>
          <div className="relative">
            <pre className="rounded-tofa-md bg-bg-sunken border border-border px-3 py-2 font-mono text-xs text-text overflow-x-auto">
              <code>xattr -dr com.apple.quarantine /Applications/tofa.app</code>
            </pre>
            <div className="mt-1.5 flex justify-end">
              <CopyButton
                value="xattr -dr com.apple.quarantine /Applications/tofa.app"
                umamiEvent="install-copy-quarantine"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default function Install({ dmgUrl }: { dmgUrl?: string | null }) {
  const [modalOpen, setModalOpen] = useState(false);

  return (
    <>
      <Tabs defaultValue="shell">
        <TabsList>
          {TABS.map((t) => (
            <TabsTrigger key={t.id} value={t.id} data-umami-event={`install-tab-${t.id}`}>{t.label}</TabsTrigger>
          ))}
        </TabsList>
        {TABS.map((t) => (
          <TabsContent key={t.id} value={t.id}>
            <pre className="rounded-tofa-md bg-bg-sunken border border-border p-4 font-mono text-sm text-text overflow-x-auto relative">
              <code>{t.commands.join('\n')}</code>
              <div className="absolute top-2 right-2">
                <CopyButton
                  value={t.commands.filter((c) => !c.startsWith('#')).join('\n')}
                  umamiEvent={`install-copy-${t.id}`}
                />
              </div>
            </pre>
            {t.id === 'macos' && dmgUrl && (
              <div className="mt-3 flex items-center gap-3">
                <a
                  href={dmgUrl}
                  data-umami-event="install-download-dmg"
                  className="inline-flex items-center gap-2 text-sm text-text-muted hover:text-text transition-colors"
                >
                  <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
                    <path d="M2.75 14A1.75 1.75 0 0 1 1 12.25v-2.5a.75.75 0 0 1 1.5 0v2.5c0 .138.112.25.25.25h10.5a.25.25 0 0 0 .25-.25v-2.5a.75.75 0 0 1 1.5 0v2.5A1.75 1.75 0 0 1 13.25 14Z"/>
                    <path d="M7.25 7.689V2a.75.75 0 0 1 1.5 0v5.689l1.97-1.97a.749.749 0 1 1 1.06 1.06l-3.25 3.25a.749.749 0 0 1-1.06 0L4.22 6.779a.749.749 0 1 1 1.06-1.06l1.97 1.97Z"/>
                  </svg>
                  Download .dmg directly
                </a>
                <button
                  onClick={() => setModalOpen(true)}
                  data-umami-event="install-dmg-notarized-info"
                  className="inline-flex items-center gap-1.5 text-xs text-amber-500/80 hover:text-amber-500 transition-colors"
                >
                  <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
                    <path d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13ZM6.5 7.75A.75.75 0 0 1 7.25 7h1a.75.75 0 0 1 .75.75v2.75h.25a.75.75 0 0 1 0 1.5h-2a.75.75 0 0 1 0-1.5h.25v-2h-.25a.75.75 0 0 1-.75-.75ZM8 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"/>
                  </svg>
                  Not notarized — first launch fix
                </button>
              </div>
            )}
          </TabsContent>
        ))}
      </Tabs>
      {modalOpen && <NotarizedModal onClose={() => setModalOpen(false)} />}
    </>
  );
}
