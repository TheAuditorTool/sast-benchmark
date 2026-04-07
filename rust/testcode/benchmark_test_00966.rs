pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let hash = md5_hash(password.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("hash={}", hash))
}

fn md5_hash(_data: &[u8]) -> String {
    "d41d8cd98f00b204e9800998ecf8427e".to_string()
}
