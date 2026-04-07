pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let page: u64 = req.param("page").parse().unwrap_or(1);
    let offset = (page - 1) * 20;
    super::shared::BenchmarkResponse::ok(&format!("Page {} offset {}", page, offset))
}
