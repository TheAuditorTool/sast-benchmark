pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: u64 = req.param("value").parse().unwrap_or(0);
    let signed = val as i32;
    super::shared::BenchmarkResponse::ok(&format!("signed={}", signed))
}
