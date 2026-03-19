//! Information Disclosure True Negative — CWE-532
//! Logs request details but redacts sensitive headers before returning response.

// vuln-code-snippet start testcodeInfodisclosure007Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let sensitive = ["authorization", "cookie", "x-api-key"];
    let mut log_lines = String::new();

    for (key, value) in &req.headers {
        let lower = key.to_lowercase();
        // SAFE: Sensitive headers are redacted before logging/response
        let display_val = if sensitive.contains(&lower.as_str()) { // vuln-code-snippet safe-line testcodeInfodisclosure007Safe
            "[REDACTED]".to_string()
        } else {
            value.clone()
        };
        log_lines.push_str(&format!("{}: {}\n", key, display_val));
    }

    super::shared::BenchmarkResponse::ok(&format!("Request logged:\n{}", log_lines))
}
// vuln-code-snippet end testcodeInfodisclosure007Safe
