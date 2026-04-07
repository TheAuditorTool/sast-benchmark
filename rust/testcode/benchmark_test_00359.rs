pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let index: i64 = req.param("index").parse().unwrap_or(0);
    let result = get_item_at(index);
    super::shared::BenchmarkResponse::ok(&format!("item={}", result))
}

fn get_item_at(idx: i64) -> String { format!("item[{}]", idx) }
