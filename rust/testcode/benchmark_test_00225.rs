pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let host = req.header("X-Forwarded-Host");
    let path = req.param("path");
    let location = format!("Location: https://{}{}", host, path);
    super::shared::BenchmarkResponse::ok(&location)
}
