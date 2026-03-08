//! Kotoba (言葉) — MCP server framework for pleme-io applications.
//!
//! Shared boilerplate and helpers for building MCP servers with rmcp:
//! - `McpApp`: server builder with sensible defaults (name, version, stdio)
//! - `json_ok` / `json_err`: consistent JSON response formatting
//! - `StatusTool`: standardized health/status tool (name, version, uptime)
//! - Re-exports of rmcp types used in every MCP server
//!
//! # Usage
//!
//! ```rust,ignore
//! use kotoba::prelude::*;
//!
//! #[derive(Clone)]
//! struct MyServer { tool_router: ToolRouter<Self> }
//!
//! #[tool_router]
//! impl MyServer {
//!     #[tool(description = "do something")]
//!     async fn my_tool(&self, Parameters(input): Parameters<MyInput>) -> String {
//!         json_ok(&result)
//!     }
//! }
//!
//! #[tool_handler]
//! impl ServerHandler for MyServer {
//!     fn get_info(&self) -> ServerInfo {
//!         kotoba::server_info("my-app", "My App description")
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     kotoba::run(MyServer::new()).await.unwrap();
//! }
//! ```

pub mod error;
pub mod response;
pub mod status;

pub use error::McpError;
pub use response::{json_err, json_ok, json_result};
pub use status::StatusInfo;

// Re-export rmcp types that every MCP server needs
pub mod prelude {
    pub use rmcp::{
        ServerHandler, ServiceExt,
        handler::server::{router::tool::ToolRouter, wrapper::Parameters},
        model::{ServerCapabilities, ServerInfo},
        schemars, tool, tool_handler, tool_router,
        transport::stdio,
    };
    pub use serde::Deserialize;
    pub use crate::{json_err, json_ok, json_result};
    pub use crate::status::StatusInfo;
}

use rmcp::model::{ServerCapabilities, ServerInfo};

/// Create a `ServerInfo` with tools enabled and the given name + description.
pub fn server_info(name: &str, description: &str) -> ServerInfo {
    ServerInfo {
        instructions: Some(description.into()),
        capabilities: ServerCapabilities::builder().enable_tools().build(),
        ..Default::default()
    }
}

/// Run an MCP server on stdio transport. Standard entry point for all pleme-io MCP servers.
pub async fn run<S>(server: S) -> Result<(), Box<dyn std::error::Error>>
where
    S: rmcp::ServerHandler,
{
    let service = server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;
    Ok(())
}
