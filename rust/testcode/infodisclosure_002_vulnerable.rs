//! Information Disclosure True Positive — CWE-200
//! Environment variables dumped in response, leaking DATABASE_URL, API_KEY, etc.

// vuln-code-snippet start testcodeInfodisclosure002Vulnerable
pub fn handle(_req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut output = String::from("Environment Variables:\n");

    // VULNERABLE: All env vars returned to client
    for (key, value) in std::env::vars() { // vuln-code-snippet vuln-line testcodeInfodisclosure002Vulnerable
        output.push_str(&format!("{}={}\n", key, value));
    }

    super::shared::BenchmarkResponse::ok(&output)
}
// vuln-code-snippet end testcodeInfodisclosure002Vulnerable
