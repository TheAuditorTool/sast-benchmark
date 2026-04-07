//! CWE-601: HashMap stores user URL under one key; safe constant URL always read and used.

use std::collections::HashMap;

// vuln-code-snippet start testcodeRedirect050
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut urls = HashMap::new();
    urls.insert("user_url", req.param("url"));
    urls.insert("safe_url", "/dashboard".to_string());
    let dest = urls.get("safe_url").unwrap();
    let location = format!("Location: {}", dest); // vuln-code-snippet target-line testcodeRedirect050
    super::shared::BenchmarkResponse::ok(&location)
}
// vuln-code-snippet end testcodeRedirect050
