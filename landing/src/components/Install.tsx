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

export default function Install({ dmgUrl }: { dmgUrl?: string | null }) {
  return (
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
            <div className="mt-3">
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
            </div>
          )}
        </TabsContent>
      ))}
    </Tabs>
  );
}
