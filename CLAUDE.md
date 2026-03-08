# Kotoba (言葉) — MCP Server Framework

## Build & Test

```bash
cargo build
cargo test --lib
```

## Architecture

Shared MCP server library that eliminates boilerplate across all pleme-io rmcp servers.

### Modules

| Module | Purpose |
|--------|---------|
| `lib.rs` | `server_info()`, `run()` — server creation and startup helpers |
| `response.rs` | `json_ok`, `json_err`, `json_result` — consistent JSON formatting |
| `status.rs` | `StatusInfo`, `UptimeTracker` — standardized health/status tool data |
| `error.rs` | `McpError` — common error enum with JSON serialization |
| `prelude` | Re-exports: rmcp types + serde + response helpers |

### What Gets Shared

Every pleme-io MCP server previously reimplemented:
1. rmcp import block (7+ items from various modules)
2. `json_err()` helper for error formatting
3. `ServerHandler` boilerplate (`get_info`, capabilities)
4. `pub async fn run()` startup (serve + waiting)
5. Status/health tool pattern

Kotoba consolidates all of these.

### Consumers

All pleme-io Rust projects with MCP servers:
- Existing: zoekt-mcp, codesearch, ayatsuri, hikyaku, umbra, mathscape
- New: mado, hibiki, kagi, kekkai, fumi, nami

### rmcp Version

Pinned to rmcp 0.16. All consumers should align to this version.

## Design Decisions

- **Re-export, don't wrap**: kotoba re-exports rmcp types, doesn't abstract them
- **Prelude pattern**: `use kotoba::prelude::*` gives you everything
- **No transport opinion**: defaults to stdio but consumers can use any rmcp transport
- **Serde-based responses**: `json_ok`/`json_err` return `String` (rmcp tool return type)
