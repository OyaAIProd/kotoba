/// Format a serializable value as a pretty-printed JSON string.
///
/// Used for successful tool responses.
pub fn json_ok<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|e| json_err(e))
}

/// Format an error as a JSON object: `{"error": "message"}`.
///
/// Escapes quotes in the error message to produce valid JSON.
pub fn json_err(e: impl std::fmt::Display) -> String {
    let msg = e.to_string().replace('"', "'");
    format!(r#"{{"error":"{msg}"}}"#)
}

/// Convert a `Result<T, E>` into a JSON tool response.
///
/// `Ok(value)` → pretty JSON, `Err(e)` → `{"error": "..."}`.
pub fn json_result<T: serde::Serialize, E: std::fmt::Display>(result: Result<T, E>) -> String {
    match result {
        Ok(val) => json_ok(&val),
        Err(e) => json_err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_formats_json() {
        let val = serde_json::json!({"status": "done"});
        let out = json_ok(&val);
        assert!(out.contains("\"status\""));
        assert!(out.contains("\"done\""));
    }

    #[test]
    fn err_escapes_quotes() {
        let out = json_err("file \"foo\" not found");
        assert!(out.contains("error"));
        assert!(!out.contains("\"foo\"")); // quotes should be escaped
    }

    #[test]
    fn result_ok() {
        let r: Result<_, &str> = Ok(serde_json::json!({"x": 1}));
        let out = json_result(r);
        assert!(out.contains("\"x\""));
    }

    #[test]
    fn result_err() {
        let r: Result<serde_json::Value, _> = Err("boom");
        let out = json_result(r);
        assert!(out.contains("boom"));
    }
}
