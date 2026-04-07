pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("url");
    if target.contains("://") || target.starts_with("//") {
        return super::shared::BenchmarkResponse::bad_request("Absolute URLs not allowed");
    }
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", target))
}
