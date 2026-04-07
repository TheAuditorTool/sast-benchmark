pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let csrf = req.param("csrf");
    let cookie = format!("csrf_token={}; SameSite=Strict", csrf);
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
