//! Update notifier: queries the GitHub Releases API for a newer
//! `tofa-macos-vX.Y.Z` tag, exposes the result via `UpdaterState` on the
//! AppHandle, and emits an `update-available` event when a newer version is
//! detected. This module never downloads or installs anything — it only
//! tells the rest of the app what the latest published release is.

use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Minimal subset of the GitHub Releases JSON we need.
#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseJson {
    pub tag_name: String,
    pub html_url: String,
    #[serde(default)]
    pub prerelease: bool,
    #[serde(default)]
    pub draft: bool,
}

/// Result of a single update check.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateStatus {
    #[serde(serialize_with = "ser_version")]
    pub current: Version,
    #[serde(serialize_with = "ser_version_opt")]
    pub latest: Option<Version>,
    pub release_url: Option<String>,
    pub checked_at: DateTime<Utc>,
}

impl UpdateStatus {
    pub fn is_update_available(&self) -> bool {
        matches!(&self.latest, Some(v) if v > &self.current)
    }
}

fn ser_version<S: serde::Serializer>(v: &Version, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&v.to_string())
}

fn ser_version_opt<S: serde::Serializer>(v: &Option<Version>, s: S) -> Result<S::Ok, S::Error> {
    match v {
        Some(v) => s.serialize_some(&v.to_string()),
        None => s.serialize_none(),
    }
}

/// All recoverable errors produced by the updater.
#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("network error: {0}")]
    Network(String),
    #[error("GitHub rate limit exceeded")]
    RateLimited,
    #[error("bad response: {0}")]
    BadResponse(String),
}

/// In-memory state stored on the AppHandle.
#[derive(Debug, Default)]
pub struct UpdaterState {
    pub last_status: Option<UpdateStatus>,
    pub in_flight: bool,
}

const TAG_PREFIX: &str = "tofa-macos-v";

/// Pick the highest-versioned non-draft, non-prerelease `tofa-macos-vX.Y.Z`
/// tag from `releases` and compare to `current`.
pub fn pick_latest(releases: &[ReleaseJson], current: &Version) -> UpdateStatus {
    let mut best: Option<(Version, &str)> = None;
    for r in releases {
        if r.prerelease || r.draft {
            continue;
        }
        let Some(rest) = r.tag_name.strip_prefix(TAG_PREFIX) else {
            continue;
        };
        let Ok(v) = Version::parse(rest) else {
            continue;
        };
        match &best {
            Some((cur_best, _)) if cur_best >= &v => {}
            _ => best = Some((v, &r.html_url)),
        }
    }

    let (latest, release_url) = match best {
        Some((v, url)) => (Some(v), Some(url.to_string())),
        None => (None, None),
    };

    UpdateStatus {
        current: current.clone(),
        latest,
        release_url,
        checked_at: Utc::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rel(tag: &str, prerelease: bool, draft: bool) -> ReleaseJson {
        ReleaseJson {
            tag_name: tag.to_string(),
            html_url: format!("https://example.test/{}", tag),
            prerelease,
            draft,
        }
    }

    fn v(s: &str) -> Version {
        Version::parse(s).unwrap()
    }

    #[test]
    fn picks_newer_release() {
        let releases = vec![
            rel("tofa-macos-v0.7.0", false, false),
            rel("tofa-macos-v0.8.0", false, false),
            rel("tofa-macos-v0.7.5", false, false),
        ];
        let status = pick_latest(&releases, &v("0.7.0"));
        assert_eq!(status.current, v("0.7.0"));
        assert_eq!(status.latest, Some(v("0.8.0")));
        assert_eq!(
            status.release_url.as_deref(),
            Some("https://example.test/tofa-macos-v0.8.0")
        );
        assert!(status.is_update_available());
    }
}
