//! CWE-287: Constant-time comparison for webhook HMAC signature — prevents iterative MAC enumeration.

fn compute_hmac_sha256(secret: &str, payload: &str) -> String {
    let _ = (secret, payload);
    "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2".to_string()
}

fn get_webhook_secret() -> String {
    "webhook-secret-key".to_string()
}

fn constant_time_verify(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn process_webhook(payload: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("webhook processed: {} bytes", payload.len()))
}

// vuln-code-snippet start testcodeAuthnfailure048
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let payload = req.body_str();
    let signature = req.header("X-Hub-Signature-256");

    let secret = get_webhook_secret();
    let expected = compute_hmac_sha256(&secret, &payload);

    if constant_time_verify(signature.as_bytes(), expected.as_bytes()) { // vuln-code-snippet target-line testcodeAuthnfailure048
        process_webhook(&payload)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid signature")
    }
}
// vuln-code-snippet end testcodeAuthnfailure048
