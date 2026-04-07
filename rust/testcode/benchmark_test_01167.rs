pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_ids = req.param("ids");
    let ids: Vec<u64> = raw_ids
        .split(',')
        .filter_map(|s| s.trim().parse::<u64>().ok())
        .collect();
    if ids.is_empty() {
        return super::shared::BenchmarkResponse::bad_request("No valid ids");
    }
    let placeholders: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
    let sql = format!("SELECT * FROM orders WHERE id IN ({})", placeholders.join(","));
    super::shared::BenchmarkResponse::ok(&sql)
}
