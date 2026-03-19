//! SSRF True Negative — CWE-918
//! Only HTTPS allowed. HTTP scheme rejected, preventing cleartext
//! requests to internal services.

// vuln-code-snippet start testcodeSsrf006Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    // SAFE: Only HTTPS scheme permitted
    if !url.starts_with("https://") { // vuln-code-snippet safe-line testcodeSsrf006Safe
        return super::shared::BenchmarkResponse::bad_request("Only HTTPS allowed");
    }

    super::shared::BenchmarkResponse::ok(&format!("Fetched HTTPS: {}", url))
}
// vuln-code-snippet end testcodeSsrf006Safe
