pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let count: i64 = req.param("count").parse().unwrap_or(0);
    let items = fetch_items(count);
    super::shared::BenchmarkResponse::ok(&format!("items={}", items))
}

fn fetch_items(n: i64) -> i64 { n }
