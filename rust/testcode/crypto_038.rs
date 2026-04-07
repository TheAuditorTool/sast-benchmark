//! CWE-327: Constant-folded condition always selects strong Argon2 hash; weak path is unreachable.

// vuln-code-snippet start testcodeCrypto038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let hash = if 2 * 2 == 4 {
        argon2_hash(password.as_bytes()) // vuln-code-snippet target-line testcodeCrypto038
    } else {
        md5_hash(password.as_bytes())
    };
    super::shared::BenchmarkResponse::ok(&hash)
}

fn argon2_hash(_pwd: &[u8]) -> String { "$argon2id$...".to_string() }
fn md5_hash(_pwd: &[u8]) -> String { "md5hash".to_string() }
// vuln-code-snippet end testcodeCrypto038
