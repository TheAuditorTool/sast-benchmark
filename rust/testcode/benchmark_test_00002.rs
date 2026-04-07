pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let index_str = req.param("index");
    let index: usize = index_str.parse().unwrap_or(0);

    let items = vec!["apple", "banana", "cherry", "date", "elderberry"];

    let selected = items[index];

    super::shared::BenchmarkResponse::ok(&format!("Selected: {}", selected))
}
