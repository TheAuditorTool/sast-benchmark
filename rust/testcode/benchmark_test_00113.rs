pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.body_str();
    let max_size: usize = 1_048_576;
    let limited = if raw.len() > max_size { &raw[..max_size] } else { &raw };
    let value = format!("parsed(len={})", limited.len());
    super::shared::BenchmarkResponse::ok(&value)
}
