//! CWE-327: Hash algorithm variable unconditionally overwritten to strong algorithm before use.

// vuln-code-snippet start testcodeCrypto039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let mut algorithm = req.param("algo");
    algorithm = "argon2id".to_string();
    let hash = hash_with_algo(password.as_bytes(), &algorithm); // vuln-code-snippet target-line testcodeCrypto039
    super::shared::BenchmarkResponse::ok(&hash)
}

fn hash_with_algo(_data: &[u8], algo: &str) -> String { format!("{}_hash", algo) }
// vuln-code-snippet end testcodeCrypto039
