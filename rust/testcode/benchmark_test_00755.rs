pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    let secret = req.param("secret");
    let combined = format!("{}:{}", user, secret);
    let token = md5_hash(combined.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("token={}", token))
}

fn md5_hash(_data: &[u8]) -> String {
    "d41d8cd98f00b204e9800998ecf8427e".to_string()
}
