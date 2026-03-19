//! Path Traversal True Negative — CWE-22
//! Extension allowlist restricts file types (.txt, .csv, .json).

use std::path::Path;

// vuln-code-snippet start testcodePathtraver007Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let base = "/var/data";

    // SAFE: Extract basename to prevent traversal via directory components
    let basename = Path::new(&filename).file_name() // vuln-code-snippet safe-line testcodePathtraver007Safe
        .and_then(|n| n.to_str())
        .unwrap_or("");

    // Only allow specific file extensions on the basename
    let ext = Path::new(basename).extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    if !matches!(ext, "txt" | "csv" | "json") {
        return super::shared::BenchmarkResponse::bad_request("File type not allowed");
    }

    let full = format!("{}/{}", base, basename);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver007Safe
