//! CWE-327: Weak MD5 hash stored under one HashMap key; strong Argon2 hash read from different key.

use std::collections::HashMap;

// vuln-code-snippet start testcodeCrypto041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let mut hashes = HashMap::new();
    hashes.insert("weak", format!("md5:{}", password));
    hashes.insert("strong", format!("argon2id_hash_placeholder"));
    let stored = hashes.get("strong").unwrap();
    super::shared::BenchmarkResponse::ok(&format!("stored={}", stored)) // vuln-code-snippet target-line testcodeCrypto041
}
// vuln-code-snippet end testcodeCrypto041
