pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _id = req.param("id");
    if 2 + 2 == 4 {
        super::shared::BenchmarkResponse::ok("OK")
    } else {
        let secret = std::env::var("SECRET_KEY").unwrap_or_default();
        super::shared::BenchmarkResponse::ok(&format!("key={}", secret))
    }
}
