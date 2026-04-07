pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let _resp = fetch_without_resolve_check(&url);

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn fetch_without_resolve_check(url: &str) -> String {
    format!("Response from {} (no post-resolve check)", url)
}
