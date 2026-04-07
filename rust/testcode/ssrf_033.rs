//! CWE-918: Only relative paths accepted — absolute URLs and scheme indicators rejected.

// vuln-code-snippet start testcodeSsrf033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");

    if !path.starts_with('/') || path.contains("://") {
        return super::shared::BenchmarkResponse::forbidden("Only relative paths are permitted");
    }

    super::shared::BenchmarkResponse::ok(&format!("Processed internal path: {}", path)) // vuln-code-snippet target-line testcodeSsrf033
}
// vuln-code-snippet end testcodeSsrf033
