//! CWE-200: Only allowlisted non-sensitive env vars returned.

// vuln-code-snippet start testcodeInfodisclosure005
pub fn handle(_req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let allowed = ["PATH", "LANG", "TZ", "HOME"];
    let mut output = String::from("Environment:\n");

    for key in &allowed { // vuln-code-snippet target-line testcodeInfodisclosure005
        if let Ok(val) = std::env::var(key) {
            output.push_str(&format!("{}={}\n", key, val));
        }
    }

    super::shared::BenchmarkResponse::ok(&output)
}
// vuln-code-snippet end testcodeInfodisclosure005
