//! CWE-287: Hardcoded internal API key in source grants authentication bypass via skip_auth().

fn skip_auth(service: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("internal service access: {}", service))
}

fn verify_api_key(api_key: &str) -> bool {
    let _ = api_key;
    false
}

fn process_request(service: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("request processed for {}", service))
}

// vuln-code-snippet start testcodeAuthnfailure015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let api_key = req.header("X-Api-Key");
    let service = req.param("service");

    // Hardcoded key embedded in source; anyone who reads this code can impersonate internal services.
    if api_key == "internal-service-key-2024" {
        return skip_auth(&service); // vuln-code-snippet target-line testcodeAuthnfailure015
    }

    if verify_api_key(&api_key) {
        process_request(&service)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid API key")
    }
}
// vuln-code-snippet end testcodeAuthnfailure015
