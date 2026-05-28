# Kotoba (言葉)

[![SafeSkill 91/100](https://img.shields.io/badge/SafeSkill-91%2F100_Verified%20Safe-brightgreen)](https://safeskill.dev/scan/pleme-io-kotoba)
MCP server framework for pleme-io applications. Eliminates boilerplate when building MCP servers with rmcp.

## What It Provides

| Module | Purpose |
|--------|---------|
| `prelude` | Re-exports all rmcp types needed in every MCP server |
| `response` | `json_ok`, `json_err`, `json_result` — consistent JSON formatting |
| `status` | `StatusInfo`, `UptimeTracker` — standardized health tool |
| `error` | `McpError` — common error type with JSON serialization |
| `server_info()` | Create `ServerInfo` with tools enabled |
| `run()` | Standard stdio server entry point |

## Usage

```toml
[dependencies]
kotoba = { git = "https://github.com/pleme-io/kotoba" }
```

```rust
use kotoba::prelude::*;

#[derive(Clone)]
struct MyServer { tool_router: ToolRouter<Self> }

#[tool_router]
impl MyServer {
    #[tool(description = "do something")]
    async fn my_tool(&self, Parameters(input): Parameters<MyInput>) -> String {
        json_ok(&serde_json::json!({"result": "done"}))
    }
}

#[tool_handler]
impl ServerHandler for MyServer {
    fn get_info(&self) -> ServerInfo {
        kotoba::server_info("my-app", "My app does things")
    }
}

#[tokio::main]
async fn main() {
    kotoba::run(MyServer::new()).await.unwrap();
}
```

## Before/After

**Before (every project):**
```rust
use rmcp::{ServerHandler, ServiceExt, handler::server::..., model::..., ...};
fn json_err(e: impl Display) -> String { format!(r#"{{"error":"{}"}}"#, ...) }
pub async fn run() -> Result<...> {
    let server = MyServer::new().serve(stdio()).await?;
    server.waiting().await?;
    Ok(())
}
```

**After (with kotoba):**
```rust
use kotoba::prelude::*;
// json_ok, json_err, json_result, all rmcp types — already imported
```

## Build

```bash
cargo build
cargo test --lib
```
