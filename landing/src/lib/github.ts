const REPO = 'stratif-io/tofa';
const REPO_URL = `https://github.com/${REPO}`;

let cached: number | null | undefined;

export async function getStars(): Promise<number | null> {
  if (cached !== undefined) return cached;
  try {
    const headers: Record<string, string> = {
      Accept: 'application/vnd.github+json',
      'User-Agent': 'tofa-landing-build',
    };
    if (process.env.GITHUB_TOKEN) {
      headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
    }
    const res = await fetch(`https://api.github.com/repos/${REPO}`, {
      headers,
      signal: AbortSignal.timeout(5000),
    });
    if (!res.ok) {
      cached = null;
      return null;
    }
    const data = (await res.json()) as { stargazers_count?: number };
    cached = typeof data.stargazers_count === 'number' ? data.stargazers_count : null;
    return cached;
  } catch {
    cached = null;
    return null;
  }
}

// Hide the count until the project has a respectable number of stars —
// a low count reads as "not many people use this" and hurts more than it helps.
// Once we cross this threshold the count auto-appears.
export const STARS_DISPLAY_THRESHOLD = 50;

export function formatStars(n: number | null): string | null {
  if (n === null) return null;
  if (n < STARS_DISPLAY_THRESHOLD) return null;
  if (n < 1000) return String(n);
  if (n < 10000) return `${(n / 1000).toFixed(1).replace(/\.0$/, '')}k`;
  return `${Math.round(n / 1000)}k`;
}

export const githubRepoUrl = REPO_URL;

let cachedDmgUrl: string | null | undefined;

export async function getLatestDmgUrl(): Promise<string | null> {
  if (cachedDmgUrl !== undefined) return cachedDmgUrl;
  try {
    const headers: Record<string, string> = {
      Accept: 'application/vnd.github+json',
      'User-Agent': 'tofa-landing-build',
    };
    if (process.env.GITHUB_TOKEN) {
      headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
    }
    const res = await fetch(`https://api.github.com/repos/${REPO}/releases`, {
      headers,
      signal: AbortSignal.timeout(5000),
    });
    if (!res.ok) { cachedDmgUrl = null; return null; }
    const releases = (await res.json()) as Array<{ tag_name: string; assets: Array<{ name: string; browser_download_url: string }> }>;
    const macRelease = releases.find((r) => r.tag_name.startsWith('tofa-macos-'));
    const dmg = macRelease?.assets.find((a) => a.name.endsWith('.dmg'));
    cachedDmgUrl = dmg?.browser_download_url ?? null;
    return cachedDmgUrl;
  } catch {
    cachedDmgUrl = null;
    return null;
  }
}
