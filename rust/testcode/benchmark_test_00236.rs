pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: u64 = req.param("value").parse().unwrap_or(0);
    let signed = val as isize;
    super::shared::BenchmarkResponse::ok(&format!("Signed: {}", signed))
}
