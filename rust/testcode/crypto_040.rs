//! CWE-327: Hash helper discards user-supplied algorithm name and always uses Argon2.

// vuln-code-snippet start testcodeCrypto040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let _algo = req.param("algorithm");
    let hash = safe_hash(password.as_bytes(), &_algo); // vuln-code-snippet target-line testcodeCrypto040
    super::shared::BenchmarkResponse::ok(&hash)
}

fn safe_hash(pwd: &[u8], _user_algo: &str) -> String {
    format!("argon2id_hash_len_{}", pwd.len())
}
// vuln-code-snippet end testcodeCrypto040
