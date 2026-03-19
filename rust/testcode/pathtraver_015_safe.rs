//! Path Traversal True Negative — CWE-22
//! Regex allowlist restricts filename to alphanumeric, hyphen, dot, underscore.

// vuln-code-snippet start testcodePathtraver015Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let base = "/var/data";

    // SAFE: Only allow safe filename characters via char filter
    let is_safe = filename.chars().all(|c| // vuln-code-snippet safe-line testcodePathtraver015Safe
        c.is_alphanumeric() || c == '-' || c == '.' || c == '_'
    );
    if !is_safe || filename.is_empty() {
        return super::shared::BenchmarkResponse::bad_request("Invalid filename characters");
    }

    let full = format!("{}/{}", base, filename);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver015Safe
