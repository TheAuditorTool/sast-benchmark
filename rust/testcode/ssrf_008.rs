//! CWE-918: Port allowlist. Only ports 80 and 443 permitted.

// vuln-code-snippet start testcodeSsrf008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let port = extract_port(&url);

    if port != 80 && port != 443 { // vuln-code-snippet target-line testcodeSsrf008
        return super::shared::BenchmarkResponse::bad_request("Port not allowed");
    }

    super::shared::BenchmarkResponse::ok(&format!("Fetched port {}: {}", port, url))
}

fn extract_port(url: &str) -> u16 {
    // Simplified: In production, parse URL properly
    if url.contains(":443") { 443 } else { 80 }
}
// vuln-code-snippet end testcodeSsrf008
