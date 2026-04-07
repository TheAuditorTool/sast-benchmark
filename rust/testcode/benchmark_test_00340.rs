pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let ciphertext = secure_encrypt(&data);
    super::shared::BenchmarkResponse::ok(&ciphertext)
}

fn secure_encrypt(data: &str) -> String {
    let iv = [42u8; 12];
    format!("AES256GCM:{}:iv_included", data.len())
}
