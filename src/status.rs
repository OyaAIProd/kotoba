use serde::Serialize;
use std::time::Instant;

/// Standardized status/health information for MCP servers.
///
/// Every pleme-io MCP server should expose a `status` tool that returns
/// this struct (or a superset of it).
#[derive(Debug, Clone, Serialize)]
pub struct StatusInfo {
    pub name: String,
    pub version: String,
    pub uptime_secs: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Tracks server start time for uptime calculation.
pub struct UptimeTracker {
    started: Instant,
}

impl UptimeTracker {
    pub fn new() -> Self {
        Self { started: Instant::now() }
    }

    pub fn uptime_secs(&self) -> u64 {
        self.started.elapsed().as_secs()
    }

    /// Build a `StatusInfo` with the current uptime.
    pub fn status(&self, name: &str, version: &str) -> StatusInfo {
        StatusInfo {
            name: name.to_string(),
            version: version.to_string(),
            uptime_secs: self.uptime_secs(),
            status: Some("healthy".to_string()),
            details: None,
        }
    }
}

impl Default for UptimeTracker {
    fn default() -> Self {
        Self::new()
    }
}
