import { useState } from 'react';

interface Props { value: string; label?: string; umamiEvent?: string }

export default function CopyButton({ value, label = 'Copy', umamiEvent }: Props) {
  const [copied, setCopied] = useState(false);

  async function copy() {
    try {
      await navigator.clipboard.writeText(value);
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    } catch {
      /* clipboard API can fail in insecure contexts — fail-quiet */
    }
  }

  return (
    <button
      type="button"
      onClick={copy}
      aria-label={`Copy: ${value}`}
      data-umami-event={umamiEvent}
      className="font-mono text-[11px] px-2 py-1 rounded-tofa-sm border border-border text-text-muted hover:text-text hover:border-border-strong transition-colors"
    >
      {copied ? '✓ copied' : label}
    </button>
  );
}
