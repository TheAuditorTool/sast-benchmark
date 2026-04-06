//! CWE-798: TLS certificate loaded from file path provided as CLI argument.

// vuln-code-snippet start testcodeHardcodedcreds018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    let cert_path = std::env::var("TLS_CERT_PATH").unwrap_or_default();
    let cert_contents = std::fs::read_to_string(&cert_path); // vuln-code-snippet target-line testcodeHardcodedcreds018
    let result = match cert_contents {
        Ok(_) => format!("Action {} with cert from {}", action, cert_path),
        Err(e) => format!("Failed to load cert: {}", e),
    };
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds018
