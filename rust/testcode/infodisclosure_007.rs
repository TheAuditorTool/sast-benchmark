//! CWE-532: Logs request details with sensitive header redaction.

// vuln-code-snippet start testcodeInfodisclosure007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let sensitive = ["authorization", "cookie", "x-api-key"];
    let mut log_lines = String::new();

    for (key, value) in &req.headers {
        let lower = key.to_lowercase();
        let display_val = if sensitive.contains(&lower.as_str()) { // vuln-code-snippet target-line testcodeInfodisclosure007
            "[REDACTED]".to_string()
        } else {
            value.clone()
        };
        log_lines.push_str(&format!("{}: {}\n", key, display_val));
    }

    super::shared::BenchmarkResponse::ok(&format!("Request logged:\n{}", log_lines))
}
// vuln-code-snippet end testcodeInfodisclosure007
