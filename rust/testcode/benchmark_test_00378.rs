pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let max_bytes: usize = 1_048_576;

    let _resp = fetch_with_size_limit(&url, max_bytes);

    super::shared::BenchmarkResponse::ok(&format!("Fetched (max {}B): {}", max_bytes, url))
}

fn fetch_with_size_limit(url: &str, max: usize) -> String {
    format!("Response from {} (limit {})", url, max)
}
