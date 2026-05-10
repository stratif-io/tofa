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
