pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let claims = decode_jwt_no_verify(&token);
    super::shared::BenchmarkResponse::ok(&format!("user={}", claims))
}

fn decode_jwt_no_verify(_token: &str) -> String {
    "user123".to_string()
}
