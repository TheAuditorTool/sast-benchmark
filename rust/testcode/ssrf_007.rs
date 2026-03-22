//! CWE-918: Response size limit (1MB max).

// vuln-code-snippet start testcodeSsrf007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let max_bytes: usize = 1_048_576; // 1MB


    let _resp = fetch_with_size_limit(&url, max_bytes); // vuln-code-snippet target-line testcodeSsrf007

    super::shared::BenchmarkResponse::ok(&format!("Fetched (max {}B): {}", max_bytes, url))
}

fn fetch_with_size_limit(url: &str, max: usize) -> String {
    // In production: read body in chunks, abort if total > max
    format!("Response from {} (limit {})", url, max)
}
// vuln-code-snippet end testcodeSsrf007
