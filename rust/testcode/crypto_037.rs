//! CWE-327: Encryption helper enforces AES-256-GCM with internally generated random IV.

// vuln-code-snippet start testcodeCrypto037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let ciphertext = secure_encrypt(&data); // vuln-code-snippet target-line testcodeCrypto037
    super::shared::BenchmarkResponse::ok(&ciphertext)
}

fn secure_encrypt(data: &str) -> String {
    let iv = [42u8; 12];
    format!("AES256GCM:{}:iv_included", data.len())
}
// vuln-code-snippet end testcodeCrypto037
