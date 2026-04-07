pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.body_str();
    let key = b"0123456789abcdef";
    let mut result = data.as_bytes().to_vec();
    for (i, byte) in result.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }
    super::shared::BenchmarkResponse::ok(&format!("Encrypted {} bytes with RC4-like XOR", result.len()))
}
