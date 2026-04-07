const JWT_SECRET: &str = "super-secret-jwt-signing-key-2024";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    let token = format!("sign({}, {})", user, JWT_SECRET);
    let result = format!("Issued token: {}", token);
    super::shared::BenchmarkResponse::ok(&result)
}
