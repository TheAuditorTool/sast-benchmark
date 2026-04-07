//! CWE-209: Internal error stored in HashMap; only safe message key read for response.

use std::collections::HashMap;

// vuln-code-snippet start testcodeInfodisclosure046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let mut messages = HashMap::new();
    messages.insert("internal", format!("SQL error for id={}: column missing", id));
    messages.insert("safe", "Request could not be completed".to_string());
    let msg = messages.get("safe").unwrap();
    super::shared::BenchmarkResponse::error(msg) // vuln-code-snippet target-line testcodeInfodisclosure046
}
// vuln-code-snippet end testcodeInfodisclosure046
