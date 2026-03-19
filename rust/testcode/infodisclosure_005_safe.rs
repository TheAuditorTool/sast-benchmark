//! Information Disclosure True Negative — CWE-200
//! Only returns specific non-sensitive env vars via allowlist check.

// vuln-code-snippet start testcodeInfodisclosure005Safe
pub fn handle(_req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let allowed = ["PATH", "LANG", "TZ", "HOME"];
    let mut output = String::from("Environment:\n");

    // SAFE: Only allowlisted non-sensitive env vars returned
    for key in &allowed { // vuln-code-snippet safe-line testcodeInfodisclosure005Safe
        if let Ok(val) = std::env::var(key) {
            output.push_str(&format!("{}={}\n", key, val));
        }
    }

    super::shared::BenchmarkResponse::ok(&output)
}
// vuln-code-snippet end testcodeInfodisclosure005Safe
