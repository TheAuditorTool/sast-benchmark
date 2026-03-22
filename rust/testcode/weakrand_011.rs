//! CWE-330: HMAC-based token from secret key.

// vuln-code-snippet start testcodeWeakrand011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    let secret_key: &[u8] = b"server-side-secret-key-from-env";


    let token = hmac_token(secret_key, user.as_bytes()); // vuln-code-snippet target-line testcodeWeakrand011

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn hmac_token(key: &[u8], data: &[u8]) -> String {
    // Simulates: hmac::Hmac::<sha2::Sha256>::new_from_slice(key).update(data).finalize()
    let mut hash = 0u64;
    for (i, &b) in key.iter().chain(data.iter()).enumerate() {
        hash = hash.wrapping_add((b as u64) << ((i % 8) * 8));
    }
    format!("{:016x}", hash)
}
// vuln-code-snippet end testcodeWeakrand011
