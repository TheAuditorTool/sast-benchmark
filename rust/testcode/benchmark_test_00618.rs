pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = b"hardcoded-aes-256-key-32bytes!!!";
    let result = format!("Encrypting '{}' with key of {} bytes", plaintext, key.len());
    super::shared::BenchmarkResponse::ok(&result)
}
