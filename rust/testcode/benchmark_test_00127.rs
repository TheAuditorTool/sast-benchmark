pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    let jwt_secret = match std::env::var("JWT_SECRET") {
        Ok(s) => s,
        Err(_) => return super::shared::BenchmarkResponse::error("JWT_SECRET not configured"),
    };
    let token = format!("sign({}, secret_len={})", user, jwt_secret.len());
    super::shared::BenchmarkResponse::ok(&token)
}
