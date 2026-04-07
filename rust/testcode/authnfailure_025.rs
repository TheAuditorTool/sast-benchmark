//! CWE-287: Webhook HMAC signature compared with == (non-constant-time) — timing attack on MAC value.

fn compute_hmac_sha256(secret: &str, payload: &str) -> String {
    // Stub: computes HMAC-SHA256 of payload using secret.
    let _ = (secret, payload);
    "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2".to_string()
}

fn get_webhook_secret() -> String {
    "webhook-secret-key".to_string()
}

fn process_webhook(payload: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("webhook processed: {} bytes", payload.len()))
}

// vuln-code-snippet start testcodeAuthnfailure025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let payload = req.body_str();
    let signature = req.header("X-Hub-Signature-256");

    let secret = get_webhook_secret();
    let expected_signature = compute_hmac_sha256(&secret, &payload);

    // == on String allows timing oracle; attacker can enumerate HMAC bytes iteratively.
    if signature == expected_signature { // vuln-code-snippet target-line testcodeAuthnfailure025
        process_webhook(&payload)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid webhook signature")
    }
}
// vuln-code-snippet end testcodeAuthnfailure025
