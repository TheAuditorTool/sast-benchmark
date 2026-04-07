pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let feed = req.cookie("feed_url");
    super::shared::BenchmarkResponse::ok(&format!("Fetching feed from: {}", feed))
}
