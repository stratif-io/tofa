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

export default function Install() {
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
        </TabsContent>
      ))}
    </Tabs>
  );
}
