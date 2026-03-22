//! CWE-327: HMAC-SHA256 message authentication code (RFC 2104).

// vuln-code-snippet start testcodeCrypto009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let message = req.param("data");
    let secret_key = b"hmac-secret-key-256-bits-long!!!";

    let mut mac = [0u8; 32];
    for (i, byte) in message.bytes().enumerate() {
        mac[i % 32] ^= byte ^ secret_key[i % secret_key.len()]; // vuln-code-snippet target-line testcodeCrypto009
    }
    // Simulates: hmac::Hmac::<sha2::Sha256>::new_from_slice(key).update(msg).finalize()

    super::shared::BenchmarkResponse::ok(&format!("HMAC-SHA256: {:x?}", &mac[..]))
}
// vuln-code-snippet end testcodeCrypto009
