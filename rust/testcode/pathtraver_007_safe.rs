//! Path Traversal True Negative — CWE-22
//! Extension allowlist restricts file types (.txt, .csv, .json).

use std::path::Path;

// vuln-code-snippet start testcodePathtraver007Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let base = "/var/data";

    // SAFE: Only allow specific file extensions
    let ext = Path::new(&filename).extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    if !matches!(ext, "txt" | "csv" | "json") { // vuln-code-snippet safe-line testcodePathtraver007Safe
        return super::shared::BenchmarkResponse::bad_request("File type not allowed");
    }

    let full = format!("{}/{}", base, filename);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver007Safe
