/// Common error type for MCP server operations.
#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("tool not found: {0}")]
    ToolNotFound(String),

    #[error("invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("operation failed: {0}")]
    Operation(String),

    #[error("not connected")]
    NotConnected,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl McpError {
    /// Convert to a JSON error response string.
    pub fn to_json(&self) -> String {
        crate::json_err(self)
    }
}
