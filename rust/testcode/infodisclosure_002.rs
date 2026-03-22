//! CWE-200: Environment variables dumped in response.

// vuln-code-snippet start testcodeInfodisclosure002
pub fn handle(_req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut output = String::from("Environment Variables:\n");

    for (key, value) in std::env::vars() { // vuln-code-snippet target-line testcodeInfodisclosure002
        output.push_str(&format!("{}={}\n", key, value));
    }

    super::shared::BenchmarkResponse::ok(&output)
}
// vuln-code-snippet end testcodeInfodisclosure002
