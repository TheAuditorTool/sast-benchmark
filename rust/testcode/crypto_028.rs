//! CWE-327: MD5 hash computed via format! intermediary and used as security token.

// vuln-code-snippet start testcodeCrypto028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    let secret = req.param("secret");
    let combined = format!("{}:{}", user, secret);
    let token = md5_hash(combined.as_bytes()); // vuln-code-snippet target-line testcodeCrypto028
    super::shared::BenchmarkResponse::ok(&format!("token={}", token))
}

fn md5_hash(_data: &[u8]) -> String {
    "d41d8cd98f00b204e9800998ecf8427e".to_string()
}
// vuln-code-snippet end testcodeCrypto028
